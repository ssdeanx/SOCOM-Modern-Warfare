use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player, Shoulder};
use socom_core::resources::SensitivityMultiplier;
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::controls::turn_rate;
use crate::stamina::Stamina;
use crate::states::AppState;
use crate::weapon_handling::WeaponHandling;

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
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// Updates the sensitivity multiplier based on stance, weapon, stamina.
pub fn turn_rate_update_system(
    player_query: Query<(&MovementState, &Stamina, &WeaponHandling), With<Player>>,
    mut sens_mult: ResMut<SensitivityMultiplier>,
) {
    let Ok((stance, stamina, handling)) = player_query.single() else {
        return;
    };
    sens_mult.0 = turn_rate::turn_rate_mult(stance, handling.current_weight_mult, stamina);
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
