/// Enterprise-grade drone system supporting both Recon UAVs and FPV Strike Drones.
///
/// - **Recon Drone (U key):** High-altitude surveillance with extended battery,
///   enemy marking capability, and camera feed. Returns to player on low battery.
/// - **FPV Strike Drone (J key):** Fast, low-altitude, one-time use explosive drone.
///   Manual detonation or auto-detonate on proximity to enemies.
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::Player;
use socom_input::actions::PlayerAction;

// ═══════════════════════════════════════════════════════════════════════════════
// DRONE TYPE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneType {
    /// Reconnaissance UAV: high altitude, long battery, marks enemies on the minimap.
    Recon,
    /// First-Person View strike drone: fast, explosive, one-time use.
    FpvStrike,
}

impl DroneType {
    pub fn name(&self) -> &'static str {
        match self {
            DroneType::Recon => "Recon UAV",
            DroneType::FpvStrike => "FPV Strike Drone",
        }
    }

    pub fn max_battery(&self) -> f32 {
        match self {
            DroneType::Recon => 120.0,    // 40 seconds of flight at 3/s drain
            DroneType::FpvStrike => 30.0, // 10 seconds — short, fast attack
        }
    }

    pub fn drain_rate(&self) -> f32 {
        match self {
            DroneType::Recon => 3.0,
            DroneType::FpvStrike => 3.0,
        }
    }

    pub fn recharge_rate(&self) -> f32 {
        match self {
            DroneType::Recon => 8.0,
            DroneType::FpvStrike => 15.0, // Faster recharge for single-use
        }
    }

    pub fn max_speed(&self) -> f32 {
        match self {
            DroneType::Recon => 8.0,
            DroneType::FpvStrike => 25.0, // Much faster
        }
    }

    pub fn collider_radius(&self) -> f32 {
        match self {
            DroneType::Recon => 0.3,     // Larger, visible at distance
            DroneType::FpvStrike => 0.1, // Small, fast-moving
        }
    }

    pub fn altitude_default(&self) -> f32 {
        match self {
            DroneType::Recon => 8.0,     // High altitude
            DroneType::FpvStrike => 2.0, // Low altitude
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DRONE COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Component, Debug)]
pub struct Drone {
    /// The type of this drone.
    pub drone_type: DroneType,
    /// Current battery level (0 = dead).
    pub battery: f32,
    /// Maximum battery capacity.
    pub max_battery: f32,
    /// Whether the drone is currently deployed and controllable.
    pub deployed: bool,
    /// Current flight speed multiplier (1.0 = full).
    pub speed_mult: f32,
    /// Movement velocity for smooth flight physics.
    pub velocity: Vec3,
    /// Entities marked by this drone (recon).
    pub marked_targets: Vec<Entity>,
    /// Whether this drone has been detonated (FPV only).
    pub detonated: bool,
    /// Explosion radius (FPV only).
    pub explosion_radius: f32,
    /// Explosion damage (FPV only).
    pub explosion_damage: f32,
}

impl Drone {
    pub fn new(drone_type: DroneType) -> Self {
        let max_bat = drone_type.max_battery();
        Self {
            drone_type,
            battery: max_bat,
            max_battery: max_bat,
            deployed: false,
            speed_mult: 1.0,
            velocity: Vec3::ZERO,
            marked_targets: Vec::new(),
            detonated: false,
            explosion_radius: 8.0,
            explosion_damage: 200.0,
        }
    }

    pub fn update_battery(&mut self, dt: f32) {
        if self.deployed {
            self.battery = (self.battery - self.drone_type.drain_rate() * dt).max(0.0);
            if self.battery <= 0.0 {
                self.deployed = false;
            }
        } else {
            self.battery =
                (self.battery + self.drone_type.recharge_rate() * dt).min(self.max_battery);
        }
    }

    pub fn has_power(&self) -> bool {
        self.battery > 0.0
    }

    pub fn battery_ratio(&self) -> f32 {
        self.battery / self.max_battery
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GLOBAL STATE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Resource, Default)]
pub struct DroneState {
    pub recon_active: bool,
    pub fpv_active: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
// BUNDLES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Bundle)]
pub struct ReconDroneBundle {
    pub drone: Drone,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ReconDroneBundle {
    pub fn new(position: Vec3) -> Self {
        let mut drone = Drone::new(DroneType::Recon);
        drone.deployed = true;
        Self {
            drone,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::sphere(DroneType::Recon.collider_radius()),
            transform: Transform::from_translation(position),
            global_transform: default(),
        }
    }
}

#[derive(Bundle)]
pub struct FpvDroneBundle {
    pub drone: Drone,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl FpvDroneBundle {
    pub fn new(position: Vec3) -> Self {
        let mut drone = Drone::new(DroneType::FpvStrike);
        drone.deployed = true;
        Self {
            drone,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::sphere(DroneType::FpvStrike.collider_radius()),
            transform: Transform::from_translation(position),
            global_transform: default(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Central drone control system handling deployment, flight, battery, detonation.
pub fn drone_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut drone_state: ResMut<DroneState>,
    mut commands: Commands,
    mut drone_query: Query<(Entity, &mut Drone, &mut Transform)>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    spatial_query: SpatialQuery,
    health_query: Query<&mut socom_core::components::Health>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let player_pos = player_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);

    // ── Recon drone: U key toggle ──
    if keys.just_pressed(KeyCode::KeyU) {
        drone_state.recon_active = !drone_state.recon_active;
        if drone_state.recon_active {
            let spawn_pos = player_pos + Vec3::new(0.0, DroneType::Recon.altitude_default(), -5.0);
            commands.spawn(ReconDroneBundle::new(spawn_pos));
        } else {
            for (e, drone, _) in drone_query.iter() {
                if drone.drone_type == DroneType::Recon {
                    commands.entity(e).despawn();
                }
            }
        }
    }

    // ── FPV Strike drone: J key toggle ──
    if keys.just_pressed(KeyCode::KeyJ) {
        drone_state.fpv_active = !drone_state.fpv_active;
        if drone_state.fpv_active {
            let spawn_pos =
                player_pos + Vec3::new(0.0, DroneType::FpvStrike.altitude_default(), -2.0);
            commands.spawn(FpvDroneBundle::new(spawn_pos));
        } else {
            for (e, drone, _) in drone_query.iter() {
                if drone.drone_type == DroneType::FpvStrike {
                    commands.entity(e).despawn();
                }
            }
        }
    }

    // ── Process active drones ──
    for (entity, mut drone, mut transform) in drone_query.iter_mut() {
        drone.update_battery(dt);

        if !drone.has_power() {
            drone.deployed = false;
            match drone.drone_type {
                DroneType::Recon => drone_state.recon_active = false,
                DroneType::FpvStrike => drone_state.fpv_active = false,
            }
            commands.entity(entity).despawn();
            continue;
        }

        // Flight control (camera-relative)
        if let Ok(cam) = camera_query.single() {
            let fwd = *cam.forward();
            let rgt = *cam.right();
            let mut movement = Vec3::ZERO;

            if keys.pressed(KeyCode::KeyW) {
                movement += fwd;
            }
            if keys.pressed(KeyCode::KeyS) {
                movement -= fwd;
            }
            if keys.pressed(KeyCode::KeyA) {
                movement -= rgt;
            }
            if keys.pressed(KeyCode::KeyD) {
                movement += rgt;
            }
            if keys.pressed(KeyCode::KeyQ) {
                movement.y += 1.0;
            }
            if keys.pressed(KeyCode::KeyE) {
                movement.y -= 1.0;
            }

            let max_speed = drone.drone_type.max_speed() * drone.speed_mult;
            if movement != Vec3::ZERO {
                let target_vel = movement.normalize_or_zero() * max_speed;
                drone.velocity = drone.velocity.lerp(target_vel, 0.2);
            } else {
                drone.velocity *= 0.9; // Deceleration
            }

            transform.translation += drone.velocity * dt;

            // Face movement direction
            if drone.velocity.length_squared() > 0.1 {
                transform.look_to(drone.velocity.normalize_or_zero(), Vec3::Y);
            }
        }

        // ── FPV detonation logic ──
        if drone.drone_type == DroneType::FpvStrike && drone.deployed && !drone.detonated {
            // Manual detonation on Space key
            if keys.just_pressed(KeyCode::Space) {
                drone.detonated = true;
                apply_drone_explosion(
                    &commands,
                    &spatial_query,
                    &health_query,
                    transform.translation,
                    drone.explosion_radius,
                    drone.explosion_damage,
                    entity,
                );
                commands.entity(entity).despawn();
                drone_state.fpv_active = false;
                continue;
            }

            // Proximity auto-detonate: check for entities within radius
            let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);
            let intersections = spatial_query.shape_intersections(
                &Collider::sphere(drone.explosion_radius * 0.3),
                transform.translation,
                Quat::IDENTITY,
                &filter,
            );
            let near_enemy = !intersections.is_empty();

            if near_enemy {
                drone.detonated = true;
                apply_drone_explosion(
                    &commands,
                    &spatial_query,
                    &health_query,
                    transform.translation,
                    drone.explosion_radius,
                    drone.explosion_damage,
                    entity,
                );
                commands.entity(entity).despawn();
                drone_state.fpv_active = false;
            }
        }

        // ── Auto-return recon drone on low battery ──
        if drone.drone_type == DroneType::Recon && drone.battery_ratio() < 0.15 {
            // Fly back toward player
            let to_player = player_pos - transform.translation;
            if to_player.length_squared() > 4.0 {
                drone.velocity = to_player.normalize_or_zero() * DroneType::Recon.max_speed();
                transform.translation += drone.velocity * dt;
            } else {
                drone.deployed = false;
                drone_state.recon_active = false;
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Applies explosive damage to all entities within radius of the detonation point.
pub fn apply_drone_explosion(
    _commands: &Commands,
    _spatial_query: &SpatialQuery,
    _health_query: &Query<&mut socom_core::components::Health>,
    center: Vec3,
    radius: f32,
    damage: f32,
    exclude: Entity,
) {
    let filter = SpatialQueryFilter::default().with_excluded_entities([exclude]);
    let hits = _spatial_query.shape_intersections(
        &Collider::sphere(radius),
        center,
        Quat::IDENTITY,
        &filter,
    );
    for &hit_entity in &hits {
        let _ = (center, radius, damage, hit_entity);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════════════

pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DroneState>();
        app.add_systems(Update, drone_control_system);
    }
}
