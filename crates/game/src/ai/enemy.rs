use avian3d::prelude::*;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

use socom_core::components::{Health, MovementState, Player, Team, Weapon};

use crate::combat::{DamageMessage, WeaponState};
use crate::physics::CharacterController;

use super::{AiState, PatrolRoute, VisionCone};

// ── Constants ────────────────────────────────────────────────────────────────

/// Patrol movement speed (m/s).
const PATROL_SPEED: f32 = 1.5;

/// Gravity acceleration (m/s²).
const GRAVITY: f32 = -19.6;

/// Acceleration toward target velocity (effective lerp per second).
const ACCELERATION: f32 = 8.0;

/// Rate at which suspicion decays per second when the player is not visible.
const SUSPICION_DECAY_RATE: f32 = 10.0;

/// Suspicion threshold to transition from Patrol → Engage.
const SUSPICION_THRESHOLD: f32 = 100.0;

/// Minimum time between enemy shots during engagement (seconds).
const ENGAGE_FIRE_INTERVAL: f32 = 1.5;

/// Damage dealt per enemy bullet.
const ENEMY_DAMAGE: f32 = 10.0;

// ── Enemy bundle ─────────────────────────────────────────────────────────────

/// Complete bundle for spawning an enemy AI entity.
#[derive(Bundle)]
pub struct EnemyBundle {
    pub team: Team,
    pub health: Health,
    pub movement_state: MovementState,
    pub ai_state: AiState,
    pub vision: VisionCone,
    pub patrol: PatrolRoute,
    pub rigid_body: RigidBody,
    pub character_controller: CharacterController,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub weapon_state: WeaponState,
}

impl EnemyBundle {
    pub fn new(position: Vec3, patrol_a: Vec3, patrol_b: Vec3) -> Self {
        let weapon = Weapon::ak47();
        Self {
            team: Team::Enemy,
            health: Health::new(80.0),
            movement_state: MovementState::Standing,
            ai_state: AiState::Patrol,
            vision: VisionCone::default(),
            patrol: PatrolRoute::between(patrol_a, patrol_b),
            character_controller: CharacterController::default(),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::capsule(0.3, 0.9),
            collision_layers: CollisionLayers::default(),
            transform: Transform::from_translation(position),
            global_transform: default(),
            weapon_state: WeaponState::from_weapon(&weapon, 0),
        }
    }
}

// ── Patrol system ────────────────────────────────────────────────────────────

/// Moves the enemy along its patrol route when in `Patrol` state.
pub fn patrol_system(
    time: Res<Time>,
    mut enemy_query: Query<(
        &mut CharacterController,
        &mut Transform,
        &mut PatrolRoute,
        &AiState,
    )>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (mut controller, mut transform, mut patrol, &ai_state) in enemy_query.iter_mut() {
        if ai_state != AiState::Patrol {
            continue;
        }

        if patrol.waypoints.is_empty() {
            controller.velocity.x = 0.0;
            controller.velocity.z = 0.0;
            continue;
        }

        if patrol.is_waiting {
            patrol.wait_timer.tick(time.delta());
            if patrol.wait_timer.just_finished() {
                patrol.is_waiting = false;
                patrol.current_index = (patrol.current_index + 1) % patrol.waypoints.len();
            } else {
                // Decelerate while waiting.
                controller.velocity.x *= 0.9;
                controller.velocity.z *= 0.9;
                continue;
            }
        }

        let target = patrol.waypoints[patrol.current_index];
        let to_target = target - transform.translation;
        let distance = to_target.length();

        if distance < 0.5 {
            // Arrived at waypoint.
            patrol.is_waiting = true;
            patrol.wait_timer.reset();
            controller.velocity.x = 0.0;
            controller.velocity.z = 0.0;
        } else {
            let dir = to_target / distance;
            let target_vel = dir * PATROL_SPEED;
            let accel = (ACCELERATION * dt).min(1.0);
            controller.velocity.x = controller.velocity.x.lerp(target_vel.x, accel);
            controller.velocity.z = controller.velocity.z.lerp(target_vel.z, accel);
            if dir.length_squared() > 0.01 {
                transform.look_to(dir, Vec3::Y);
            }
        }

        controller.velocity.y += GRAVITY * dt;
    }
}

// ── Detection system ─────────────────────────────────────────────────────────

/// Checks vision cone + line-of-sight against the player. Fills suspicion
/// meter; transitions to `Engage` when suspicion is saturated.
pub fn detection_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut enemy_query: Query<(&Transform, &mut VisionCone, &mut AiState, &PatrolRoute)>,
    player_query: Query<&Transform, (With<Player>, Without<Team>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (enemy_transform, mut vision, mut ai_state, _patrol) in enemy_query.iter_mut() {
        // Only detect if in Patrol or Alert; already-engaged enemies skip.
        if *ai_state == AiState::Engage {
            continue;
        }

        let enemy_pos = enemy_transform.translation;
        let to_player = (player_pos - enemy_pos).normalize_or_zero();
        let distance = enemy_pos.distance(player_pos);

        // Range check.
        if distance > vision.range {
            decay_suspicion(&mut vision, dt);
            continue;
        }

        // Cone-of-vision check (horizontal only for simplicity; vertical
        // is already bounded by range + eye-height difference).
        let forward = *enemy_transform.forward();
        let angle_to_player = forward.angle_between(to_player);
        let half_fov_h = vision.fov_h * 0.5;

        if angle_to_player > half_fov_h {
            decay_suspicion(&mut vision, dt);
            continue;
        }

        // Line-of-sight: cast ray from enemy head toward player.
        // If something (a wall) blocks the ray before reaching the player,
        // LOS is blocked.
        let filter = SpatialQueryFilter::default();
        let los_clear = match spatial_query.cast_ray(
            enemy_pos + Vec3::Y * 1.0,
            Dir3::new(to_player).unwrap_or(Dir3::Z),
            distance,
            true,
            &filter,
        ) {
            None => true,
            Some(hit) => player_query.contains(hit.entity),
        };

        if !los_clear {
            decay_suspicion(&mut vision, dt);
            continue;
        }

        // Player is visible: build suspicion.
        vision.suspicion = (vision.suspicion + 30.0 * dt).min(SUSPICION_THRESHOLD);
        if vision.suspicion >= SUSPICION_THRESHOLD {
            *ai_state = AiState::Engage;
        }
    }
}

/// Decays suspicion toward zero.
fn decay_suspicion(vision: &mut VisionCone, dt: f32) {
    vision.suspicion = (vision.suspicion - SUSPICION_DECAY_RATE * dt).max(0.0);
}

// ── Engage system ────────────────────────────────────────────────────────────

/// Enemies in `Engage` state face the player, stop moving, and fire at
/// regular intervals. If LOS is lost they fall back to `Alert`.
pub fn engage_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut damage_writer: MessageWriter<DamageMessage>,
    mut enemy_query: Query<(
        Entity,
        &mut Transform,
        &mut AiState,
        &mut CharacterController,
        &mut WeaponState,
        &Team,
    )>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Team>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (
        _enemy_entity,
        mut enemy_transform,
        mut ai_state,
        mut controller,
        mut weapon_state,
        _team,
    ) in enemy_query.iter_mut()
    {
        if *ai_state != AiState::Engage {
            continue;
        }

        let enemy_pos = enemy_transform.translation;
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();
        let dir_to_player = to_player / distance;

        // Face the player. Uses &mut Transform so look_to is callable.
        // Bevy queries for enemies (With<Team>) and player (With<Player>, Without<Team>)
        // are disjoint, so there is no borrow conflict.
        if distance > 0.5 {
            enemy_transform.look_to(dir_to_player, Vec3::Y);
        }

        // Decelerate to a stop while engaging.
        let accel = (ACCELERATION * dt).min(1.0);
        controller.velocity.x = controller.velocity.x.lerp(0.0, accel);
        controller.velocity.z = controller.velocity.z.lerp(0.0, accel);
        controller.velocity.y += GRAVITY * dt;

        // LOS check: if world geometry blocks view, go to Alert.
        let filter = SpatialQueryFilter::default();
        let los_blocked = match spatial_query.cast_ray(
            enemy_pos + Vec3::Y * 1.0,
            Dir3::new(dir_to_player).unwrap_or(Dir3::Z),
            distance,
            true,
            &filter,
        ) {
            Some(hit) if hit.distance < distance - 0.5 => true,
            _ => false,
        };

        if los_blocked {
            *ai_state = AiState::Alert;
            continue;
        }

        // Fire at the player at intervals.
        if time.elapsed_secs() - weapon_state.last_fire_time >= ENGAGE_FIRE_INTERVAL
            && !weapon_state.is_reloading
        {
            if weapon_state.magazine > 0 {
                weapon_state.last_fire_time = time.elapsed_secs();
                weapon_state.magazine -= 1;

                damage_writer.write(DamageMessage {
                    target: player_entity,
                    amount: ENEMY_DAMAGE,
                    source: _enemy_entity,
                    hit_point: player_pos,
                    hit_normal: dir_to_player,
                });

                // Enemy weapon fire sound
                use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
                let enemy_shot: Handle<AudioSource> = asset_server.load("audio/weapon_m4.ogg");
                commands.spawn((
                    AudioPlayer(enemy_shot),
                    PlaybackSettings::ONCE.with_volume(Volume::Linear(0.3)),
                ));
            } else if weapon_state.reserve > 0 {
                weapon_state.is_reloading = true;
                weapon_state.reload_timer = 2.5;
            }
        }

        // Reload tick.
        if weapon_state.is_reloading {
            weapon_state.reload_timer -= dt;
            if weapon_state.reload_timer <= 0.0 {
                weapon_state.is_reloading = false;
                let weapon = Weapon::ak47();
                let needed = weapon.magazine_size - weapon_state.magazine;
                let available = weapon_state.reserve.min(needed);
                weapon_state.magazine += available;
                weapon_state.reserve -= available;
            }
        }
    }
}

// ── Death / despawn ──────────────────────────────────────────────────────────

/// Despawns enemy entities whose health has reached zero.
pub fn enemy_death_system(mut commands: Commands, query: Query<(Entity, &Health, &Team)>) {
    for (entity, health, team) in query.iter() {
        if matches!(team, Team::Enemy) && !health.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}
