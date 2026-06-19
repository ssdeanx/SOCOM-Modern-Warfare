use bevy::prelude::*;

use crate::menu::MenuPlugin;

/// Thin wrapper delegating to the full menu module.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin);
    }
}
