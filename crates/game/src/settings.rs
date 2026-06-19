use bevy::audio::PlaybackSettings;
use bevy::prelude::*;

use socom_core::resources::GameSettings;

/// Plugin for loading, saving, and applying game settings.
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default());
        app.add_systems(Startup, load_settings);
        app.add_systems(Update, apply_audio_volume);
    }
}

/// Attempts to load settings from disk. Falls back to defaults.
fn load_settings(mut settings: ResMut<GameSettings>) {
    let path = dirs_config_path();
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(loaded) = ron::from_str::<GameSettings>(&content) {
            *settings = loaded;
            return;
        }
    }
    // Save defaults if no file exists.
    let _ = save_to_disk(&*settings);
}

/// Applies the current master volume to all active audio sources.
fn apply_audio_volume(settings: Res<GameSettings>, mut query: Query<&mut PlaybackSettings>) {
    let master = settings.master_volume.max(0.0).min(1.0);
    for mut playback in query.iter_mut() {
        playback.volume = bevy::audio::Volume::Linear(master);
    }
}

/// Returns the path to the settings file.
fn dirs_config_path() -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    format!("{home}/.socom/settings.ron")
}

/// Saves settings to disk.
fn save_to_disk(settings: &GameSettings) -> Result<(), Box<dyn std::error::Error>> {
    let path = dirs_config_path();
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = ron::ser::to_string_pretty(settings, ron::ser::PrettyConfig::default())?;
    std::fs::write(&path, content)?;
    Ok(())
}
