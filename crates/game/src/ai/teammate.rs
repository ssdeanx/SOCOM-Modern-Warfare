use super::AiSystems;
use crate::combat::{DamageMessage, WeaponState};
use crate::physics::CharacterController;
use crate::squad::orders::{ActiveOrders, SquadOrder};
use avian3d::prelude::*;
use bevy::prelude::*;
use socom_core::components::{Health, MovementState, Player, Team, Weapon};
use socom_core::resources::is_not_paused;

const FOLLOW_DISTANCE: f32 = 2.5;
const FOLLOW_SPEED: f32 = 3.5;
const ACCELERATION: f32 = 8.0;
const GRAVITY: f32 = -19.6;
const FOLLOW_STOP_DISTANCE: f32 = 2.0;
const ENGAGE_RANGE: f32 = 25.0;
const TEAMMATE_FIRE_INTERVAL: f32 = 1.0;
const TEAMMATE_DAMAGE: f32 = 20.0;

#[derive(Bundle)]
pub struct TeammateBundle {
    pub team: Team,
    pub health: Health,
    pub movement_state: MovementState,
    pub character_controller: CharacterController,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub weapon_state: WeaponState,
}

impl TeammateBundle {
    pub fn new(position: Vec3) -> Self {
        let weapon = Weapon::m4a1();
        Self {
            team: Team::Teammate,
            health: Health::new(100.0),
            movement_state: MovementState::Standing,
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

pub struct TeammatePlugin;

impl Plugin for TeammatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                teammate_follow_system,
                teammate_combat_system,
                teammate_reload_system,
            )
                .in_set(AiSystems)
                .run_if(is_not_paused),
        );
    }
}

pub fn teammate_follow_system(
    time: Res<Time>,
    mut teammate_query: Query<
        (Entity, &mut CharacterController, &mut Transform, &Health),
        (With<Team>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Team>)>,
    active_orders: Res<ActiveOrders>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;
    let player_back = -*player_transform.forward();
    for (entity, mut controller, mut transform, health) in teammate_query.iter_mut() {
        if !health.is_alive() {
            continue;
        }
        // Check for active order. If HOLD, stay in place.
        if let Some(order) = active_orders.orders.get(&entity) {
            match order {
                SquadOrder::HoldPosition => {
                    controller.velocity.x *= 0.9;
                    controller.velocity.z *= 0.9;
                    continue;
                }
                SquadOrder::RegroupOnPlayer => {
                    // Fall through to follow logic below
                }
                _ => {}
            }
        }
        let target_pos = player_pos + player_back * FOLLOW_DISTANCE;
        let to_target = target_pos - transform.translation;
        let distance = to_target.length();
        if distance > FOLLOW_STOP_DISTANCE {
            let dir = to_target / distance;
            let target_vel = dir * FOLLOW_SPEED;
            let accel = (ACCELERATION * dt).min(1.0);
            controller.velocity.x = controller.velocity.x.lerp(target_vel.x, accel);
            controller.velocity.z = controller.velocity.z.lerp(target_vel.z, accel);
            if dir.length_squared() > 0.01 {
                transform.look_to(dir, Vec3::Y);
            }
        } else {
            controller.velocity.x *= 0.9;
            controller.velocity.z *= 0.9;
        }
        controller.velocity.y += GRAVITY * dt;
    }
}

pub fn teammate_combat_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut damage_writer: bevy::ecs::message::MessageWriter<DamageMessage>,
    mut teammate_query: Query<
        (Entity, &Transform, &mut WeaponState, &Health),
        (With<Team>, Without<Player>),
    >,
    enemy_query: Query<(Entity, &Transform, &Health), (With<Team>, Without<Player>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (teammate_entity, teammate_transform, mut weapon_state, health) in teammate_query.iter_mut()
    {
        if !health.is_alive() || weapon_state.is_reloading {
            continue;
        }
        let mut closest_enemy: Option<(Entity, f32)> = None;
        let filter = SpatialQueryFilter::default();
        for (enemy_entity, enemy_transform, enemy_health) in enemy_query.iter() {
            if !enemy_health.is_alive() {
                continue;
            }
            let to_enemy = enemy_transform.translation - teammate_transform.translation;
            let distance = to_enemy.length();
            if distance > ENGAGE_RANGE {
                continue;
            }
            let dir = Dir3::new(to_enemy / distance).unwrap_or(Dir3::Z);
            let los_clear = match spatial_query.cast_ray(
                teammate_transform.translation + Vec3::Y * 1.0,
                dir,
                distance,
                true,
                &filter,
            ) {
                None => true,
                Some(hit) => hit.entity == enemy_entity,
            };
            if los_clear {
                match closest_enemy {
                    Some((_, closest_dist)) if distance < closest_dist => {
                        closest_enemy = Some((enemy_entity, distance));
                    }
                    None => {
                        closest_enemy = Some((enemy_entity, distance));
                    }
                    _ => {}
                }
            }
        }
        if let Some((enemy_entity, _)) = closest_enemy {
            if time.elapsed_secs() - weapon_state.last_fire_time >= TEAMMATE_FIRE_INTERVAL {
                weapon_state.last_fire_time = time.elapsed_secs();
                weapon_state.magazine = weapon_state.magazine.saturating_sub(1);
                damage_writer.write(DamageMessage {
                    target: enemy_entity,
                    amount: TEAMMATE_DAMAGE,
                    source: teammate_entity,
                    hit_point: Vec3::ZERO,
                    hit_normal: Vec3::ZERO,
                });
            }
        }
    }
}

pub fn teammate_reload_system(
    time: Res<Time>,
    mut teammate_query: Query<&mut WeaponState, (With<Team>, Without<Player>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for mut weapon_state in teammate_query.iter_mut() {
        if weapon_state.is_reloading {
            weapon_state.reload_timer -= dt;
            if weapon_state.reload_timer <= 0.0 {
                weapon_state.is_reloading = false;
                let needed = 30 - weapon_state.magazine;
                let available = weapon_state.reserve.min(needed);
                weapon_state.magazine += available;
                weapon_state.reserve -= available;
            }
        } else if weapon_state.magazine == 0 && weapon_state.reserve > 0 {
            weapon_state.is_reloading = true;
            weapon_state.reload_timer = 2.1;
        }
    }
}
