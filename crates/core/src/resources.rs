use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Resource;

/// Global game settings persisted between sessions
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    // Audio
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    // Controls
    pub sensitivity: f32,
    pub invert_y: bool,
    // Video
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            sfx_volume: 1.0,
            music_volume: 0.5,
            sensitivity: 1.0,
            invert_y: false,
            fullscreen: false,
            vsync: true,
        }
    }
}

/// Input keybinding profile (placeholder for Phase 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputMapping {
    pub move_forward: String,
    pub move_backward: String,
    pub move_left: String,
    pub move_right: String,
    pub sprint: String,
    pub crouch: String,
    pub prone: String,
    pub jump: String,
    pub interact: String,
    pub pause: String,
}

impl Default for InputMapping {
    fn default() -> Self {
        Self {
            move_forward: "KeyW".into(),
            move_backward: "KeyS".into(),
            move_left: "KeyA".into(),
            move_right: "KeyD".into(),
            sprint: "ShiftLeft".into(),
            crouch: "KeyC".into(),
            prone: "KeyZ".into(),
            jump: "Space".into(),
            interact: "KeyE".into(),
            pause: "Escape".into(),
        }
    }
}

/// Pause state for the game.
///
/// When `true`, gameplay systems are frozen and the pause overlay is shown.
/// Toggled by pressing Escape during gameplay.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Paused(pub bool);

/// Multiplier applied to mouse sensitivity each frame.
/// Updated by the game crate based on stance + weapon weight + stamina.
/// Read by the rendering crate's camera_look_system.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Debug, Clone, PartialEq)]
pub struct SensitivityMultiplier(pub f32);

impl Default for SensitivityMultiplier {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Run condition: returns `true` when the game is NOT paused.
/// Use this with `.run_if()` on all gameplay systems.
#[cfg(feature = "bevy")]
pub fn is_not_paused(paused: bevy_ecs::prelude::Res<Paused>) -> bool {
    !paused.0
}
