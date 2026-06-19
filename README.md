# SOCOM Tactical Shooter

A modern reimagining of the classic PS2 tactical shooter **SOCOM: U.S. Navy SEALs** (and SOCOM II), built in Rust with the [Bevy](https://bevyengine.org/) game engine.

> **Phase 0 — Foundation.** This is a pre-alpha scaffold. The engine, physics, input, camera, and audio pipelines are wired. No gameplay, no AI, no weapons, no assets yet.

---

## Quick Start

```bash
# Build and run
cargo run

# Verify everything compiles
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

Requires: Rust 1.85+ stable (see `rust-toolchain.toml`), x86_64-pc-windows-msvc target.

## What's Here

```ascii
SOCOM/
├── crates/
│   ├── core/         # Data types (zero Bevy dependency)
│   ├── input/        # Keyboard + gamepad input bindings
│   ├── rendering/    # 3rd-person chase camera
│   ├── audio/        # Footstep + ambient audio
│   └── game/         # Binary — states, physics, player, level
├── .omo/
│   ├── drafts/       # Design specs
│   └── plans/        # Implementation plans
├── AGENTS.md         # AI agent conventions
├── CHANGELOG.md
└── README.md
```

### Crate dependency graph

```
game (binary)
  ├── core         no Bevy → pure data types
  ├── input        leafwing-input-manager actions
  ├── rendering    custom 3rd-person camera
  └── audio        bevy_audio footstep + ambient
```

## Controls (Phase 0)

| Input | Action |
|-------|--------|
| WASD | Move |
| Mouse | Camera look |
| Shift | Sprint toggle |
| C | Crouch toggle |
| Z | Prone toggle |
| Space | Jump (placeholder) / Menu start |
| Escape | Pause |

## Tech Stack

| Component | Choice | Why |
|-----------|--------|-----|
| **Engine** | Bevy 0.18.1 | Stable, native ECS, excellent DX |
| **Physics** | Avian3d 0.6.1 | Native ECS, XPBD solver, move-and-slide |
| **Input** | leafwing-input-manager 0.20 | Action-based, network-safe, gamepad support |
| **Serialization** | serde + glam | Save games, config, network messages |
| **Audio** | bevy_audio (built-in) | Phase 0 placeholder; spatial audio in Phase 2 |

## Phase Plan

- **Phase 0 (current):** Workspace, engine loop, physics, input, camera, audio, greybox level. Single player prototype.
- **Phase 1:** Squad system (3-4 AI teammates), command wheel, weapon system, cover mechanics.
- **Phase 2:** Full level geometry (Blender), spatial audio, HUD/UI, mission scripting.
- **Phase 3:** Networking (lightyear), co-op, competitive modes.

See `.omo/plans/phase-0-foundation.md` for the detailed Phase 0 task breakdown.

## Developer Tools

Installed in this project:

```bash
cargo outdated       # Check dependency freshness
cargo audit          # Security audit
cargo watch -x check # Auto-rebuild on changes
cargo expand         # Macro expansion debugging
typos                # Spelling check
```

## License

MIT — (placeholder, update when decided)
