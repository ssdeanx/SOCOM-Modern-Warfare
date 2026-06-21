use bevy::prelude::*;
use bevy::window::{MonitorSelection, VideoModeSelection, WindowMode, WindowResolution};

use socom_core::resources::{DisplayMode, GameSettings};

/// Applies display-related settings (resolution, mode, vsync) to the window.
pub fn apply_display_settings(
    settings: Res<GameSettings>,
    mut window_query: Query<&mut Window>,
) {
    if !settings.is_changed() {
        return;
    }
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    // ── Window mode ──
    window.mode = match settings.display_mode {
        DisplayMode::Windowed => WindowMode::Windowed,
        DisplayMode::Borderless => {
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        }
        DisplayMode::Exclusive => {
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
        }
    };

    // ── Resolution (only applied in windowed mode) ──
    if settings.display_mode == DisplayMode::Windowed {
        window.resolution = WindowResolution::new(
            settings.resolution_width,
            settings.resolution_height,
        );
    }

    // ── V-Sync ──
    window.present_mode = if settings.vsync {
        bevy::window::PresentMode::AutoVsync
    } else {
        bevy::window::PresentMode::AutoNoVsync
    };
}

/// Applies camera-related settings to the `ThirdPersonCamera` component.
pub fn apply_camera_settings(
    settings: Res<GameSettings>,
    mut camera_query: Query<&mut socom_rendering::camera::ThirdPersonCamera>,
) {
    if !settings.is_changed() {
        return;
    }
    for mut cam in camera_query.iter_mut() {
        cam.fov_third_person = settings.fov_third_person;
        cam.fov_first_person = settings.fov_first_person;
        cam.target_fov = if cam.perspective == socom_rendering::camera::CameraPerspective::FirstPerson {
            settings.fov_first_person
        } else {
            settings.fov_third_person
        };
        cam.collision = settings.camera_collision;
    }
}

/// Applies graphics-quality settings (bloom toggle, shadow quality).
pub fn apply_graphics_settings(
    settings: Res<GameSettings>,
    mut quality_query: Query<&mut socom_rendering::post_processing::PostProcessingProfile>,
) {
    if !settings.is_changed() {
        return;
    }

    let bloom_intensity = if settings.bloom_enabled { 1.0 } else { 0.0 };
    for mut profile in quality_query.iter_mut() {
        profile.intensity = bloom_intensity;
    }
}
