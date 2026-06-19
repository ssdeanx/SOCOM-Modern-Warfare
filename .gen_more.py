import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
base = "crates/game/src"

with open(os.path.join(base, "drones", "mod.rs"), "w", encoding="utf-8") as f:
    f.write("""use avian3d::prelude::*;
use bevy::prelude::*;
use socom_core::components::Player;

#[derive(Component, Debug)]
pub struct Drone {
    pub battery: f32,
    pub max_battery: f32,
    pub deployed: bool,
    pub speed_mult: f32,
}
impl Default for Drone {
    fn default() -> Self { Self { battery: 100.0, max_battery: 100.0, deployed: false, speed_mult: 1.0 } }
}
impl Drone {
    pub fn update_battery(&mut self, dt: f32) {
        if self.deployed { self.battery = (self.battery - 3.0 * dt).max(0.0); if self.battery <= 0.0 { self.deployed = false; } }
        else { self.battery = (self.battery + 10.0 * dt).min(self.max_battery); }
    }
    pub fn has_power(&self) -> bool { self.battery > 0.0 }
}

#[derive(Resource, Default)]
pub struct DroneState { pub active: bool }

#[derive(Bundle)]
pub struct DroneBundle {
    pub drone: Drone,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
impl DroneBundle {
    pub fn new(position: Vec3) -> Self {
        Self { drone: Drone::default(), rigid_body: RigidBody::Kinematic, collider: Collider::sphere(0.2), transform: Transform::from_translation(position), global_transform: default() }
    }
}

pub fn drone_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut drone_state: ResMut<DroneState>,
    mut commands: Commands,
    mut drone_query: Query<(Entity, &mut Drone, &mut Transform)>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 { return; }
    if keys.just_pressed(KeyCode::KeyU) {
        drone_state.active = !drone_state.active;
        if drone_state.active {
            if let Ok(p) = player_query.single() { commands.spawn(DroneBundle::new(p.translation + Vec3::new(0.0, 2.0, -3.0))); }
        } else {
            for (e, _, _) in drone_query.iter() { commands.entity(e).despawn(); }
        }
    }
    if drone_state.active {
        for (_, mut drone, mut t) in drone_query.iter_mut() {
            drone.update_battery(dt);
            if !drone.has_power() { drone.deployed = false; drone_state.active = false; continue; }
            if let Ok(cam) = camera_query.single() {
                let fwd = *cam.forward(); let rgt = *cam.right();
                let mut mv = Vec3::ZERO;
                if keys.pressed(KeyCode::KeyW) { mv += fwd; }
                if keys.pressed(KeyCode::KeyS) { mv -= fwd; }
                if keys.pressed(KeyCode::KeyA) { mv -= rgt; }
                if keys.pressed(KeyCode::KeyD) { mv += rgt; }
                if keys.pressed(KeyCode::KeyQ) { mv.y += 1.0; }
                if keys.pressed(KeyCode::KeyE) { mv.y -= 1.0; }
                let speed = 8.0 * drone.speed_mult;
                if mv != Vec3::ZERO { t.translation += mv.normalize_or_zero() * speed * dt; }
            }
        }
    }
}

pub struct DronePlugin;
impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DroneState>();
        app.add_systems(Update, drone_control_system);
    }
}
""")
print("drones created")

with open(os.path.join(base, "ammo_type", "mod.rs"), "w", encoding="utf-8") as f:
    f.write("""use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AmmoType {
    Fmj, HollowPoint, ArmourPiercing, Tracer,
}
impl AmmoType {
    pub fn name(&self) -> &'static str {
        match self { AmmoType::Fmj => "FMJ", AmmoType::HollowPoint => "HP", AmmoType::ArmourPiercing => "AP", AmmoType::Tracer => "TRACER" }
    }
    pub fn damage_mult(&self) -> f32 { match self { AmmoType::Fmj => 1.0, AmmoType::HollowPoint => 1.25, AmmoType::ArmourPiercing => 0.85, AmmoType::Tracer => 0.95 } }
    pub fn penetration_mult(&self) -> f32 { match self { AmmoType::Fmj => 1.0, AmmoType::HollowPoint => 0.5, AmmoType::ArmourPiercing => 1.8, AmmoType::Tracer => 1.0 } }
    pub fn spread_mult(&self) -> f32 { match self { AmmoType::Fmj => 1.0, AmmoType::HollowPoint => 1.15, AmmoType::ArmourPiercing => 1.05, AmmoType::Tracer => 1.0 } }
}
impl Default for AmmoType { fn default() -> Self { AmmoType::Fmj } }

#[derive(Component, Debug, Clone)]
pub struct LoadedAmmo { pub ammo_type: AmmoType, pub count: u32 }
""")
print("ammo_type created")

print("All base modules created")
