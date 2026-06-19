use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_core::resources::is_not_paused;

const FOOTSTEP_INTERVAL_WALK: f32 = 0.5; // seconds
const FOOTSTEP_INTERVAL_SPRINT: f32 = 0.35;
const FOOTSTEP_INTERVAL_CROUCH: f32 = 0.7;
const FOOTSTEP_INTERVAL_PRONE: f32 = 1.0;

/// Tracks footstep timing and previous position per player entity.
/// Uses position delta between frames to compute actual movement speed,
/// which is more accurate than checking `transform.translation.length()`
/// (that measures distance from world origin, not movement).
#[derive(Component, Default)]
pub struct FootstepTimer {
    pub elapsed: f32,
    pub previous_position: Option<Vec3>,
}

/// Plugin that registers footstep audio systems
pub struct FootstepPlugin;

impl Plugin for FootstepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, footstep_system.run_if(is_not_paused));
    }
}

/// Simple hash of an f32 value into a usize for pseudo-random selection.
fn hash_to_index(value: f32, max: usize) -> usize {
    let bits = value.to_bits() as u64;
    // Mix bits using a simple LCG (64-bit)
    let mixed = bits
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (mixed as usize) % max
}

/// Available footstep surface types.
const SURFACES: &[&str] = &["dirt", "gravel", "concrete", "metal", "grass"];

/// Plays footsteps when the player moves, varying interval by movement state.
/// Uses a pseudo-random surface variant each step.
/// Movement speed is derived from the position delta between frames,
/// avoiding the common mistake of reading `transform.translation.length()`
/// (which returns the entity's distance from the world origin, not speed).
fn footstep_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<(&Transform, Option<&MovementState>, &mut FootstepTimer), With<Player>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (transform, movement_state, mut timer) in query.iter_mut() {
        // Compute actual movement speed from frame-to-frame position delta.
        let speed = timer
            .previous_position
            .map(|prev| (transform.translation - prev).length() / dt)
            .unwrap_or(0.0);
        timer.previous_position = Some(transform.translation);

        if speed < 0.1 {
            timer.elapsed = 0.0;
            continue;
        }

        let interval = match movement_state {
            Some(MovementState::Sprinting) => FOOTSTEP_INTERVAL_SPRINT,
            Some(MovementState::Crouching) => FOOTSTEP_INTERVAL_CROUCH,
            Some(MovementState::Prone) | Some(MovementState::InCover) => FOOTSTEP_INTERVAL_PRONE,
            _ => FOOTSTEP_INTERVAL_WALK,
        };

        timer.elapsed += dt;
        if timer.elapsed >= interval {
            let seed = time.elapsed_secs() + transform.translation.x;
            timer.elapsed = 0.0;
            // Pick a random surface and variant for variety.
            let surface = SURFACES[hash_to_index(seed, SURFACES.len())];
            let variant = hash_to_index(seed * 0.5, 3) + 1;
            let path = format!("audio/footstep_{}_{}.ogg", surface, variant);
            let handle: Handle<AudioSource> = asset_server.load(&path);
            commands.spawn((
                AudioPlayer(handle),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(1.0)),
            ));
        }
    }
}
