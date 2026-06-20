# Getting Started — SOCOM Tactical Shooter

## Prerequisites

- **Rust toolchain:** 1.85+ (edition 2024)
- **Cargo workspace:** Bevy 0.18.1, avian3d 0.6.1, leafwing-input-manager 0.20
- **Audio:** Dev files for ALSA/pipewire (Linux) or WASAPI (Windows, included)

## Quick Start

```bash
# Clone the repository
git clone <repo-url> socom
cd socom

# Build and run (debug)
cargo run

# Build and run (release — recommended for gameplay)
cargo run --release

# Run individual checks
cargo check           # Compile check (0 errors expected)
cargo clippy          # Lint check (0 errors, ~36 warnings)
cargo fmt --check     # Format check
```

## Project Structure

```
socom/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/               # Pure data types (zero Bevy dep)
│   ├── input/              # leafwing input bindings
│   ├── rendering/          # Camera rig + post-processing
│   ├── audio/              # bevy_audio + kira standalone host
│   └── game/               # Binary crate — everything gameplay
├── assets/                 # Game assets (audio, models, textures)
│   └── audio/
│       ├── ambient_test.ogg
│       ├── weapon_m4.ogg
│       ├── weapon_1911.ogg
│       └── ...
├── docs/
│   └── wiki/               # This documentation
└── audio/                  # Audio production files
```

## Controls

### Movement & Camera

| Input | Action |
|-------|--------|
| WASD | Camera-relative movement |
| Mouse | Look around |
| Shift | Sprint / Hold breath (while ADS) |
| C | Crouch toggle |
| Z | Prone toggle |
| Space | Jump / Detonate FPV drone |
| V / Middle Mouse | Toggle 1st/3rd person |

### Combat

| Input | Action |
|-------|--------|
| Left Click | Fire weapon |
| Right Click | Aim down sights |
| R | Reload |
| F | Melee attack (knife) |

### Equipment

| Input | Action |
|-------|--------|
| G | Use/throw selected equipment |
| X | Cycle equipment slot |
| 1 | Swap to primary weapon |
| 2 | Swap to sidearm |
| Q | Swap camera shoulder |

### Drones

| Input | Action |
|-------|--------|
| U | Deploy/recall Recon Drone |
| J | Deploy/recall FPV Strike Drone |
| H | Deploy/recall Grenade Drone |
| N | Deploy/recall Mine Drone |

### Tactical

| Input | Action |
|-------|--------|
| Tab | Command wheel |
| E | Interact / Revive teammate |

### System

| Input | Action |
|-------|--------|
| Escape | Pause / Menu |
| Backtick | Developer console |
| F5 | Quick save |
| F9 | Quick load |
| F1 | Entity inspector (bevy-inspector-egui) |

## Build Commands

### Check individual crates
```bash
# Core crate (no Bevy deps)
cargo check -p socom-core

# Input crate
cargo check -p socom-input

# Rendering crate
cargo check -p socom-rendering

# Audio crate
cargo check -p socom-audio

# Game crate (full build)
cargo check -p socom-game
```

### Verify Bevy dependency isolation
```bash
cargo tree -p socom-core
# Should show only serde + glam — no bevy*
```

### Release build for performance testing
```bash
cargo build --release
./target/release/socom-game
```

## Development Commands

### Run with logging
```bash
# Game debug logs
RUST_LOG=socom=debug cargo run

# Minimal output
RUST_LOG=error cargo run

# All logs
RUST_LOG=debug cargo run
```

### Run clippy
```bash
cargo clippy -- -W clippy::pedantic  # Full pedantic
cargo clippy                          # Default lint set
```

## Current Phase

**Phase 3 — Asset Integration & Polish** (active)

Completed:
- ✅ VFX system (bevy_hanabi): 5 effects
- ✅ Post-processing: ACES + Bloom + SSAO + DoF
- ✅ Weapon Audio Plugin relay scaffold
- ✅ Kira Audio Host: 4-bus hierarchy
- ✅ Destruction System: 6 files, 971 lines

In Progress:
- ☐ Placeholder audio assets (.ogg files)
- ☐ Character/weapon models
- ☐ Level art

## Key Architecture Rules

1. **`core` must never depend on Bevy** — verified by `cargo tree -p socom-core`
2. **Messages, not Events** — Use `MessageReader`/`MessageWriter`, register with `app.add_message::<T>()`
3. **Mouse look** uses `Res<AccumulatedMouseMotion>` (not `EventReader<MouseMotion>`)
4. **Audio** uses `(AudioPlayer(handle), PlaybackSettings::ONCE)` pattern
5. **UI** uses `Text + TextFont + TextColor + Node` (not `TextBundle`/`TextStyle`/`Style`)
6. **Despawn** always via `commands.entity(e).despawn()` (always recursive)
7. **Camera FOV** accessed via `&mut bevy::camera::Projection`, match on `Perspective`

## Troubleshooting

### Build Errors
```bash
# Clear cache if dependencies get corrupted
cargo clean
cargo build

# Check Rust version
rustup show

# Ensure correct toolchain
rustup toolchain install nightly-2025-03-01
```

### Missing Audio Files
The game gracefully handles missing audio files:
- bevy_audio: skips silently
- kira: logs a warning via `warn!()`

### Performance
- Debug builds are slow (no optimizations). Use `--release` for actual gameplay.
- 120 Hz fixed timestep for physics
- Post-processing stack is performance-intensive; disable in debug by commenting out
