# Getting Started

## Prerequisites

- **Rust toolchain** — stable channel, pinned in `rust-toolchain.toml`
- **Target** — `x86_64-pc-windows-msvc` (Windows 10/11)
- **Disk space** — ~5 GB for debug build artifacts (Bevy + deps)

## Installation

```bash
# Clone and enter the repo
git clone <repo-url> SOCOM
cd SOCOM

# Verify Rust channel (reads rust-toolchain.toml)
rustup show

# Build and check
cargo check

# Run linter
cargo clippy
```

## First Run

```bash
# Build and launch
cargo run
```

Expected: A 1280×720 window appears with "SOCOM Tactical Shooter" title. You'll see a main menu with title text. Press **Space** → 0.5s loading screen → in-game 3D scene with a greybox room (walls, pillars, ramp, stairs). WASD moves the player, mouse orbits the camera, Shift/C/Z toggle sprint/crouch/prone.

**Note:** No audio assets exist yet, so `AmbientPlugin` will log a missing-asset warning (harmless).

## Common Workflows

### Build and iterate quickly

```bash
cargo watch -x check -x clippy
```

### Check for outdated dependencies

```bash
cargo outdated -R
```

### Security audit

```bash
cargo audit
```

### Spell-check docs

```bash
typos
```

### Expand macros for debugging

```bash
cargo expand -p socom-core
```

### Generate a fresh build

```bash
cargo clean && cargo check
```

## Project Layout

```
SOCOM/
├── Cargo.toml            # Workspace manifest (5 member crates)
├── rust-toolchain.toml   # Channel + target pinning
├── .cargo/config.toml    # dev profile opt-level=1
├── crates/
│   ├── core/             # Pure data types — no Bevy
│   ├── input/            # leafwing input bindings
│   ├── rendering/        # ThirdPersonCamera plugin
│   ├── audio/            # Footstep + ambient audio
│   └── game/             # Binary — states, physics, player, level
├── assets/               # Runtime assets (empty in Phase 0)
├── docs/
│   ├── wiki/             # Auto-generated wiki
│   └── Specs/            # PRD, design, tasks
├── AGENTS.md             # AI agent conventions and context
├── CHANGELOG.md
└── README.md
```

## Configuration

No runtime configuration files yet. The `.cargo/config.toml` sets `[profile.dev] opt-level = 1` for faster iteration during development (skips some LLVM optimization passes).

## Where to Go Next

- Architecture overview: [architecture.md](architecture.md)
- Module reference: [README.md](README.md)
- Phase 0 plan: `../../.omo/plans/phase-0-foundation.md`
