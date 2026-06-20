# Core Crate (`socom-core`)

**Path:** `crates/core/`  
**Files:** `lib.rs`, `components.rs`, `resources.rs`  
**Dependencies:** `serde`, `glam` (zero Bevy dependency)  
**Purpose:** Pure data types shared across all crates. Optional `bevy` feature derives `Component`/`Resource`.

## Architecture

The core crate is the **foundation layer** of the SOCOM architecture. It defines all fundamental game data types that are used by every other crate. By keeping Bevy out of the dependency tree, these types remain serializable with `serde` and usable in non-Bevy contexts (e.g., networking layer, save files, editor tools).

```
socom-core (lib)
├── components.rs — ECS component data types
└── resources.rs — Global resources
```

## Component Types (`components.rs`)

### `Player`
Marker component for the player entity. Zero-sized type used to identify the player in queries.

```rust
#[derive(Component)]
pub struct Player;
```

### `Health`
Health pool with armor, bleed-out, and revive support.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `current` | `f32` | 100.0 | Current hit points |
| `max` | `f32` | 100.0 | Maximum hit points |
| `armor` | `f32` | 0.0 | Armor points absorbing damage |
| `max_armor` | `f32` | 100.0 | Maximum armor capacity |
| `is_downed` | `bool` | false | Bleed-out / downed state |
| `bleed_out_remaining` | `f32` | 30.0 | Seconds before final death |
| `revive_progress` | `f32` | 0.0 | Progress towards revival |

Key methods: `is_alive()`, `is_down()`, `ratio()`, `armor_ratio()`

### `MovementState`
Enum representing the player's current stance.

```rust
pub enum MovementState { Standing, Sprinting, Crouching, Prone, InCover }
```

### `Team`
Team affiliation for entity identification.

```rust
pub enum Team { Player, Teammate, Enemy }
```

### `Weapon`
Static weapon configuration data. Includes factory methods: `m4a1()`, `mp5sd()`, `m1911()`, `ak47()`.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Human-readable name |
| `fire_rate` | `f32` | Rounds per second |
| `damage` | `f32` | Base damage per bullet |
| `magazine_size` | `u32` | Magazine capacity |
| `reserve_ammo` | `u32` | Total spare rounds |
| `reload_time` | `f32` | Reload duration (seconds) |
| `is_automatic` | `bool` | Full-auto vs semi-auto |
| `spread_degrees` | `f32` | Bullet deviation |
| `max_range` | `f32` | Effective range (metres) |

### `WeaponSlot`
Container for primary + sidearm weapon slots.

```rust
pub struct WeaponSlot {
    pub primary: Option<Weapon>,   // M4A1 default
    pub sidearm: Option<Weapon>,   // M1911 default
    pub active_slot: u8,           // 0 = primary, 1 = sidearm
}
```

Key methods: `active_weapon()`, `active_weapon_mut()`, `swap_slot()`

### `Shoulder`
Camera shoulder preference.

```rust
pub enum Shoulder { Right, Left }
```

### `Velocity`
Movement velocity vector component.

```rust
pub struct Velocity(pub glam::Vec3);
```

## Resource Types (`resources.rs`)

### `GameSettings`
Persistent game settings serialized to `~/.socom/settings.ron`.

| Field | Type | Default |
|-------|------|---------|
| `master_volume` | `f32` | 0.8 |
| `sfx_volume` | `f32` | 1.0 |
| `music_volume` | `f32` | 0.5 |
| `sensitivity` | `f32` | 1.0 |
| `invert_y` | `bool` | false |
| `fullscreen` | `bool` | false |
| `vsync` | `bool` | true |

### `InputMapping`
Keyboard binding profile. Defaults to WASD + common FPS keys.

### `Paused`
Global pause state. When `true`, gameplay systems freeze.

```rust
pub struct Paused(pub bool);
```

### `SensitivityMultiplier`
Frame-by-frame sensitivity modifier updated by stance, weapon weight, and stamina.

```rust
pub struct SensitivityMultiplier(pub f32);
```

### `is_not_paused()`
Run condition function used with `.run_if()` on all gameplay systems.
