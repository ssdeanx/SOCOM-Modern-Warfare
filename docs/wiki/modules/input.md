# Module: `input`

**Path:** `crates/input/`

Wraps `leafwing-input-manager` v0.20 to provide action-based input for the SOCOM game. Translates raw keyboard/gamepad events into semantic `PlayerAction` enums.

## Responsibilities

- Define the `PlayerAction` enum (all game actions)
- Build the default keyboard + gamepad input map
- Register an `InputPlugin` that registers `PlayerAction` as `Actionlike`

## Key Files

- [`actions.rs`](../../crates/input/src/actions.rs) — `PlayerAction` enum definition
- [`bindings.rs`](../../crates/input/src/bindings.rs) — `default_input_map()` factory function
- [`lib.rs`](../../crates/input/src/lib.rs) — `InputPlugin` plugin definition

## Public API

### `PlayerAction` enum

| Variant | Description | Keyboard Binding | Gamepad Binding |
|---------|-------------|-----------------|-----------------|
| `Move` | WASD / left stick dual-axis | `VirtualDPad::wasd()` | `GamepadStick::LEFT` |
| `Look` | Mouse / right stick dual-axis | `MouseMove::default()` | `GamepadStick::RIGHT` |
| `Sprint` | Toggle sprint | `ShiftLeft` | `LeftThumb` |
| `Crouch` | Toggle crouch | `KeyC` | `South` (A on Xbox) |
| `Prone` | Toggle prone | `KeyZ` | `DPadDown` |
| `Jump` | Jump (placeholder) | `Space` | `East` (B on Xbox) |
| `Interact` | Use/interact | `KeyE` | `North` (Y on Xbox) |
| `Pause` | Pause / menu | `Escape` | `Start` |

### `default_input_map() -> InputMap<PlayerAction>`

Returns a fully populated `InputMap<PlayerAction>` with all keyboard + gamepad bindings above. Uses:
- `input_map.insert_dual_axis(...)` for 2D analog inputs (Move, Look)
- `input_map.insert(...)` for discrete button inputs

### `InputPlugin`

```rust
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.insert_resource(default_input_map());
        app.add_systems(Update, register_with_input_map);
    }
}
```

Registers the `InputManagerPlugin` for `PlayerAction` and inserts the default input map as a resource.

## Dependencies

- **Used by:** `socom-game` (physics systems query `ActionState<PlayerAction>`)
- **Uses:** `socom-core` (with `"bevy"` feature), `leafwing-input-manager`, `bevy`

## Notable Patterns / Gotchas

- leafwing-input-manager v0.20 API uses `insert_dual_axis()` for 2D inputs, not `bind_dual_axis()` from earlier versions.
- `MouseMove` is imported from `leafwing_input_manager::user_input::mouse::MouseMove`, not from bevy.
- `GamepadStick::LEFT` / `GamepadStick::RIGHT` are imported from `leafwing_input_manager::user_input::gamepad`.
- The input map is a `Resource` — it can be replaced at runtime for customizable controls (Phase 1).
