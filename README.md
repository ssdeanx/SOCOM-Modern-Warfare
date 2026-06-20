# SOCOM Tactical Shooter

A modern reimagining of the classic PS2 tactical shooter rebuilt for PC with AAA production values using Rust and the [Bevy](https://bevyengine.org/) game engine. Targets authentic squad-based tactical combat inspired by Arma 3, Squad, and Ghost Recon.

> **Phase 3 Active — Asset Integration & Polish.** 100 source files across 5 crates. Zero compilation errors. All core gameplay systems implemented — VFX, post-processing, kira audio host, destruction system, equipment, healing, drones, modular weapons, AI, progression, HUD, save/load.

---

## Quick Start

```bash
# Build and run (requires ~10GB free disk space for debug build)
cargo run --release

# Verify everything compiles (fast incremental)
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt

# Run with logging
RUST_LOG="socom=debug,warn" cargo run
```

Requires: Rust 1.85+ stable (see `rust-toolchain.toml`), x86_64-pc-windows-msvc target.

## Project Structure

```
SOCOM/
├── crates/
│   ├── core/          (3 files) — Pure data types, ZERO Bevy dependency
│   ├── input/         (3 files) — leafwing-input-manager bindings (17 actions)
│   ├── rendering/     (2 files) — 1st/3rd person camera rig + post-processing
│   ├── audio/         (4 files) — bevy_audio + kira host plugin (bus hierarchy)
│   └── game/          (88+ files) — Binary: everything gameplay
├── docs/
│   ├── Specs/         — requirements.md, design.md, tasks.md (versioned)
│   └── wiki/          — code-wiki generated documentation
├── audio/             — Audio asset directory (placeholder .ogg paths)
├── AGENTS.md          — AI agent conventions & project context
├── CHANGELOG.md       — Release history
└── README.md
```

### Crate dependency graph

```
game (binary — 88+ files)
  ├── core         no Bevy → pure data types (Player, Health, Weapon, Team)
  ├── input        leafwing-input-manager → 17 action variants
  ├── rendering    custom camera rig + post-processing (ACES, Bloom, SSAO, DoF)
  └── audio        bevy_audio + kira host (4-bus hierarchy)
```

### Game Module Map (16 directories in game crate)

| Directory | Purpose |
|-----------|---------|
| `combat/` | Shooting, damage, death, reload, weapon bob/model/state, VFX, destruction |
| `weapons/` | 8-file modular system: chassis, caliber, barrel, sight, underbarrel, magazine, stock |
| `ai/` | Enemy FSM (Patrol→Alert→Engage), teammate follow/engage |
| `squad/` | Orders (Move/Engage/Suppress/Regroup), formation positioning |
| `tactical/` | Command wheel, cover detection, suppression system |
| `drones/` | Recon UAV, FPV Strike, Grenade Drone, Mine Drone |
| `gear/` | Items, inventory, attachments, workshop, equipment, healing, melee, throwables, deployables |
| `physics/` | Player/enemy movement, stance, collision layers |
| `hud/` | Health, stamina, ammo, crosshair, XP, achievements, kill feed, squad status |
| `progression/` | XP/leveling, stats, 7 achievements, 4 specializations |
| `states/` | AppState machine: MainMenu→Loading→InGame |
| `controls/` | Stance transitions, turn rate limiting |
| `menu/` | Settings, keybinds |
| `missions/` | 5 objective types (EliminateAll, ReachLocation, etc.) |
| `breathing/` | Hold-breath weapon steadying |
| `stamina/` | Drain/regen, exhaustion penalties |
| `weapon_handling/` | Weight classes (Light/Medium/Heavy/Sniper) |

## Controls

| Input | Action |
|-------|--------|
| WASD | Camera-relative movement |
| Mouse | Camera look (pitch/yaw) |
| Left Click | Fire weapon |
| Right Click | Aim down sights |
| Shift | Sprint / Hold breath (while ADS) |
| C | Crouch toggle |
| Z | Prone toggle |
| Space | Jump / Detonate FPV drone |
| R | Reload |
| Q | Shoulder swap (left/right) |
| E | Interact / Revive teammate |
| G | Use/throw selected equipment |
| X | Cycle equipment slot |
| F | Melee attack (knife) |
| Tab | Command wheel |
| V / MMB | Toggle 1st/3rd person |
| 1 / 2 | Primary / Sidearm |
| U | Deploy/recall Recon Drone |
| J | Deploy/recall FPV Strike Drone |
| H | Deploy/recall Grenade Drone |
| N | Deploy/recall Mine Drone |
| Escape | Pause / Menu |
| Backtick | Dev console (help, god, noclip, killall, timescale, tp) |
| F5 | Quick save |
| F9 | Quick load |
| F1 | Inspector (bevy-inspector-egui) |

## Tech Stack

| Component | Choice | Version | Purpose |
|-----------|--------|---------|---------|
| **Engine** | Bevy | **0.18.1** | Stable ECS, renderer, asset pipeline |
| **Physics** | Avian3d | **0.6.1** | XPBD solver, move-and-slide @ 120Hz |
| **Input** | leafwing-input-manager | 0.20 | Action-based keyboard + gamepad |
| **VFX** | bevy_hanabi | 0.18.0 | GPU particles (5 effects wired) |
| **Audio** | kira + bevy_audio | 0.12 / built-in | Professional bus hierarchy + placeholder |
| **Serialization** | serde + ron | 1.x / 0.12 | Save/load, config, level files |
| **Networking** | bevy_replicon + lightyear | 0.41 / 0.26 | Phase 6 ready |
| **Debug** | bevy-inspector-egui | 0.36 | Runtime entity inspector |

## Phase Plan

| Phase | Title | Status | Description |
|-------|-------|--------|-------------|
| 0 | Foundation | ✅ Complete | Workspace, engine, core types, input, camera, greybox level |
| 1 | Core Systems | ✅ Complete | MoveAndSlide, shooting, damage/death, enemy AI, HUD, reload |
| 2 | Advanced Systems | ✅ Complete | 19 subtasks: modular weapons, drones, gear, stamina, VFX, post-processing |
| 3 | Asset Integration | 🟡 Active | Audio (kira), destruction system, placeholder assets |
| 4 | Training & Practice | ⏳ Planned | Target range, weapon locker, aim trainer, squad bots |
| 5 | Competitive Modes | ⏳ Planned | TDM, Demolition, CTF, round system, economy, spectator |
| 6 | Multiplayer | ⏳ Planned | Dedicated servers, netcode, matchmaking, voice chat |
| 7 | Production Polish | ⏳ Planned | Main menu, player profile, video/audio/controls settings, tutorial |

## Build Status

- `cargo check` — **0 errors** across all 5 crates (100+ source files)
- `cargo clippy` — 0 errors
- `cargo tree -p socom-core` — confirms **zero Bevy dependencies** in core crate
- 36 remaining `dead_code` warnings (weapon attachment system awaiting CompleteWeapon integration)

## Development Philosophy

- **Message-driven architecture** — 18 system messages for inter-system communication (not Bevy Events)
- **Zero Bevy dependency** in core crate — all data types are pure Rust with serde
- **Single-responsibility files** — max ~200 lines per file, modular by design
- **Enterprise-grade structure** — workspace with 5 focused crates, clear dependency direction
- **Systems built before assets** — all gameplay code complete before Blender models

## Developer Tools

```bash
cargo check        # Fast compilation check
cargo clippy       # Linting with best practices
cargo fmt          # Consistent formatting
cargo test         # Run Rust tests
cargo outdated     # Check dependency freshness
cargo audit        # Security audit
cargo watch -x check  # Auto-rebuild on changes
typos              # Spelling check
```

## License

MIT — (placeholder, update when decided)
