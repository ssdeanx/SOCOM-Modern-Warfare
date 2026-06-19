use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::*;

/// Plugin that registers ambient audio systems
pub struct AmbientPlugin;

impl Plugin for AmbientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_ambient);
    }
}

/// Starts the ambient audio loop on app startup
fn start_ambient(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Phase 0: placeholder ambient loop
    // Loads from assets/audio/ambient_test.ogg (generated later)
    let handle: Handle<AudioSource> = asset_server.load("audio/ambient_test.ogg");
    commands.spawn((
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)),
    ));
}
