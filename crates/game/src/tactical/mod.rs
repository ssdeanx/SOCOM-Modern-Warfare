pub mod command_wheel;
pub mod cover;
pub mod suppression;

use bevy::prelude::*;

pub struct TacticalPlugin;

impl Plugin for TacticalPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(command_wheel::CommandWheelState::default());
        app.add_systems(
            Update,
            (
                command_wheel::command_wheel_input_system,
                command_wheel::command_wheel_ui_system,
                cover::cover_detection_system,
                suppression::suppression_system,
                suppression::suppression_fx_system,
            ),
        );
    }
}
