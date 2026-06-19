use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::weapon_model::WeaponModelRoot;
use crate::stamina::{stamina_sway_mult, Stamina};
/// Shared ADS state updated each frame by the bob system.
/// Read by shooting and movement systems to apply modifiers.
#[derive(Resource, Default)]
pub struct AdsState {
    /// 0.0 = hip fire, 1.0 = fully aimed.
    pub factor: f32,
    /// Spread multiplier (1.0 at hip, 0.5 when aimed).
    pub spread_mult: f32,
    /// Movement speed multiplier (1.0 at hip, 0.7 when aimed).
    pub speed_mult: f32,
}

/// Tracks weapon bob phase per weapon model.
#[derive(Component)]
pub struct WeaponBobState {
    pub phase: f32,
}

impl Default for WeaponBobState {
    fn default() -> Self {
        Self { phase: 0.0 }
    }
}

const BOB_FREQ_WALK: f32 = 8.0;
const BOB_FREQ_SPRINT: f32 = 12.0;
const BOB_FREQ_CROUCH: f32 = 5.0;
const BOB_FREQ_PRONE: f32 = 0.0;
const BOB_AMP_WALK: f32 = 0.015;
const BOB_AMP_SPRINT: f32 = 0.03;
const BOB_AMP_CROUCH: f32 = 0.008;
const BOB_AMP_PRONE: f32 = 0.0;
const ADS_LERP_SPEED: f32 = 5.0;
const HIP_POS: Vec3 = Vec3::new(0.35, -0.25, -0.6);
const ADS_POS: Vec3 = Vec3::new(0.0, -0.1, -0.4);

fn bob_params(stance: &MovementState) -> (f32, f32) {
    match stance {
        MovementState::Sprinting => (BOB_FREQ_SPRINT, BOB_AMP_SPRINT),
        MovementState::Crouching => (BOB_FREQ_CROUCH, BOB_AMP_CROUCH),
        MovementState::Prone | MovementState::InCover => (BOB_FREQ_PRONE, BOB_AMP_PRONE),
        MovementState::Standing => (BOB_FREQ_WALK, BOB_AMP_WALK),
    }
}

/// Animates weapon bob and updates the shared ADS state resource.
pub fn weapon_bob_system(
    time: Res<Time>,
    player_query: Query<(&ActionState<PlayerAction>, &MovementState, &Stamina), With<Player>>,
    mut bob_query: Query<(&mut WeaponBobState, &mut Transform), With<WeaponModelRoot>>,
    mut ads_state: ResMut<AdsState>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok((action_state, stance, stamina)) = player_query.single() else {
        return;
    };

    let moving = action_state.axis_pair(&PlayerAction::Move) != Vec2::ZERO;
    let (freq, amp) = bob_params(stance);
    let aiming = action_state.pressed(&PlayerAction::Aim);

    // Apply stamina sway multiplier
    let sway_mult = stamina_sway_mult(stamina);
    let final_amp = amp * sway_mult;

    // Update ADS factor
    let target_ads = if aiming { 1.0 } else { 0.0 };
    let ads = ads_state.factor;
    let new_ads = ads + (target_ads - ads) * (ADS_LERP_SPEED * dt).min(1.0);
    ads_state.factor = new_ads;
    ads_state.spread_mult =
        (1.0 - new_ads * 0.5) * (if stamina.is_exhausted() { 1.5 } else { 1.0 });
    ads_state.speed_mult = 1.0 - new_ads * 0.3;

    for (mut bob, mut transform) in bob_query.iter_mut() {
        if moving && freq > 0.0 {
            bob.phase += freq * dt;
        } else {
            bob.phase *= 0.9;
        }

        let bob_y = final_amp * bob.phase.sin();
        let bob_x = final_amp * 0.5 * (bob.phase * 1.3).cos();
        let base = HIP_POS.lerp(ADS_POS, new_ads);
        transform.translation = base + Vec3::new(bob_x, bob_y, 0.0);
    }
}

/// Applies ADS FOV modifier to the camera.
pub fn ads_fov_system(
    ads_state: Res<AdsState>,
    mut cam_query: Query<(&ThirdPersonCamera, &mut bevy::camera::Projection)>,
) {
    for (tp_cam, mut projection) in cam_query.iter_mut() {
        if let bevy::camera::Projection::Perspective(ref mut persp) = projection.as_mut() {
            persp.fov = (tp_cam.fov - ads_state.factor * 10.0).to_radians();
        }
    }
}
