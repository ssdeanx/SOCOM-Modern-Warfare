use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::ai::AiPlugin;
use crate::combat::CombatPlugin;
use crate::hud::HudPlugin;
use crate::level::LevelPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::squad::SquadPlugin;
use crate::states::AppState;
use crate::tactical::TacticalPlugin;

/// Plugin for the in-game state.
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (setup_ingame, capture_cursor));
        app.add_systems(OnExit(AppState::InGame), (cleanup_ingame, release_cursor));

        // Phase 1 gameplay plugins
        app.add_plugins((
            PlayerPlugin,
            PhysicsPlugin,
            LevelPlugin,
            CombatPlugin,
            TacticalPlugin,
            SquadPlugin,
            AiPlugin,
            HudPlugin,
        ));
    }
}

/// Marker component for in-game entities.
/// Any entity tagged with this during `InGame` state will be automatically
/// despawned when the state exits (return to menu, etc.).
#[derive(Component)]
pub struct IngameEntity;

fn setup_ingame(mut commands: Commands) {
    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -2.0, -1.0), Vec3::Y),
        IngameEntity,
    ));

    // Ambient light
    commands.spawn((
        AmbientLight {
            color: Color::srgb(0.1, 0.1, 0.1),
            brightness: 0.5,
            ..default()
        },
        IngameEntity,
    ));
}

fn cleanup_ingame(mut commands: Commands, query: Query<Entity, With<IngameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Grabs and hides the cursor when entering gameplay so mouse look works
/// without the cursor leaving the window.
/// Uses the separate `CursorOptions` component (Bevy 0.18 `#[require]` pattern).
fn capture_cursor(mut cursor_query: Query<&mut CursorOptions>) {
    let Ok(mut cursor) = cursor_query.single_mut() else {
        return;
    };
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

/// Releases the cursor when exiting gameplay (returning to menu, etc.)
/// so the player can interact with UI normally.
fn release_cursor(mut cursor_query: Query<&mut CursorOptions>) {
    let Ok(mut cursor) = cursor_query.single_mut() else {
        return;
    };
    cursor.grab_mode = CursorGrabMode::None;
    cursor.visible = true;
}
