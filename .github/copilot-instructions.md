# SOCOM Tactical Shooter — Repo Instructions

## Project Overview

A modern reimagining of the classic SOCOM: U.S. Navy SEALs tactical shooter, built in **Rust** with the **Bevy 0.18.1** game engine. Workspace of **5 crates, ~68 source files**. Pre-alpha — all core gameplay systems are implemented, placeholder audio/assets remain.

## Tech Stack (versions locked)

| Component | Version | Notes |
|-----------|---------|-------|
| Rust | **stable** (1.85+) | `rust-toolchain.toml`, target `x86_64-pc-windows-msvc` |
| Bevy | **0.18.1** | NOT 0.19 RC. Pinned across ALL crates. |
| Avian3d | **0.6.1** | XPBD physics, move-and-slide character controller |
| leafwing-input-manager | **0.20.x** | Action-based input, `Actionlike` trait |
| serde + ron | 1.x / 0.12 | Serialization for saves, configs |
| glam | **0.29** | Must match bevy 0.18.1 |
| bevy_hanabi | 0.18.0 | GPU particles (wired, not configured yet) |
| bevy_replicon + lightyear | 0.41.0-rc.1 / 0.26.4 | Networking (Phase 3, deps only) |

## Crate Dependency Graph

```
game (binary) — crates/game/
  ├── core (path)      → Pure data types. ZERO Bevy dependency.
  ├── input (path)     → leafwing-input-manager bindings (PlayerAction enum, 15 variants)
  ├── rendering (path) → 1st/3rd person camera rig with ADS zoom, shoulder swap, freelook
  └── audio (path)     → bevy_audio footstep/ambient loops
```

**Cardinal rule:** `core` must never depend on Bevy. Other crates depend on `core` with `features = ["bevy"]` to derive `Component` on its types.

## Build & Validate

Always use these commands in this order. `cargo watch` is installed (`cargo install cargo-watch`).

```bash
cargo check          # Fast compile check — 0 errors expected across all 5 crates
cargo clippy         # Linter — clean (expect ~78 dead_code warnings, those are expected)
cargo fmt            # Format — consistent across all files
cargo build          # Full build
cargo run            # Run the game binary
cargo watch -x check -x clippy   # Watch mode for iterative work
```

All packages from `cargo add` / `cargo rm` (cargo-edit installed). All external deps in `Cargo.toml` must be wired into the app, not just declared.

## Project Layout

```
SOCOM/
├── Cargo.toml                     # Workspace root, resolver 2, 5 members
├── rust-toolchain.toml            # stable, x86_64-pc-windows-msvc
├── AGENTS.md                      # Full agent onboarding — module map, build status, Bevy 0.18.1 quirks
├── .github/copilot-instructions.md  # THIS FILE
├── assets/audio/                  # Placeholder .ogg files needed
├── docs/                          # Specs, wiki, architecture diagrams, module docs
├── crates/
│   ├── core/src/                  # 3 files: components.rs, resources.rs, lib.rs
│   ├── input/src/                 # 3 files: actions.rs, bindings.rs, lib.rs
│   ├── rendering/src/             # 2 files: camera.rs, lib.rs
│   ├── audio/src/                 # 3 files: footsteps.rs, ambient.rs, lib.rs
│   └── game/src/                  # 10 root files + 16 module directories
│       ├── main.rs, player.rs, level.rs, messages.rs, pause.rs,
│       ├── console.rs, camera_control.rs, settings.rs, settings_applier.rs, save_load.rs
│       ├── ai/          (3 files)  # Enemy FSM, Teammate AI
│       ├── ammo_type/   (1 file)   # AmmoType enum + LoadedAmmo component
│       ├── breathing/   (1 file)   # Breathing + hold-breath system
│       ├── combat/      (8 files)  # shooting, damage, death, reload, weapon_bob, etc.
│       ├── controls/    (3 files)  # Stance transitions, turn rate
│       ├── drones/      (1 file)   # Recon UAV + FPV Strike Drone
│       ├── feedback/    (4 files)  # hit_marker, vignette, enemy_fx
│       ├── gear/        (5 files)  # items, inventory, attachments, workshop
│       ├── hud/         (9 files)  # elements, xp_notification, stamina_bar, kill_feed, etc.
│       ├── menu/        (3 files)  # settings, keybinds
│       ├── missions/    (1 file)   # MissionState, objective tracking
│       ├── physics/     (5 files)  # player_movement, enemy_movement, stance, layers
│       ├── progression/ (5 files)  # xp, stats, achievements, specializations
│       ├── squad/       (3 files)  # orders, formation
│       ├── stamina/     (1 file)   # Stamina drain/regen + sway/spread
│       ├── states/      (4 files)  # AppState: MainMenu → Loading → InGame
│       ├── tactical/    (3 files)  # command_wheel, cover, suppression
│       ├── weapon_handling/ (1)    # WeaponWeight + handling stats
│       └── weapons/     (8 files)  # chassis, caliber, barrel, sight, underbarrel, magazine, stock
└── target/                        # Build artifacts (gitignored)
```

## Architecture Patterns

- **ECS (Entity Component System):** All gameplay is built on Bevy ECS. Systems query components, resources, and messages.
- **Messages not Events:** Use `#[derive(Message)]` + `app.add_message::<T>()` for cross-system communication (14 system messages defined: DamageMessage, DeathMessage, XpGainedMessage, etc.)
- **State Machine:** `AppState` enum: `MainMenu → Loading → InGame ↔ (Escape)`
- **No Events pattern:** Never use Bevy `Event`/`EventReader`/`EventWriter` — the project uses the `Message` pattern throughout.
- **Single responsibility:** Max ~200 lines per file, keep modular.

## Bevy 0.18.1 Quirks (important)

- **Mouse look:** Use `Res<AccumulatedMouseMotion>`, not `EventReader<MouseMotion>`
- **Audio:** `(AudioPlayer(handle), PlaybackSettings::ONCE)`, not `Res<Audio>.play()`
- **UI:** `Text + TextFont + TextColor + Node`, NOT `TextBundle`/`TextStyle`/`Style`
- **Despawn:** `commands.entity(e).despawn()` (always recursive)
- **Projection:** `&mut bevy::camera::Projection`, match on `Perspective(ref mut persp)`
- **Fullscreen:** `WindowMode::BorderlessFullscreen(MonitorSelection::Current)`
- **Camera3d FOV:** On `Projection::Perspective(PerspectiveProjection)`, NOT on `Camera3d`

## Controls (full game)

| Input | Action |
|-------|--------|
| WASD | Camera-relative movement |
| Mouse | Look (pitch/yaw) |
| Left Click | Fire |
| Right Click | ADS |
| Shift | Sprint / Hold breath (while ADS) |
| C / Z | Crouch / Prone toggle |
| Space | Jump / Detonate FPV drone |
| R | Reload |
| Q | Shoulder swap |
| E | Interact |
| Tab | Command wheel |
| V / MMB | Toggle 1st/3rd person |
| 1 / 2 | Primary / Sidearm |
| U / J | Deploy/recall Recon / FPV Strike Drone |
| Escape | Pause |
| Backtick | Dev console |
| F5 / F9 | Quick save / load |
| F1 | Inspector (bevy-inspector-egui) |

## Key Resources

- **AGENTS.md** at root has the complete module map, build status, and all system messages — read it first for full onboard context.
- **docs/** contains specs, architecture, sequences, class diagrams, and per-module documentation.
- If you get a "No matches found" error from grep, set `includeIgnoredFiles` to **true** — the search tool may ignore files by default.

## File Modularity Rules

- Keep files under ~200 lines, single responsibility
- Modules use `mod.rs` pattern (e.g. `combat/mod.rs` re-exports all sub-modules)
- All message types are in `crates/game/src/messages.rs`
- Root game systems are in individual files at `crates/game/src/` (player.rs, console.rs, etc.)