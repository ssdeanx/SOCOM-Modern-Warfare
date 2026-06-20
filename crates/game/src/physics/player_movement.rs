use avian3d::character_controller::move_and_slide::{
    MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse, MoveAndSlideOutput,
};
use avian3d::math::{AdjustPrecision, AsF32};
use avian3d::prelude::*;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, MovementState, Player};
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::damage::DamageMessage;
use crate::combat::weapon_bob::AdsState;
use crate::physics::layers::CharacterController;
use crate::stamina::{stamina_speed_mult, Stamina};
use crate::weapon_handling::WeaponHandling;

const WALK_SPEED: f32 = 3.0;
const SPRINT_SPEED: f32 = 5.0;
const CROUCH_SPEED: f32 = 1.5;
const PRONE_SPEED: f32 = 0.8;
const GROUND_FRICTION: f32 = 0.82;
const ACCELERATION: f32 = 12.0;
const GRAVITY: f32 = -19.6;
const MAX_FALL_SPEED: f32 = -30.0;
const JUMP_VELOCITY: f32 = 6.0;

/// Maximum distance from the player's feet to the ground for ground detection.
const GROUNDED_RAYCAST_DISTANCE: f32 = 0.15;

/// Fall distance in metres below which no damage is applied.
const SAFE_FALL_DISTANCE: f32 = 3.0;

/// Damage per metre fallen beyond `SAFE_FALL_DISTANCE`.
const FALL_DAMAGE_PER_METRE: f32 = 5.0;

fn stance_speed(stance: &MovementState) -> f32 {
    match stance {
        MovementState::Sprinting => SPRINT_SPEED,
        MovementState::Crouching => CROUCH_SPEED,
        MovementState::Prone | MovementState::InCover => PRONE_SPEED,
        MovementState::Standing => WALK_SPEED,
    }
}

fn camera_relative_direction(input: Vec2, camera_yaw: f32) -> Vec3 {
    if input == Vec2::ZERO {
        return Vec3::ZERO;
    }
    let (sin, cos) = camera_yaw.sin_cos();
    Vec3::new(
        input.x * cos - input.y * sin,
        0.0,
        input.x * sin + input.y * cos,
    )
    .normalize_or_zero()
}

/// Reads input and stance, computes velocity, applies MoveAndSlide.
/// Also handles jump input, ground detection, and fall damage.
pub fn player_movement_system(
    time: Res<Time>,
    mut move_and_slide: MoveAndSlide,
    spatial_query: SpatialQuery,
    _damage_writer: MessageWriter<DamageMessage>,
    mut query: Query<
        (
            Entity,
            &Collider,
            &mut CharacterController,
            &mut Transform,
            &MovementState,
            &ActionState<PlayerAction>,
            &mut Health,
            &Stamina,
            &WeaponHandling,
        ),
        With<Player>,
    >,
    camera_query: Query<&ThirdPersonCamera>,
    ads_state: Res<AdsState>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let camera_yaw = camera_query.iter().next().map(|cam| cam.yaw).unwrap_or(0.0);

    for (
        entity,
        collider,
        mut controller,
        mut transform,
        movement_state,
        action_state,
        mut health,
        stamina,
        handling,
    ) in query.iter_mut()
    {
        // ── 1. Ground detection ────────────────────────────────────────
        // Cast a short ray from the player's feet downward.
        let feet_pos = transform.translation;
        let ground_hit = spatial_query.cast_ray(
            feet_pos,
            Dir3::NEG_Y,
            GROUNDED_RAYCAST_DISTANCE,
            true,
            &SpatialQueryFilter::from_excluded_entities([entity]),
        );
        let was_on_ground = controller.on_ground;
        controller.on_ground = ground_hit.is_some();

        // ── 2. Fall damage ─────────────────────────────────────────────
        if was_on_ground && !controller.on_ground {
            // Just left the ground — record the height.
            controller.fall_start_y = feet_pos.y;
        } else if !was_on_ground && controller.on_ground {
            // Just landed — calculate fall damage.
            let fall_distance = controller.fall_start_y - feet_pos.y;
            if fall_distance > SAFE_FALL_DISTANCE {
                let damage = (fall_distance - SAFE_FALL_DISTANCE) * FALL_DAMAGE_PER_METRE;
                health.current = (health.current - damage).max(0.0);
            }
        } else if !controller.on_ground {
            // Still airborne — update fall_start_y if we're going higher
            // (so we measure from peak, not from launch point).
            if feet_pos.y > controller.fall_start_y {
                controller.fall_start_y = feet_pos.y;
            }
        }

        // ── 3. Jump ────────────────────────────────────────────────────
        if controller.on_ground && action_state.just_pressed(&PlayerAction::Jump) {
            controller.velocity.y = JUMP_VELOCITY;
            controller.on_ground = false; // will be re-checked next frame
        }

        // ── 4. Read movement input ─────────────────────────────────────
        let input = action_state.axis_pair(&PlayerAction::Move);
        if input == Vec2::ZERO {
            controller.velocity.x *= GROUND_FRICTION;
            controller.velocity.z *= GROUND_FRICTION;
            if controller.velocity.x.abs() < 0.01 {
                controller.velocity.x = 0.0;
            }
            if controller.velocity.z.abs() < 0.01 {
                controller.velocity.z = 0.0;
            }
        } else {
            let speed = stance_speed(movement_state)
                * ads_state.speed_mult
                * stamina_speed_mult(&stamina)
                * handling.current_weight_mult;
            let world_dir = camera_relative_direction(input, camera_yaw);
            let target_velocity = world_dir * speed;
            let accel_factor = (ACCELERATION * dt).min(1.0);
            controller.velocity.x = controller.velocity.x.lerp(target_velocity.x, accel_factor);
            controller.velocity.z = controller.velocity.z.lerp(target_velocity.z, accel_factor);
        }

        // ── 5. Apply gravity ───────────────────────────────────────────
        if !controller.on_ground {
            controller.velocity.y += GRAVITY * dt;
            if controller.velocity.y < MAX_FALL_SPEED {
                controller.velocity.y = MAX_FALL_SPEED;
            }
        } else if controller.velocity.y < 0.0 {
            // Snap to ground when on ground and falling.
            controller.velocity.y = 0.0;
        }

        // ── 6. Run MoveAndSlide ────────────────────────────────────────
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let config = MoveAndSlideConfig::default();
        let shape_position = transform.translation.adjust_precision();
        let shape_rotation = transform.rotation.adjust_precision();
        let vel = controller.velocity.adjust_precision();
        let MoveAndSlideOutput {
            position: new_position,
            projected_velocity: new_velocity,
        } = move_and_slide.move_and_slide(
            collider,
            shape_position,
            shape_rotation,
            vel,
            time.delta(),
            &config,
            &filter,
            |_hit| MoveAndSlideHitResponse::Accept,
        );
        transform.translation = new_position.f32();
        controller.velocity = new_velocity.f32();
    }
}
