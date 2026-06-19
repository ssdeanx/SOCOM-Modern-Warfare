use bevy::prelude::*;
use avian3d::prelude::*;

use socom_core::components::{Player, Team};
use crate::physics::layers::CharacterController;

/// Formation offsets relative to player (index 0 = left, 1 = right, 2 = rear).
const FORMATION_OFFSETS: [Vec3; 3] = [
    Vec3::new(-1.5, 0.0, -1.0),
    Vec3::new(1.5, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -2.5),
];

/// Moves teammates to their formation positions relative to the player.
pub fn squad_formation_system(
    player_query: Query<&Transform, (With<Player>, Without<Team>)>,
    mut teammate_query: Query<(Entity, &mut CharacterController, &mut Transform), (With<Team>, Without<Player>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 { return; }
    let Ok(player_transform) = player_query.single() else { return; };
    let player_pos = player_transform.translation;
    let player_forward = *player_transform.forward();

    // Right vector for formation positioning
    let player_right = player_forward.cross(Vec3::Y).normalize();

    for (i, (_, mut controller, mut transform)) in teammate_query.iter_mut().enumerate() {
        if i >= FORMATION_OFFSETS.len() { break; }
        let offset = FORMATION_OFFSETS[i];
        // Transform offset from local to world space
        let world_offset = player_right * offset.x + Vec3::Y * offset.y + player_forward * offset.z;
        let target_pos = player_pos + world_offset;
        let to_target = target_pos - transform.translation;
        let dist = to_target.length();

        if dist > 0.5 {
            let dir = to_target / dist;
            let speed = 3.5;
            let accel = (8.0 * dt).min(1.0);
            let target_vel = dir * speed;
            controller.velocity.x += (target_vel.x - controller.velocity.x) * accel;
            controller.velocity.z += (target_vel.z - controller.velocity.z) * accel;
            controller.velocity.y = 0.0;
            transform.look_to(dir, Vec3::Y);
        } else {
            controller.velocity.x *= 0.9;
            controller.velocity.z *= 0.9;
        }
    }
}
