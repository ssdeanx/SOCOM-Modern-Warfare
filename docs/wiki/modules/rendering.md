# Module: `rendering`

**Path:** `crates/rendering/`

Provides the third-person camera system — a chase camera that orbits a target entity in response to mouse movement, with smooth lerp interpolation.

## Responsibilities

- Define the `ThirdPersonCamera` component with all camera parameters
- `camera_follow_system` — lerps the camera position toward a computed shoulder offset behind the target
- `camera_look_system` — reads `AccumulatedMouseMotion` and updates pitch/yaw on the camera

## Key Files

- [`camera.rs`](../../crates/rendering/src/camera.rs) — Component + systems + plugin
- [`lib.rs`](../../crates/rendering/src/lib.rs) — Crate root

## Public API

### `ThirdPersonCamera` Component

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `target` | `Entity` | (required) | Player entity to track |
| `distance` | `f32` | 4.0 | Orbit radius in meters |
| `pitch` | `f32` | 0.35 rad (~20°) | Vertical angle above horizontal |
| `yaw` | `f32` | 0.0 | Horizontal angle |
| `min_pitch` | `f32` | -0.52 rad (~-30°) | Clamp: don't go below ground |
| `max_pitch` | `f32` | 1.4 rad (~80°) | Clamp: don't flip over |
| `shoulder` | `Shoulder` | `Right` | Which shoulder the camera sits over |
| `collision` | `bool` | `true` | Obstruct camera if something is in the way (Phase 1) |
| `fov` | `f32` | 70.0° | Field of view in degrees |
| `lerp_factor` | `f32` | 0.1 | Smoothing (0.0 = instant, 1.0 = no movement) |
| `desired_position` | `Vec3` | ZERO | Computed each frame for lerp target |

### `CameraPlugin`

Registers two systems in `Update`:
- `camera_follow_system` (priority: runs after look)
- `camera_look_system`

### System: `camera_follow_system`

For each `ThirdPersonCamera`, computes the desired world-space position:
```
desired = target_position + shoulder_offset + orbit_offset
```
Where `orbit_offset` is a spherical coordinate offset using `distance * pitch/yaw` spherical→cartesian conversion. Camera `Transform::translation` lerps toward `desired_position` each frame.

### System: `camera_look_system`

Reads `AccumulatedMouseMotion.delta` (Bevy 0.18 replacement for `EventReader<MouseMotion>`). Applies `delta.x` to `cam.yaw` and `delta.y` to `cam.pitch`, clamped to `[min_pitch, max_pitch]`. Scales by `GameSettings.sensitivity` if present.

## Dependencies

- **Used by:** `socom-game` (spawns camera entity with `ThirdPersonCamera`)
- **Uses:** `socom-core` (with `"bevy"` feature for `Shoulder` component), `bevy`

## Notable Patterns / Gotchas

- **Bevy 0.18 mouse API:** `AccumulatedMouseMotion` is a resource, not an event. Read `Res<AccumulatedMouseMotion>.delta` once per frame; do NOT call `.clear()` — it auto-accumulates.
- The shoulder offset is hardcoded at ±0.5 units on X. No smooth shoulder-swap transition yet (Phase 1).
- Camera collision raycasting is flagged (`cam.collision`) but not implemented. Phase 1 will add a raycast from target to desired position.
