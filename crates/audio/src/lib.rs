pub mod ambient;
pub mod footsteps;
pub mod weapon_audio;

use bevy::prelude::*;

use crate::ambient::AmbientPlugin;
use crate::footsteps::FootstepPlugin;
use crate::weapon_audio::WeaponAudioPlugin;

/// Master audio plugin that registers all audio sub-plugins.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FootstepPlugin, AmbientPlugin, WeaponAudioPlugin));
    }
}
