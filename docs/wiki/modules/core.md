# Module: `core`

**Path:** `crates/core/`

Core data types for the SOCOM game — pure domain objects with **zero Bevy dependency**. Designed to be shared with non-Bevy consumers (dedicated server, editor, CLI tools).

## Responsibilities

- Define player identity (`Player`, `Health`, `MovementState`, `Team`)
- Define weapon system types (`Weapon`, `WeaponSlot`, `Shoulder`)
- Define persistence structures (`GameSettings`, `InputMapping`)
- Provide serialization via serde on all types
- Optionally derive `bevy_ecs::Component` via the `"bevy"` feature flag

## Key Files

- [`components.rs`](../../crates/core/src/components.rs) — All ECS component types
- [`resources.rs`](../../crates/core/src/resources.rs) — Global resources (settings, input config)
- [`lib.rs`](../../crates/core/src/lib.rs) — Crate root (re-exports)

## Public API

### Components (behind `bevy` feature)

| Type | Description |
|------|-------------|
| `Player` | Marker component. Zero-sized, identifies the player entity. |
| `Health` | `{ current: f32, max: f32 }`. `new(max)`, `is_alive()`, `ratio()`. Default: 100 HP. |
| `MovementState` | Enum: `Standing`, `Sprinting`, `Crouching`, `Prone`, `InCover`. `Copy`, `Default = Standing`. |
| `Team` | Enum: `Player`, `Ally`, `Enemy`, `Civilian`. Marking component for team identity. |
| `Weapon` | `{ name, fire_rate, damage, mag_size, reserve_ammo }`. |
| `WeaponSlot` | `{ primary: Option<Weapon>, sidearm: Option<Weapon> }`. Default = both empty. |
| `Shoulder` | Enum: `Left`, `Right`. Camera shoulder preference. Default: Right. |

### Resources (always available)

| Type | Description |
|------|-------------|
| `GameSettings` | `{ master_volume, sensitivity, invert_y }`. Default: volume 0.8, sens 1.0, invert false. |
| `InputMapping` | String-based keybinding profile. Default: WASD + Shift/C/Z/Space/E/Escape. |

## Internal Structure

No internal structure — all types are flat enums and structs. The `components.rs` file is the primary source; `resources.rs` holds the two resource types. Both are gated only by serde, not Bevy.

Feature-flag pattern used throughout:
```rust
#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Component;

#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
```

## Dependencies

- **Used by:** `socom-input`, `socom-rendering`, `socom-audio`, `socom-game`
- **Uses:** `serde`, `glam` (for `Vec3` in `Velocity`)
- **External (optional):** `bevy_ecs` (behind `"bevy"` feature)

## Notable Patterns / Gotchas

- `Velocity(pub glam::Vec3)` does NOT derive `Component` — it's a pure math type, not an ECS component. The crate uses Bevy's built-in `LinearVelocity` from Avian3d.
- All types implement `Serialize` + `Deserialize` for future save-game and network serialization.
- `InputMapping` uses raw `String` values for key names, not `KeyCode` — avoids the Bevy dependency. Phase 1 will parse these into `KeyCode` at the input layer.
