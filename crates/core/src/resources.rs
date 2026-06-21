use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Resource;

// ═══════════════════════════════════════════════════════════════════════════════
// DISPLAY MODE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Regular windowed mode
    Windowed,
    /// Borderless fullscreen (matches monitor resolution)
    Borderless,
    /// Exclusive fullscreen
    Exclusive,
}

impl DisplayMode {
    pub fn next(&self) -> Self {
        match self {
            Self::Windowed => Self::Borderless,
            Self::Borderless => Self::Exclusive,
            Self::Exclusive => Self::Windowed,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Windowed => "Windowed",
            Self::Borderless => "Borderless",
            Self::Exclusive => "Fullscreen",
        }
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Borderless
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GRAPHICS QUALITY
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl GraphicsQuality {
    pub fn next(&self) -> Self {
        match self {
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High => Self::Ultra,
            Self::Ultra => Self::Low,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Ultra => "Ultra",
        }
    }

    /// Shadow map resolution (power-of-two side length).
    pub fn shadow_map_size(&self) -> u32 {
        match self {
            Self::Low => 512,
            Self::Medium => 1024,
            Self::High => 2048,
            Self::Ultra => 4096,
        }
    }

    /// Max shadow distance factor (multiplied into the cascade far planes).
    pub fn shadow_distance_factor(&self) -> f32 {
        match self {
            Self::Low => 0.5,
            Self::Medium => 0.75,
            Self::High => 1.0,
            Self::Ultra => 1.5,
        }
    }

    /// Whether bloom is enabled at this quality level.
    pub fn bloom_enabled(&self) -> bool {
        match self {
            Self::Low => false,
            Self::Medium => true,
            Self::High => true,
            Self::Ultra => true,
        }
    }
}

impl Default for GraphicsQuality {
    fn default() -> Self {
        Self::High
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GAME SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════

/// Global game settings persisted between sessions.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    // ── Display ──
    pub display_mode: DisplayMode,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub vsync: bool,

    // ── Graphics ──
    pub graphics_quality: GraphicsQuality,
    pub bloom_enabled: bool,

    // ── Camera ──
    pub fov_third_person: f32,
    pub fov_first_person: f32,
    pub camera_collision: bool,
    pub camera_shake: bool,

    // ── Audio ──
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,

    // ── Controls ──
    pub sensitivity: f32,
    pub invert_y: bool,
    /// Hold-to-sprint mode (default: true — Shift held = sprint, released = walk).
    pub hold_to_sprint: bool,
    /// Hold-to-crouch mode (default: false — toggle on/off with C).
    pub hold_to_crouch: bool,

    // ── Misc ──
    /// Kept for backward compat — no longer used directly, DisplayMode replaces it.
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            // Display
            display_mode: DisplayMode::default(),
            resolution_width: 1920,
            resolution_height: 1080,
            vsync: true,
            // Graphics
            graphics_quality: GraphicsQuality::High,
            bloom_enabled: true,
            // Camera
            fov_third_person: 70.0,
            fov_first_person: 80.0,
            camera_collision: true,
            camera_shake: true,
            // Audio
            master_volume: 0.8,
            sfx_volume: 1.0,
            music_volume: 0.5,
            // Controls
            sensitivity: 1.0,
            invert_y: false,
            hold_to_sprint: true,
            hold_to_crouch: false,
            // Misc
            fullscreen: false,
        }
    }
}

/// Input keybinding profile (placeholder for Phase 1).
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
