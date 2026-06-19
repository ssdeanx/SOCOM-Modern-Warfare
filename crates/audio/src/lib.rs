pub mod ambient;
pub mod footsteps;

use bevy::prelude::*;

use crate::ambient::AmbientPlugin;
use crate::footsteps::FootstepPlugin;

/// Master audio plugin that registers all audio sub-plugins.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FootstepPlugin, AmbientPlugin));
    }
}
