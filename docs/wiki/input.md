# Input Crate (`socom-input`)

**Path:** `crates/input/`  
**Files:** `lib.rs`, `actions.rs`, `bindings.rs`  
**Dependencies:** `bevy 0.18.1`, `leafwing-input-manager 0.20`, `core`  
**Purpose:** Player input abstraction via leafwing-input-manager

## Architecture

The input crate wraps `leafwing-input-manager` to provide a declarative input system. It defines a `PlayerAction` enum with 17 variants and constructs a default `InputMap<PlayerAction>` with both keyboard and gamepad bindings.

```
socom-input (lib)
├── actions.rs  — PlayerAction enum
├── bindings.rs — default_input_map()
└── lib.rs      — InputPlugin
```

## PlayerAction (17 variants)

| Action | Keyboard | Gamepad | Description |
|--------|----------|---------|-------------|
| `Move` | WASD | Left Stick | Movement dual-axis |
| `Look` | Mouse | Right Stick | Camera look dual-axis |
| `Sprint` | ShiftLeft | LeftThumb | Sprint / hold breath |
| `Crouch` | C | South (B) | Toggle crouch |
| `Prone` | Z | DPadDown | Toggle prone |
| `Jump` | Space | East (A) | Jump |
| `Interact` | E | North (Y) | Interact / revive |
| `Pause` | Escape | Start | Pause / menu |
| `Fire` | Left Mouse | RightTrigger | Shoot weapon |
| `Aim` | Right Mouse | LeftTrigger | Aim down sights |
| `Reload` | R | West (X) | Reload weapon |
| `SwapPrimary` | 1 | DPadUp | Switch to primary |
| `SwapSidearm` | 2 | DPadRight | Switch to sidearm |
| `ShoulderSwap` | Q | LeftTrigger2 | Swap camera shoulder |
| `CycleEquipment` | X | DPadLeft | Cycle equipment slot |
| `UseEquipment` | G | RightTrigger2 | Use/throw equipment |
| `Melee` | F | RightThumb | Knife attack |

## InputPlugin

Registers the leafwing `InputManagerPlugin<PlayerAction>` and inserts the default `InputMap<PlayerAction>` as a resource. This allows any system to query `ActionState<PlayerAction>` on the player entity.

## Default Input Map

Created by `default_input_map()` which configures:
- **Dual-axis:** Movement (WASD + left stick), Look (mouse + right stick)
- **Keyboard bindings:** All 17 actions mapped to keys
- **Gamepad bindings:** All 17 actions mapped to controller buttons/sticks

The input map is inserted as a resource and can be modified at runtime for rebinding.
