# Game Module: `tactical/` — Tactical Systems

**Path:** `crates/game/src/tactical/`  
**Files:** 4 — `mod.rs`, `command_wheel.rs`, `cover.rs`, `suppression.rs`  
**Purpose:** Command wheel, cover detection, suppression mechanics

## Module Map

```
tactical/
├── mod.rs           — TacticalPlugin
├── command_wheel.rs — CommandWheelState, wheel input + UI systems
├── cover.rs         — InCover component, cover_detection_system
└── suppression.rs   — Suppression component, suppression_system + fx
```

## Command Wheel

Opened with **Tab** key. Radial menu with 4 slots:

| # | Order | Key |
|---|-------|-----|
| 1 | MOVE | 1 |
| 2 | ENGAGE | 2 |
| 3 | SUPPRESS | 3 |
| 4 | REGROUP | 4 |

### CommandWheelState Resource
```rust
pub struct CommandWheelState { pub open: bool, pub selected_index: usize }
```

### Systems
- `command_wheel_input_system` — Tab toggle, number key selection
- `command_wheel_ui_system` — Render radial UI nodes

## Cover System

### InCover Component
```rust
pub struct InCover { pub cover_entity: Entity, pub cover_type: CoverType }
```
`CoverType`: Low, High, CornerLeft, CornerRight

### cover_detection_system
- Casts 4 short raycasts (X, -X, Z, -Z) at 0.6m from player
- If any ray hits geometry, marks player as `InCover`
- Emits `CoverStateMessage` on state change

## Suppression System

### Suppression Component
```rust
pub struct Suppression { pub level: f32, pub decay_timer: Timer }
```

### suppression_system
- Reads `Changed<Health>` on player
- Adds 20 suppression level per damage event
- Emits `SuppressionMessage` when level changes significantly

### suppression_fx_system
- Decays suppression at 30/s after 2s of no damage
- Suppression affects weapon accuracy, screen effects
