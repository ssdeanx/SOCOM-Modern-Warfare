use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player, Shoulder};
use socom_core::resources::SensitivityMultiplier;
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::weapon_bob::AdsState;
use crate::controls::turn_rate;
use crate::messages::WeaponFiredMessage;
use crate::physics::movement_modifiers::SprintBrake;
use crate::stamina::Stamina;
use crate::states::AppState;
use crate::weapon_handling::WeaponHandling;
use crate::weapons::caliber::Caliber;
use crate::weapons::EquippedWeapon;

/// Shake trauma added per weapon caliber when fired.
fn shake_trauma_for_caliber(caliber: Caliber) -> f32 {
    match caliber {
        Caliber::NineMm => 0.15,
        Caliber::FortyFiveACP => 0.20,
        Caliber::FiveFiveSixNato => 0.25,
        Caliber::SevenSixTwoX39 => 0.30,
        Caliber::SevenSixTwoNato => 0.35,
        Caliber::TwelveGauge => 0.40,
        Caliber::FiftyBMG => 0.50,
    }
}

/// Handles camera-related input: shoulder swap and freelook.
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SensitivityMultiplier(1.0));
        app.add_systems(
            Update,
            (
                shoulder_swap_system,
                freelook_system,
                turn_rate_update_system,
                camera_shake_on_fire_system,
                ads_sync_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// Reads WeaponFiredMessage and applies shake trauma to the camera,
/// scaled by the weapon's caliber (bigger gun = more shake).
fn camera_shake_on_fire_system(
    mut fired_reader: MessageReader<WeaponFiredMessage>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
    weapon_query: Query<&EquippedWeapon>,
) {
    for msg in fired_reader.read() {
        // Determine shake intensity from caliber, defaulting to 0.2
        let trauma = weapon_query
            .get(msg.shooter)
            .ok()
            .map(|eq| shake_trauma_for_caliber(eq.weapon.caliber))
            .unwrap_or(0.20);

        for mut cam in camera_query.iter_mut() {
            // Accumulate trauma (capped at 1.0) so rapid fire builds up
            cam.shake_trauma = (cam.shake_trauma + trauma).min(1.0);
        }
    }
}

/// Updates the sensitivity multiplier based on stance, weapon, stamina, ADS state, and sprint braking.
pub fn turn_rate_update_system(
    player_query: Query<(&MovementState, &Stamina, &WeaponHandling, &SprintBrake), With<Player>>,
    mut sens_mult: ResMut<SensitivityMultiplier>,
    ads_state: Res<AdsState>,
) {
    let Ok((stance, stamina, handling, brake)) = player_query.single() else {
        return;
    };
    sens_mult.0 = turn_rate::turn_rate_mult(
        stance,
        handling.current_weight_mult,
        stamina,
        ads_state.factor,
        brake.timer.fraction() < 1.0,
    );
}

/// Toggles the camera shoulder on ShoulderSwap input (Q key / LeftShoulder).
fn shoulder_swap_system(
    player_query: Query<&ActionState<PlayerAction>, With<socom_core::components::Player>>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
) {
    let Ok(action_state) = player_query.single() else {
        return;
    };
    if !action_state.just_pressed(&PlayerAction::ShoulderSwap) {
        return;
    }
    for mut cam in camera_query.iter_mut() {
        cam.shoulder = match cam.shoulder {
            Shoulder::Right => Shoulder::Left,
            Shoulder::Left => Shoulder::Right,
        };
    }
}

/// Toggles freelook mode on middle-mouse hold.
/// When freelooking, the camera stops orbiting with mouse look,
/// allowing the player to look around while running in a fixed direction.
fn freelook_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
) {
    let held = mouse.pressed(MouseButton::Middle);
    for mut cam in camera_query.iter_mut() {
        cam.freelook = held;
    }
}

/// Syncs the ADS factor from weapon_bob's AdsState into the camera component.
/// This is the missing link that makes ADS FOV zoom and distance reduction work.
fn ads_sync_system(
    ads_state: Res<AdsState>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
) {
    for mut cam in camera_query.iter_mut() {
        cam.ads_factor = ads_state.factor;
    }
}
