use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};
use socom_core::resources::GameSettings;

pub fn apply_settings_system(settings: Res<GameSettings>, mut window_query: Query<&mut Window>) {
    if !settings.is_changed() {
        return;
    }
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };
    window.mode = if settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}
