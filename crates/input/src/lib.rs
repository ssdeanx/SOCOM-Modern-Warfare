pub mod actions;
pub mod bindings;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::actions::PlayerAction;
use crate::bindings::default_input_map;

/// Plugin that registers leafwing input handling for the player.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .insert_resource(default_input_map());
    }
}
