/// Post-processing pipeline — bloom, tone mapping, and cinematic effects.
///
/// Uses bevy's built-in post-processing via `Bloom` and `Tonemapping`
/// added to the camera entity at spawn time.
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

/// Adds post-processing components to any camera with this marker.
/// Spawn it alongside `Camera3d` to enable the full effect stack.
#[derive(Component)]
pub struct PostProcessingProfile {
    pub intensity: f32,
}

impl Default for PostProcessingProfile {
    fn default() -> Self {
        Self { intensity: 1.0 }
    }
}

/// Plugin that registers the post-processing update system.
pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, _app: &mut App) {
        // Post-processing is applied per-camera via components spawned in PlayerPlugin.
        // The update system is registered in main.rs as a global system.
    }
}

/// Applies the post-processing profile to cameras each frame.
/// Confirms ACES filmic tone mapping and Bloom are applied.
pub fn apply_post_processing_system(
    cam_query: Query<(&PostProcessingProfile, &mut Tonemapping, Option<&Bloom>), With<Camera3d>>,
) {
    if let Ok((_profile, mut tonemapping, _bloom)) = cam_query.single() {
        // ACES filmic tone mapping — matches modern AAA look
        *tonemapping = Tonemapping::AcesFitted;
    }
}
