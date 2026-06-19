# SOCOM Tactical Shooter — Wiki

> **Modern reimagining of SOCOM: U.S. Navy SEALs (PS2)** built in Rust with Bevy 0.18.
> Generated from source at commit SHA: `uncommitted`

## Key Concepts

- **Tactical Third-Person Shooter** — Squad-based military shooter with a third-person chase camera, realistic movement stances (sprint, crouch, prone), and cover mechanics.
- **ECS Architecture** — Built on Bevy's Entity Component System. Every gameplay element (player, camera, physics body) is a bundle of components processed by systems.
- **Modular Crate Layout** — Five crates in a Cargo workspace enforce clean separation: `core` (no Bevy dependency), `input`, `rendering`, `audio`, and `game` (binary).

## Entry Points

- [`crates/game/src/main.rs`](../../crates/game/src/main.rs) — App entry. Composes all plugins, configures window, launches the game loop.

## High-Level Architecture

The engine loop runs in `main.rs`. Plugins are composed in a layered stack: Bevy `DefaultPlugins` → Avian `PhysicsPlugins` → internal crates (`InputPlugin`, `CameraPlugin`, `AudioPlugin`) → game logic (`MainMenuPlugin`, `LoadingPlugin`, `InGamePlugin`). A state machine drives the menu → loading → in-game transition.

See [architecture.md](architecture.md).

## Module Map

| Module | Path | Purpose |
|--------|------|---------|
| [`core`](modules/core.md) | `crates/core/` | Pure data types. Zero Bevy dependency. Conditional `bevy` feature for ECS components. |
| [`input`](modules/input.md) | `crates/input/` | leafwing-input-manager bindings. `PlayerAction` enum, keyboard + gamepad input maps. |
| [`rendering`](modules/rendering.md) | `crates/rendering/` | `ThirdPersonCamera` component and `CameraPlugin` with follow + mouse-look systems. |
| [`audio`](modules/audio.md) | `crates/audio/` | Footstep timing and ambient loop plugins. Uses Bevy's built-in `AudioPlayer` API. |
| [`game`](modules/game.md) | `crates/game/` | Binary crate. State machine, player bundle, physics movement, procedural test level. |

## Diagrams

- [Architecture Diagram](diagrams/architecture.md) — Crate dependency graph, component data flow, entity hierarchy
- [Sequence Diagrams](diagrams/sequences.md) — Frame lifecycle, state transitions, input → movement pipeline

## Getting Started

See [getting-started.md](getting-started.md).
