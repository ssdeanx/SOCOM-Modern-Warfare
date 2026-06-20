# Game Module: `states/` — App State Machine

**Path:** `crates/game/src/states/`  
**Files:** 4 — `mod.rs`, `main_menu.rs`, `loading.rs`, `ingame.rs`  
**Purpose:** Application state management

## AppState

```rust
pub enum AppState { MainMenu, Loading, InGame }
```

### State Transitions
```
MainMenu ──(New Game)──> Loading ──(0.5s)──> InGame
    ↑                                             │
    └────────────────(Escape)─────────────────────┘
```

## State Modules

### `main_menu.rs`
Title screen with:
- New Game button → transitions to Loading
- Settings → settings screen
- Quit → exits application
- Controls display

### `loading.rs`
- Displays loading screen with progress indicator
- 0.5s timer before transition to InGame
- Placeholder for future asset loading

### `ingame.rs`
InGamePlugin that registers all gameplay sub-plugins:
- PlayerPlugin, PhysicsPlugin, LevelPlugin
- CombatPlugin, TacticalPlugin, SquadPlugin, AiPlugin
- HudPlugin, ProgressionPlugin, GearPlugin

Features:
- `IngameEntity` marker component — auto-despawned on state exit
- Light setup (directional + ambient)
- Cursor capture/grab on enter/release on exit

## State Lifecycle

### OnEnter(InGame)
1. `setup_ingame` — Spawn directional light + ambient light
2. `capture_cursor` — Lock cursor for mouse look
3. All sub-plugins fire their OnEnter systems

### OnExit(InGame)
1. `cleanup_ingame` — Despawn all `IngameEntity` tagged entities
2. `release_cursor` — Unlock cursor for menu interaction
