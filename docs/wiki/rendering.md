# Rendering Crate (`socom-rendering`)

**Path:** `crates/rendering/`  
**Files:** `lib.rs`, `camera.rs`, `post_processing.rs`  
**Dependencies:** `bevy 0.18.1`, `avian3d 0.6.1`, `core`, `input`  
**Purpose:** Camera rig, post-processing pipeline

## Architecture

The rendering crate provides the full camera system (1st/3rd person toggle, shoulder swap, ADS zoom, collision avoidance) and the post-processing stack (ACES tone mapping, Bloom, SSAO, Depth of Field).

```
socom-rendering (lib)
├── camera.rs          — ThirdPersonCamera component + CameraPlugin
└── post_processing.rs — PostProcessingProfile + PostProcessingPlugin
```

## Camera System (`camera.rs`)

### CameraPerspective
```rust
pub enum CameraPerspective { ThirdPerson, FirstPerson }
```

### ThirdPersonCamera Component
Enterprise-grade camera controller supporting ARMA/SQUAD-style behaviour.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `target` | `Entity` | — | Entity to follow (player) |
| `distance` | `f32` | 6.0 | Orbit radius |
| `pitch` | `f32` | 0.45 | Vertical angle (radians) |
| `yaw` | `f32` | 0.0 | Horizontal angle |
| `min_pitch` | `f32` | -0.52 | Pitch lower clamp |
| `max_pitch` | `f32` | 1.4 | Pitch upper clamp |
| `perspective` | `CameraPerspective` | ThirdPerson | Current mode |
| `perspective_factor` | `f32` | 0.0 | Interpolation (0=3rd, 1=1st) |
| `shoulder` | `Shoulder` | Right | Shoulder orientation |
| `shoulder_lerp` | `f32` | 1.0 | Shoulder interpolation |
| `collision` | `bool` | true | Wall collision avoidance |
| `fov` | `f32` | 70.0 | Current FOV |
| `ads_factor` | `f32` | 0.0 | ADS zoom interpolation |
| `lerp_factor` | `f32` | 0.1 | Camera smoothing |
| `freelook` | `bool` | false | Freelook mode |
| `current_eye_height` | `f32` | 1.65 | Interpolated eye height |
| `desired_position` | `Vec3` | ZERO | Target position |

### PerspectiveState Resource
Global resource for other systems to query current perspective.

### CameraPlugin
Registers 4 systems chained in Update (guarded by `is_not_paused`):

1. **`camera_look_system`** — Reads `AccumulatedMouseMotion`, updates yaw/pitch with sensitivity modulation from stance, weapon weight, and stamina.
2. **`camera_follow_system`** — Positions camera based on perspective mode with orbit math, shoulder offset, eye-height interpolation, and collision raycasts.
3. **`perspective_toggle_system`** — Toggles between 1st/3rd person on MiddleMouse or V key.
4. **`camera_fov_system`** — Applies computed FOV (blended between 3rd/1st base, modified by ADS) to the camera's `Projection`.

### FOV Values

| Mode | FOV |
|------|-----|
| Third Person | 70° |
| First Person | 80° |
| ADS Offset | -15° |
| Clamp Range | 40°–120° |

### Eye Heights Per Stance

| Stance | Height |
|--------|--------|
| Standing / Sprinting | 1.65m |
| Crouching / InCover | 1.00m |
| Prone | 0.35m |

## Post-Processing (`post_processing.rs`)

### PostProcessingProfile Component
```rust
pub struct PostProcessingProfile { pub intensity: f32 }
```

### Effect Stack
Applied to the camera entity at spawn:

1. **ACES Filmic Tone Mapping** — `Tonemapping::AcesFitted`
2. **Bloom** — `Bloom::default()`
3. **Screen Space Ambient Occlusion** — `ScreenSpaceAmbientOcclusion::default()`
4. **Depth of Field** — `DepthOfField::default()`

The `apply_post_processing_system` ensures ACES tone mapping is applied every frame.
