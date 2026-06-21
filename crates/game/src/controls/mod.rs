pub mod stance;
pub mod turn_rate;

use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stance::stance_transition_system);
        app.add_systems(Update, turn_rate::update_sensitivity_multiplier);
    }
}
