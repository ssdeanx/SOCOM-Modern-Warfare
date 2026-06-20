# Audio Crate (`socom-audio`)

**Path:** `crates/audio/`  
**Files:** `lib.rs`, `ambient.rs`, `footsteps.rs`, `kira_host.rs`, `weapon_audio.rs`  
**Dependencies:** `bevy 0.18.1`, `core`, `kira 0.12`  
**Purpose:** Dual audio system — bevy_audio for simple SFX + kira standalone host for advanced audio

## Architecture

The audio crate operates two independent audio systems that coexist without conflict:

1. **`bevy_audio`** — Used for simple one-shot sounds (footsteps, weapon fire) via `AudioPlayer` + `PlaybackSettings`
2. **`kira` standalone** — Used for advanced audio (bus hierarchy, spatial audio, occlusion filtering)

```
socom-audio (lib)
├── ambient.rs       — AmbientPlugin (bevy_audio looping ambient)
├── footsteps.rs     — FootstepPlugin (bevy_audio procedural footstep SFX)
├── weapon_audio.rs  — WeaponAudioPlugin (placeholder relay scaffold)
└── kira_host.rs     — KiraHostPlugin (kira 0.12 standalone host)
```

## Sub-plugins

### AmbientPlugin
Starts a looping ambient audio track on startup.

```rust
fn start_ambient() {
    // loads audio/ambient_test.ogg
    // spawns AudioPlayer + PlaybackSettings::LOOP
}
```

### FootstepPlugin
Procedural footstep system with stance-aware timing and pseudo-random surface variation.

- Tracks position delta between frames for accurate speed calculation
- Interval varies by stance: Walk=0.5s, Sprint=0.35s, Crouch=0.7s, Prone=1.0s
- 5 surface types: dirt, gravel, concrete, metal, grass
- Pseudo-random surface + variant selection per footstep

### WeaponAudioPlugin
Placeholder bridge — actual relay logic lives in the game crate's `audio_relay.rs`.

## Kira Host (`kira_host.rs`)

### Bus Hierarchy
```
Master
├── SFX Bus      (gunshots, impacts, footsteps) — with low-pass filter
├── Ambient Bus  (looping environmental tracks)
├── UI Bus       (menu clicks, notifications)
└── Voice Bus    (dialogue, radio chatter)
```

### KiraAudioState Resource
```rust
pub struct KiraAudioState {
    pub manager: Option<AudioManager<DefaultBackend>>,
    pub sfx_bus: Option<TrackHandle>,
    pub ambient_bus: Option<TrackHandle>,
    pub ui_bus: Option<TrackHandle>,
    pub voice_bus: Option<TrackHandle>,
    pub listener: Option<ListenerHandle>,
    pub sfx_filter: Option<FilterHandle>,  // Low-pass for occlusion
}
```

### Public API

| Function | Bus | Description |
|----------|-----|-------------|
| `play_sfx()` | SFX | One-shot sound effect |
| `play_ambient()` | Ambient | Looping/drone audio |
| `play_ui()` | UI | Menu sounds |
| `play_voice()` | Voice | Dialogue/radio |
| `set_occlusion_filter()` | SFX | Low-pass occlusion (0.0–1.0) |

### Key Design Decisions

- Uses **mint types** for kira because kira 0.12 depends on glam 0.33 while Bevy 0.18 uses glam 0.30
- All public functions return `Option` — gracefully handle missing audio device
- SFX bus has a **pre-attached low-pass filter** started at 20 kHz (fully open) for occlusion effects
- Listener follows the primary 3D camera each frame

### Occlusion Filter API
```rust
set_occlusion_filter(&mut state, intensity: f32)
// Maps 0.0 → 20,000 Hz (fully open) to 1.0 → 20 Hz (fully muffled)
```

### Scene Transition Helpers
- `stop_all()` — Pause all buses
- `resume_all()` — Resume all buses
