// REALISTIC TACTICAL MOVEMENT SYSTEM
// SOCOM/SQUAD inspired: momentum, sprint brake, landing slow, stamina tiers,
// hold-to-sprint, stance transition blocking.

use avian3d::character_controller::move_and_slide::{
    MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse, MoveAndSlideOutput,
};
use avian3d::math::{AdjustPrecision, AsF32};
use avian3d::prelude::*;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, MovementState, Player};
use socom_core::resources::GameSettings;
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::damage::DamageMessage;
use crate::combat::weapon_bob::AdsState;
use crate::physics::layers::CharacterController;
use crate::physics::movement_modifiers::{
    max_direction_change_rate, rotate_towards, LandingSlow, Momentum, SprintBrake,
};
use crate::stamina::{stamina_effects, Stamina};
use crate::weapon_handling::WeaponHandling;

// ── Movement Speed Constants ──
const WALK_SPEED: f32 = 2.5;
const SPRINT_SPEED: f32 = 5.0;
const CROUCH_SPEED: f32 = 1.2;
const PRONE_SPEED: f32 = 0.4;
const GROUND_FRICTION: f32 = 0.85;
const ACCELERATION: f32 = 12.0;
const SPRINT_ACCELERATION: f32 = 3.5;
const SPRINT_SPEED_RAMP_TIME: f32 = 1.5;
const SPRINT_TIME_DECAY_RATE: f32 = 3.0;
const GRAVITY: f32 = -19.6;
const DEAD_ZONE: f32 = 0.15;
const MAX_FALL_SPEED: f32 = -30.0;
const JUMP_VELOCITY: f32 = 6.0;

// ── Landing Slow Constants ──
const LANDING_SLOW_DURATION: f32 = 0.5;
const LANDING_SLOW_MULT: f32 = 0.6;
const HARD_LANDING_THRESHOLD: f32 = 2.5;

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

/// Determine whether the player should be sprinting based on input and settings.
fn wants_to_sprint(
    action_state: &ActionState<PlayerAction>,
    settings: &GameSettings,
    stamina: &Stamina,
) -> bool {
    // Exhausted players cannot sprint until 20% recovery
    if stamina.is_exhausted() && stamina.ratio() < 0.2 {
        return false;
    }
    if settings.hold_to_sprint {
        action_state.pressed(&PlayerAction::Sprint)
    } else {
        action_state.just_pressed(&PlayerAction::Sprint)
    }
}

/// Reads input and stance, computes velocity, applies MoveAndSlide.
/// Also handles jump input, ground detection, and fall damage.
pub fn player_movement_system(
    time: Res<Time>,
    mut move_and_slide: MoveAndSlide,
    spatial_query: SpatialQuery,
    _damage_writer: MessageWriter<DamageMessage>,
    settings: Res<GameSettings>,
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
            &mut Momentum,
            &mut SprintBrake,
            &mut LandingSlow,
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
        mut momentum,
        mut sprint_brake,
        mut landing_slow,
    ) in query.iter_mut()
    {
        // ── 0. Ground detection ────────────────────────────────────────
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

        // ── 1. Landing slow detection ──────────────────────────────────
        if !was_on_ground && controller.on_ground {
            // Just landed
            let fall_distance = controller.fall_start_y - feet_pos.y;
            if fall_distance > HARD_LANDING_THRESHOLD {
                landing_slow.timer =
                    Timer::from_seconds(LANDING_SLOW_DURATION, TimerMode::Once);
                landing_slow.active = true;
            }
            // Fall damage
            if fall_distance > SAFE_FALL_DISTANCE {
                let damage = (fall_distance - SAFE_FALL_DISTANCE) * FALL_DAMAGE_PER_METRE;
                health.current = (health.current - damage).max(0.0);
            }
        } else if was_on_ground && !controller.on_ground {
            // Just left ground
            controller.fall_start_y = feet_pos.y;
        } else if !controller.on_ground {
            // Track peak height
            if feet_pos.y > controller.fall_start_y {
                controller.fall_start_y = feet_pos.y;
            }
        }

        // ── 2. Tick LandingSlow timer ─────────────────────────────────
        let landing_mult = if landing_slow.active {
            landing_slow.timer.tick(time.delta());
            if landing_slow.timer.just_finished() {
                landing_slow.active = false;
            }
            LANDING_SLOW_MULT
        } else {
            1.0
        };

        // ── 3. Tick SprintBrake timer ─────────────────────────────────
        sprint_brake.timer.tick(time.delta());
        // Brake active while timer has not fully elapsed
        let mut is_braking = sprint_brake.timer.fraction() < 1.0;

        // ── 4. Sprint brake activation check ──────────────────────────
        // If sprinting and pressing Aim, activate sprint brake
        if *movement_state == MovementState::Sprinting
            && action_state.just_pressed(&PlayerAction::Aim)
            && !is_braking
        {
            sprint_brake.timer = Timer::from_seconds(0.3, TimerMode::Once);
            is_braking = true;
        }

        // ── 5. Jump ───────────────────────────────────────────────────
        if controller.on_ground && action_state.just_pressed(&PlayerAction::Jump) {
            controller.velocity.y = JUMP_VELOCITY;
            controller.on_ground = false;
        }

        // ── 6. Read movement input ─────────────────────────────────────
        let mut input = action_state.axis_pair(&PlayerAction::Move);
        if input.length() < DEAD_ZONE {
            input = Vec2::ZERO;
        } else {
            input = input.normalize()
                * ((input.length() - DEAD_ZONE) / (1.0 - DEAD_ZONE)).clamp(0.0, 1.0);
        }

        // ── 7. Compute base speed with sprint ramp-up ──────────────────
        let wants_sprint = wants_to_sprint(&action_state, &settings, &stamina);
        let is_sprinting = *movement_state == MovementState::Sprinting && wants_sprint;

        let base_speed = if is_sprinting {
            let sprint_elapsed = momentum.sprint_time + dt;
            let ramp_factor = (sprint_elapsed / SPRINT_SPEED_RAMP_TIME).min(1.0);
            let smooth_ramp = ramp_factor * ramp_factor; // Quadratic ease-in
            stance_speed(movement_state) * smooth_ramp
        } else {
            stance_speed(movement_state)
        };

        // Apply all multipliers
        let stamina_eff = stamina_effects(&stamina, handling.current_weight_mult);

        // ADS speed mult: 1.0 during brake (no ADS movement penalty yet),
        // otherwise from AdsState (0.4 at full ADS)
        let ads_speed_mult = if is_braking {
            1.0
        } else {
            ads_state.speed_mult
        };

        let final_speed = base_speed
            * ads_speed_mult
            * stamina_eff.speed_mult
            * handling.current_weight_mult
            * landing_mult;

        // ── 8. Compute velocity ───────────────────────────────────────
        if is_sprinting && input != Vec2::ZERO {
            // Sprint: momentum-limited direction change
            let world_dir = camera_relative_direction(input, camera_yaw);
            let current_xz = Vec3::new(momentum.current_direction.x, 0.0, momentum.current_direction.z);
            let target_xz = Vec3::new(world_dir.x, 0.0, world_dir.z);

            let max_angle_rad =
                max_direction_change_rate(movement_state, 1.0).to_radians() * dt;

            let new_dir = if current_xz.length_squared() > 0.01 && target_xz.length_squared() > 0.01
            {
                rotate_towards(current_xz, target_xz, max_angle_rad)
            } else {
                target_xz.normalize_or_zero()
            };

            let target_velocity = new_dir * final_speed;
            let accel_factor = 1.0 - (-SPRINT_ACCELERATION * dt).exp();
            controller.velocity.x = controller.velocity.x.lerp(target_velocity.x, accel_factor);
            controller.velocity.z = controller.velocity.z.lerp(target_velocity.z, accel_factor);

            // Update stored momentum direction
            momentum.current_direction = new_dir;
        } else if input == Vec2::ZERO {
            // No input: friction deceleration
            controller.velocity.x *= GROUND_FRICTION;
            controller.velocity.z *= GROUND_FRICTION;
            if controller.velocity.x.abs() < 0.01 {
                controller.velocity.x = 0.0;
            }
            if controller.velocity.z.abs() < 0.01 {
                controller.velocity.z = 0.0;
            }
            momentum.current_direction = Vec3::ZERO;
        } else {
            // Non-sprint: direct lerp (full responsiveness)
            let world_dir = camera_relative_direction(input, camera_yaw);
            let target_velocity = world_dir * final_speed;
            let accel_factor = 1.0 - (-ACCELERATION * dt).exp();
            controller.velocity.x = controller.velocity.x.lerp(target_velocity.x, accel_factor);
            controller.velocity.z = controller.velocity.z.lerp(target_velocity.z, accel_factor);
            momentum.current_direction = world_dir;
        }

        // ── 9. Update sprint_time ──────────────────────────────────────
        if is_sprinting {
            momentum.sprint_time = (momentum.sprint_time + dt)
                .min(SPRINT_SPEED_RAMP_TIME + 1.0);
        } else if momentum.sprint_time > 0.0 {
            momentum.sprint_time =
                (momentum.sprint_time - SPRINT_TIME_DECAY_RATE * dt).max(0.0);
        }

        // Update max_turn_rate based on current stance
        momentum.max_turn_rate = max_direction_change_rate(movement_state, 1.0);
        momentum.is_sprinting_up = is_sprinting
            && momentum.sprint_time < SPRINT_SPEED_RAMP_TIME;

        // ── 10. Apply gravity ──────────────────────────────────────────
        if !controller.on_ground {
            controller.velocity.y += GRAVITY * dt;
            if controller.velocity.y < MAX_FALL_SPEED {
                controller.velocity.y = MAX_FALL_SPEED;
            }
        } else if controller.velocity.y < 0.0 {
            controller.velocity.y = 0.0;
        }

        // ── 11. Run MoveAndSlide ───────────────────────────────────────
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
