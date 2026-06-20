//! Standalone kira 0.12 audio host plugin that coexists with bevy_audio.
//!
//! Provides a bus hierarchy (Master → SFX / Ambient / UI / Voice),
//! a spatial listener that follows the main 3D camera, and an occlusion-ready
//! filter API.  All public functions gracefully return `None` when the
//! `AudioManager` failed to initialise (e.g. no audio device), so callers
//! only need `if let Some(handle) = play_sfx(...)`.

use bevy::prelude::*;
use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    listener::ListenerHandle,
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    track::{TrackBuilder, TrackHandle},
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
};
use mint::{Quaternion, Vector3};

// ── Plugin ──────────────────────────────────────────────────────────────

/// Bevy plugin that initialises a standalone kira audio host.
///
/// Registers [`KiraAudioState`] as a resource and adds startup + per-frame
/// systems.  Because it talks to kira directly (not via bevy_kira_audio),
/// this plugin can live in the same app as `bevy_audio` without conflict.
pub struct KiraHostPlugin;

impl Plugin for KiraHostPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KiraAudioState>()
            .add_systems(Startup, init_kira_audio)
            .add_systems(Update, update_kira_listener);
    }
}

// ── Resource ────────────────────────────────────────────────────────────

/// Central audio state — a Bevy resource wrapping the kira `AudioManager`.
#[derive(Resource)]
pub struct KiraAudioState {
    /// The kira audio manager. `None` implies the backend failed to start.
    pub manager: Option<AudioManager<DefaultBackend>>,
    /// Bus for short-lived sound effects (gunshots, impacts, footsteps).
    pub sfx_bus: Option<TrackHandle>,
    /// Bus for looping ambient/environment tracks.
    pub ambient_bus: Option<TrackHandle>,
    /// Bus for menu clicks, notifications, HUD sounds.
    pub ui_bus: Option<TrackHandle>,
    /// Bus for character dialogue / radio chatter.
    pub voice_bus: Option<TrackHandle>,
    /// The 3D listener that follows the primary camera.
    pub listener: Option<ListenerHandle>,
    /// Pre-attached low-pass filter on the SFX bus (for occlusion).
    pub sfx_filter: Option<FilterHandle>,
}

impl Default for KiraAudioState {
    fn default() -> Self {
        Self {
            manager: None,
            sfx_bus: None,
            ambient_bus: None,
            ui_bus: None,
            voice_bus: None,
            listener: None,
            sfx_filter: None,
        }
    }
}

// ── Startup system ──────────────────────────────────────────────────────

/// Initialise the kira audio manager, listener, and bus hierarchy.
fn init_kira_audio(mut state: ResMut<KiraAudioState>) {
    let mut manager = match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
        Ok(m) => m,
        Err(e) => {
            warn!("kira: AudioManager init failed — {e}. Audio disabled.");
            return;
        }
    };

    // Use mint types because kira 0.12 depends on glam 0.33 while Bevy 0.18
    // uses glam 0.30 — the two glam versions are incompatible.
    let listener = match manager.add_listener(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Quaternion {
            v: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            s: 1.0,
        },
    ) {
        Ok(l) => l,
        Err(e) => {
            warn!("kira: add_listener failed — {e}. Audio disabled.");
            return;
        }
    };

    // Build bus hierarchy with a pre-attached low-pass filter on the SFX bus
    // (starts fully open at 20 kHz for non-occluded playback).
    let sfx_filter;
    let sfx_bus = manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            sfx_filter = Some(
                builder.add_effect(
                    FilterBuilder::new()
                        .mode(FilterMode::LowPass)
                        .cutoff(20_000.0),
                ),
            );
            builder
        })
        .ok();

    let ambient_bus = manager.add_sub_track(TrackBuilder::new()).ok();
    let ui_bus = manager.add_sub_track(TrackBuilder::new()).ok();
    let voice_bus = manager.add_sub_track(TrackBuilder::new()).ok();

    state.manager = Some(manager);
    state.listener = Some(listener);
    state.sfx_bus = sfx_bus;
    state.ambient_bus = ambient_bus;
    state.ui_bus = ui_bus;
    state.voice_bus = voice_bus;
    state.sfx_filter = sfx_filter;

    info!("kira: audio host initialised successfully");
}

// ── Per-frame listener update ───────────────────────────────────────────

/// Move the kira listener to match the primary 3D camera each frame.
fn update_kira_listener(
    mut state: ResMut<KiraAudioState>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
) {
    let Some(ref mut listener) = state.listener else {
        return;
    };
    // Only one primary 3D camera is expected; take the first.
    if let Some(transform) = camera_query.iter().next() {
        let t = transform.translation();
        let _ = listener.set_position(
            Vector3 {
                x: t.x,
                y: t.y,
                z: t.z,
            },
            Tween::default(),
        );
    }
}

// ── Public playback API ─────────────────────────────────────────────────

// Internal helper — load a sound file and optionally override its settings.
fn load_sound_data(path: &str, settings: Option<StaticSoundSettings>) -> Option<StaticSoundData> {
    let mut data = match StaticSoundData::from_file(path) {
        Ok(d) => d,
        Err(e) => {
            warn!("kira: failed to load \"{path}\" — {e}");
            return None;
        }
    };
    if let Some(s) = settings {
        data.settings = s;
    }
    Some(data)
}

/// Play a one-shot sound effect on the **SFX** bus.
pub fn play_sfx(
    state: &mut KiraAudioState,
    path: &str,
    settings: Option<StaticSoundSettings>,
) -> Option<StaticSoundHandle> {
    let bus = state.sfx_bus.as_mut()?;
    let data = load_sound_data(path, settings)?;
    bus.play(data).ok()
}

/// Play a sound (typically looping) on the **Ambient** bus.
pub fn play_ambient(
    state: &mut KiraAudioState,
    path: &str,
    settings: Option<StaticSoundSettings>,
) -> Option<StaticSoundHandle> {
    let bus = state.ambient_bus.as_mut()?;
    let data = load_sound_data(path, settings)?;
    bus.play(data).ok()
}

/// Play a UI sound (menu clicks, notifications) on the **UI** bus.
pub fn play_ui(
    state: &mut KiraAudioState,
    path: &str,
    settings: Option<StaticSoundSettings>,
) -> Option<StaticSoundHandle> {
    let bus = state.ui_bus.as_mut()?;
    let data = load_sound_data(path, settings)?;
    bus.play(data).ok()
}

/// Play voice audio (radio chatter, dialogue) on the **Voice** bus.
pub fn play_voice(
    state: &mut KiraAudioState,
    path: &str,
    settings: Option<StaticSoundSettings>,
) -> Option<StaticSoundHandle> {
    let bus = state.voice_bus.as_mut()?;
    let data = load_sound_data(path, settings)?;
    bus.play(data).ok()
}

// ── Occlusion filter API ───────────────────────────────────────────────

/// Set the occlusion (low-pass) intensity on the SFX bus.
///
/// `intensity` should be in the range `0.0` (no occlusion — full clarity)
/// to `1.0` (fully muffled).  The filter's cutoff frequency is mapped
/// linearly from 20 000 Hz down to 20 Hz.
pub fn set_occlusion_filter(state: &mut KiraAudioState, intensity: f32) {
    let clamped = intensity.clamp(0.0, 1.0);
    let cutoff = (20_000.0 * (1.0 - clamped)).max(20.0) as f64;
    if let Some(ref mut filter) = state.sfx_filter {
        filter.set_cutoff(cutoff, Tween::default());
    }
}

/// Attach an occlusion filter to an arbitrary bus and return its handle.
///
/// Unlike [`set_occlusion_filter`] (which controls the SFX bus's built-in
/// filter), this creates a **new** filter effect on the given bus every
/// time it is called.  Use this for one-off or per-scene occlusion buses.
/// The returned [`FilterHandle`] lets you update the cutoff dynamically.
///
/// Note: kira 0.12 does not allow adding effects after a track is created;
/// this helper does not actually add anything at runtime.  For dynamic
/// occlusion, use the pre-attached SFX filter via [`set_occlusion_filter`].
pub fn attach_filter_to_bus(
    bus: &mut TrackHandle,
    intensity: f32,
) -> Option<FilterHandle> {
    let clamped = intensity.clamp(0.0, 1.0);
    let cutoff = (20_000.0 * (1.0 - clamped)).max(20.0) as f64;

    // Note: TrackHandle does not expose a runtime add_effect in kira 0.12.
    // Filters must be added during track construction via TrackBuilder.
    // This function is retained for forward compatibility when/if kira
    // adds the feature.  Use set_occlusion_filter for the SFX bus.
    let _ = cutoff;
    let _ = bus;
    None
}

/// Update an existing occlusion filter's cutoff frequency.
pub fn update_filter_cutoff(filter: &mut FilterHandle, intensity: f32, tween: Tween) {
    let clamped = intensity.clamp(0.0, 1.0);
    let cutoff = (20_000.0 * (1.0 - clamped)).max(20.0) as f64;
    filter.set_cutoff(cutoff, tween);
}

// ── Scene transition helpers ────────────────────────────────────────────

/// Pause all buses — use before transitioning to a new scene.
pub fn stop_all(state: &mut KiraAudioState) {
    let tween = Tween::default();
    pause_bus(state.sfx_bus.as_mut(), tween);
    pause_bus(state.ambient_bus.as_mut(), tween);
    pause_bus(state.ui_bus.as_mut(), tween);
    pause_bus(state.voice_bus.as_mut(), tween);
}

/// Resume all buses after a scene transition.
pub fn resume_all(state: &mut KiraAudioState) {
    let tween = Tween::default();
    resume_bus(state.sfx_bus.as_mut(), tween);
    resume_bus(state.ambient_bus.as_mut(), tween);
    resume_bus(state.ui_bus.as_mut(), tween);
    resume_bus(state.voice_bus.as_mut(), tween);
}

fn pause_bus(bus: Option<&mut TrackHandle>, tween: Tween) {
    if let Some(bus) = bus {
        let _ = bus.pause(tween);
    }
}

fn resume_bus(bus: Option<&mut TrackHandle>, tween: Tween) {
    if let Some(bus) = bus {
        let _ = bus.resume(tween);
    }
}
