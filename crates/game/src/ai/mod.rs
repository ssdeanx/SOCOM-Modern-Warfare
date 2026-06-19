use bevy::prelude::*;

use socom_core::resources::is_not_paused;

pub mod enemy;
pub mod teammate;

// ── System set for AI logic ──────────────────────────────────────────────────

/// Label applied to all AI systems that set `CharacterController` velocity.
/// Used by `PhysicsPlugin` to ensure movement runs *after* AI tick.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AiSystems;

// ── Plugin ───────────────────────────────────────────────────────────────────

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(teammate::TeammatePlugin);
        app.add_systems(
            Update,
            (
                enemy::patrol_system,
                enemy::detection_system,
                enemy::engage_system,
                enemy::enemy_death_system,
            )
                .in_set(AiSystems)
                .run_if(is_not_paused),
        );
    }
}

// ── AI state machine ─────────────────────────────────────────────────────────

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiState {
    /// Following waypoints, no contact.
    Patrol,
    /// Investigating last-known position or sound.
    Alert,
    /// Actively engaging a hostile target.
    Engage,
}

impl Default for AiState {
    fn default() -> Self {
        Self::Patrol
    }
}

// ── Waypoints ────────────────────────────────────────────────────────────────

/// List of world-space positions an AI entity patrols between.
#[derive(Component, Debug, Clone)]
pub struct PatrolRoute {
    pub waypoints: Vec<Vec3>,
    pub current_index: usize,
    pub wait_timer: Timer,
    pub is_waiting: bool,
}

impl PatrolRoute {
    /// Create a simple two-point patrol between two positions.
    pub fn between(a: Vec3, b: Vec3) -> Self {
        Self {
            waypoints: vec![a, b],
            current_index: 0,
            wait_timer: Timer::from_seconds(2.0, TimerMode::Once),
            is_waiting: false,
        }
    }
}

// ── Vision / Detection ───────────────────────────────────────────────────────

#[derive(Component, Debug, Clone)]
pub struct VisionCone {
    /// Horizontal field of view in radians (full width).
    pub fov_h: f32,
    /// Vertical field of view in radians (full height).
    pub fov_v: f32,
    /// Maximum detection range in metres.
    pub range: f32,
    /// Suspicion level 0–100. At 100 the AI transitions to Alert.
    pub suspicion: f32,
}

impl Default for VisionCone {
    fn default() -> Self {
        Self {
            fov_h: 2.094, // ~120°
            fov_v: 1.047, // ~60°
            range: 40.0,
            suspicion: 0.0,
        }
    }
}
