# Module: `game`

**Path:** `crates/game/`

The binary crate — application entry point, state machine, player spawn, physics systems, and procedural level. This is the crate that ties everything together into a running game.

## Responsibilities

- **`main.rs`** — Compose all plugins (Bevy default, Avian physics, internal crates, game states), configure window, launch loop
- **`states/`** — Three-phase state machine: `MainMenu → Loading → InGame`
- **`player.rs`** — `PlayerBundle` definition + `PlayerPlugin` (spawns player + camera on startup)
- **`physics.rs`** — `player_movement_system` + `player_stance_system`
- **`level.rs`** — `LevelPlugin` that spawns a procedural test room

## State Machine

| State | Systems | Transition |
|-------|---------|------------|
| `MainMenu` | Camera2d, title text, Space key handler | Space → Loading |
| `Loading` | Camera2d, "Loading..." text, 0.5s timer | Timer done → InGame |
| `InGame` | 3D scene, player, physics, camera, level | (Phase 1: Escape → MainMenu) |

## Key Files

- [`main.rs`](../../crates/game/src/main.rs) — Entry point
- [`states/mod.rs`](../../crates/game/src/states/mod.rs) — `AppState` enum
- [`states/main_menu.rs`](../../crates/game/src/states/main_menu.rs) — `MainMenuPlugin`
- [`states/loading.rs`](../../crates/game/src/states/loading.rs) — `LoadingPlugin`
- [`states/ingame.rs`](../../crates/game/src/states/ingame.rs) — `InGamePlugin` (lights + sub-plugins)
- [`player.rs`](../../crates/game/src/player.rs) — `PlayerBundle` + `PlayerPlugin`
- [`physics.rs`](../../crates/game/src/physics.rs) — `PhysicsPlugin` (movement + stance)
- [`level.rs`](../../crates/game/src/level.rs) — `LevelPlugin` (procedural greybox room)

## Player Bundle

```rust
PlayerBundle {
    player: Player,
    health: Health(100.0),
    movement_state: MovementState::Standing,
    weapon_slot: WeaponSlot { primary: None, sidearm: None },
    rigid_body: RigidBody::Dynamic,
    collider: Collider::capsule(0.3, 0.9),
    collision_layers: CollisionLayers::default(),
    transform: Transform::from_xyz(0.0, 1.5, 0.0),
}
```

Spawned at `(0, 1.5, 0)` — 1.5m above ground (capsule height center). A `ThirdPersonCamera` is spawned alongside targeting this entity.

## Movement System

- Reads `ActionState<PlayerAction>::axis_pair(&Move)` → `Vec2`
- Applies speed based on `MovementState`: Walk 3.0, Sprint 5.0, Crouch 1.5, Prone 0.8 m/s
- Sets `LinearVelocity` directly on XZ plane, lerps to zero when no input (decel factor 0.2)
- Stance system toggles via `just_pressed` on Sprint/Crouch/Prone actions

## Procedural Level

Spawned on `InGame` enter. 20m × 20m greybox room with:
- Ground plane (40×40, dark grey)
- Four walls (3m tall, 0.3m thick, brown)
- Four pillars (1m cubes, blue-grey) at compass points
- Ramp (2×4, sloped 30°, green-brown)
- Two stair steps (red-brown) at far end

## Dependencies

- **Uses:** `socom-core`, `socom-input`, `socom-rendering`, `socom-audio`, `bevy` (with 3d/audio features), `avian3d`, `leafwing-input-manager`
- **Used by:** (binary crate, not a library)

## Notable Patterns / Gotchas

- The `PlayerPlugin` runs on `Startup` (not `OnEnter(AppState::InGame)`) — this means it only runs once. Phase 1 needs to move this to `OnEnter` for proper state transitions (return to menu + re-enter game).
- Movement is camera-agnostic in Phase 0 — forward is always +Z world space. Phase 1 factors camera yaw into movement direction.
- The procedural level uses `Startup` as well. Same issue as player — needs `OnEnter(InGame)` for Phase 1.
- FPS overlay is enabled via `bevy::dev_tools::fps_overlay::FpsOverlayPlugin` but requires `bevy_dev_tools` feature on the bevy dependency.
