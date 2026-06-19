# Module: `audio`

**Path:** `crates/audio/`

Manages footstep timing and ambient audio loops using Bevy's built-in audio engine (`bevy_audio`). Uses the v0.18 API pattern of spawning `AudioPlayer` entities rather than calling `Res<Audio>.play()`.

## Responsibilities

- `FootstepPlugin` — emits footstep audio at intervals determined by movement state
- `AmbientPlugin` — starts a looping ambient background audio track
- Provide a master `AudioPlugin` that composes both sub-plugins

## Key Files

- [`footsteps.rs`](../../crates/audio/src/footsteps.rs) — Footstep timing system + timer resource
- [`ambient.rs`](../../crates/audio/src/ambient.rs) — Ambient loop startup system
- [`lib.rs`](../../crates/audio/src/lib.rs) — Crate root + `AudioPlugin`

## Public API

### `AudioPlugin`

```rust
pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FootstepPlugin, AmbientPlugin));
    }
}
```

### `FootstepPlugin`

Registers a `footstep_system` in `Update` that queries `(Player, &MovementState)`. Each entity gets a `FootstepTimer` resource (local to the system) with an interval based on movement state:

| State | Interval |
|-------|----------|
| Standing (moving) | 0.5s |
| Sprinting | 0.35s |
| Crouching | 0.7s |
| Prone | 1.0s |

When the timer fires, it spawns:
```rust
commands.spawn((
    AudioPlayer::new(handle),
    PlaybackSettings::ONCE.with_volume(0.3),
));
```

### `AmbientPlugin`

On `Startup`, spawns a single looping audio entity:
```rust
commands.spawn((
    AudioPlayer::new(handle),
    PlaybackSettings::LOOP.with_volume(0.2),
));
```

## Dependencies

- **Used by:** `socom-game` (adds `AudioPlugin` in main)
- **Uses:** `socom-core` (with `"bevy"` feature for `Player`, `MovementState`), `bevy` (with `"vorbis"` feature for OGG decoding)

## Notable Patterns / Gotchas

- **Bevy 0.18 audio pattern:** No `Res<Audio>` — spawn entities with `(AudioPlayer(handle), PlaybackSettings)`.
- Audio assets are loaded from `assets/audio/ambient_test.ogg` and `assets/audio/footstep_{run,walk,crouch,prone}.ogg`. **None of these files exist yet** in Phase 0 — loading them logs a warning, the game continues without sound.
- Volume is set via `PlaybackSettings::ONCE.with_volume(Volume::new(0.3))` or the linear variant.
- Phase 2 will replace `bevy_audio` with `kira` for spatial audio, Doppler shifts, and per-sound bus control.
