# Phase 0: Foundation — SOCOM Tactical Shooter

> **Duration:** Weeks 1-4
> **Goal:** Initialize the full project workspace, prove the render/input/physics loop, and establish CI/build quality gates.
> **Stack:** Bevy 0.18.1 + Avian3d 0.6.1 + leafwing-input-manager 0.20.0

---

## Decision Record

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Engine | Bevy 0.18.1 (stable) | Max crate compatibility; 0.19.0-rc.3 not fully supported by ecosystem |
| Physics | Avian3d 0.6.1 | Native ECS, no bridged world; XPBD solver; enhanced-determinism |
| Character Controller | Move-and-slide (Avian built-in) | Avian 0.6 just shipped move-and-slide; bevy-tnua for advanced later |
| Input | leafwing-input-manager 0.20.0 | 25K+ downloads/mo, mature, action-based, networking support |
| Camera | Custom 3rd-person rig | Bevy 0.18 has built-in fly/pan cams but no 3rd-person chase cam |
| Audio | bevy_audio (built-in) + bevy_kira_audio | Phase 0: basic feedback sounds. Full spatial audio in Phase 2 |
| Workspace | Multi-crate Cargo workspace | core (no Bevy deps) + game (binary) + input + rendering |
| State Management | Bevy State + SubState | MainMenu → Loading → InGame |
| CI | GitHub Actions + cargo-check/clippy/test | Quality gates on every PR |
| Formatting | rustfmt + cargo sort | Consistent code style |
| **Squad System** | **Phase 1** (confirmed) | Phase 0 is single-player-only with simple follow-AI |
| **Test Level** | **Procedural geometry** (confirmed) | Code-generated room — no Blender dependency in Phase 0 |

---

## Cargo Workspace Configuration

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "crates/game",
    "crates/core",
    "crates/input",
    "crates/rendering",
    "crates/audio",
]
```

### Dependency Graph (Phase 0)

```
game (binary)
  ├── core (path)       — ZERO Bevy deps
  ├── input (path)      — core + leafwing-input-manager
  ├── rendering (path)  — core + bevy (3d features)
  └── audio (path)      — core + bevy_audio + kira
```

---

## Tasks

### T1: Initialize Cargo Workspace

**Where:** `Cargo.toml` (workspace root), `crates/*/Cargo.toml`
**How:** Create workspace manifest with all member crates. Create each crate with `lib.rs` or `main.rs`. Core crate gets no Bevy deps. Game crate gets bevy dependency.
**Expected Result:** `cargo build` compiles all crates. `cargo test` passes (zero tests).

**Files to create:**
- `Cargo.toml` — workspace manifest
- `crates/core/Cargo.toml` + `src/lib.rs`
- `crates/game/Cargo.toml` + `src/main.rs`
- `crates/input/Cargo.toml` + `src/lib.rs`
- `crates/rendering/Cargo.toml` + `src/lib.rs`
- `crates/audio/Cargo.toml` + `src/lib.rs`
- `rust-toolchain.toml` — channel = "stable"
- `.cargo/config.toml` — optional build opts

**Acceptance Criteria:**
- `cargo build` succeeds
- `cargo test` passes
- All crate directories exist with proper Cargo.tomls
- core crate has no bevy dependency (verify with `cargo tree -p core`)

---

### T2: Configure Bevy Engine with Minimal App Loop

**Where:** `crates/game/src/main.rs`
**How:** Use `bevy 0.18.1` with features `["3d", "audio"]`. Create App builder with DefaultPlugins. Add state management using Bevy states: `AppState` enum with `MainMenu`, `Loading`, `InGame` variants. Add a simple startup system that spawns a 3D scene with a ground plane and camera.

```rust
// AppState
#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    Loading,
    InGame,
}
```

**Must include:**
- `DefaultPlugins` with 3d + audio feature set
- `AppState` state machine
- Minimal startup: ground plane + directional light
- Simple `InGame` system that just despawns placeholder

**Expected Result:** Running `cargo run` opens a Bevy window with a grey ground plane and a directional light. Window title: "SOCOM Tactical Shooter — Phase 0".

**Files to create/modify:**
- `crates/game/src/main.rs`

**Acceptance Criteria:**
- Window opens at 1280x720 (configurable)
- Ground plane visible with lighting
- AppState can be transitioned programmatically
- FPS overlay visible via bevy_dev_tools (dev feature only)

---

### T3: core Crate — Define Shared Types & Components

**Where:** `crates/core/src/`
**How:** Create Phase 0 core types that all other crates depend on. These are pure data — no Bevy dependency.

**Components:**
- `Player` — marker component
- `Velocity` — `Vec3` wrapper for movement direction
- `MovementState` — enum: `Standing | Sprinting | Crouching | Prone | InCover`
- `Health` — `{ current: f32, max: f32 }`
- `Team` — enum: `Player | Teammate | Enemy`
- `WeaponSlot` — `{ primary: Option<Weapon>, sidearm: Option<Weapon> }`

**Resources:**
- `GameSettings` — `{ master_volume: f32, sensitivity: f32, invert_y: bool }`
- `InputMapping` — placeholder for Phase 1 keybinding profile

**Expected Result:** `crate core` compiles, has no bevy dependency, all types derive `Debug, Clone, PartialEq`.

**Files to create:**
- `crates/core/src/lib.rs`
- `crates/core/src/components.rs`
- `crates/core/src/resources.rs`

**Acceptance Criteria:**
- `cargo build -p core` succeeds
- `cargo tree -p core` shows zero bevy dependencies
- All types have proper derives

---

### T4: Input Crate — leafwing-input-manager Integration

**Where:** `crates/input/`
**How:** Create a Bevy plugin that integrates leafwing-input-manager. Define `PlayerAction` enum for Phase 0 actions.

**PlayerAction enum (Phase 0):**
```rust
#[derive(ActionLike, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Move,        // Axis2D — WASD / Left Stick
    Look,        // Axis2D — Mouse / Right Stick
    Sprint,      // Button — Shift / L3
    Crouch,      // Button — C / B
    Prone,       // Button — Z / Down D-Pad
    Jump,        // Button — Space / A
    Interact,    // Button — E / Y
    Pause,       // Button — Escape / Start
}
```

**Plugin structure:**
- `InputPlugin` — adds leafwing's `InputManagerPlugin::<PlayerAction>`
- Default key bindings (keyboard + gamepad)
- `InputMap::default()` with WASD + mouse + gamepad bindings

**Expected Result:** A player entity can read movement input via `ActionState<PlayerAction>` queries.

**Files to create:**
- `crates/input/src/lib.rs`
- `crates/input/src/actions.rs`
- `crates/input/src/bindings.rs`

**Acceptance Criteria:**
- `PlayerAction::Move` returns `Vec2` from WASD or left stick
- `PlayerAction::Look` returns `Vec2` from mouse or right stick
- Keyboard bindings match spec (WASD, Shift, C, Z, Space, E, Escape)
- Gamepad bindings work when controller connected
- `cargo build` succeeds

---

### T5: Rendering Crate — 3rd Person Camera Rig

**Where:** `crates/rendering/`
**How:** Create a configurable 3rd-person camera system. This is custom since Bevy doesn't have a built-in 3rd-person follow cam.

**ThirdPersonCamera component:**
```rust
#[derive(Component)]
pub struct ThirdPersonCamera {
    pub target: Entity,        // Player entity to follow
    pub distance: f32,         // 4.0m default
    pub pitch: f32,            // Current vertical angle
    pub yaw: f32,              // Current horizontal angle
    pub min_pitch: f32,        // -30° (don't go under ground)
    pub max_pitch: f32,        // 80°
    pub shoulder: Shoulder,    // Left, Right
    pub collision: bool,       // Camera collides with world
    pub fov: f32,              // 70° default
}
```

**CameraPlugin systems:**
1. `camera_follow_system` — each frame, set camera transform relative to target
2. `camera_look_system` — read `ActionState<PlayerAction>::Look` → adjust pitch/yaw
3. `camera_collision_system` — raycast from target to desired camera pos, push camera forward if blocked

**Expected Result:** Player entity has a chase camera following from behind, mouse/look controls orbit, camera pushes in when backing against a wall.

**Files to create:**
- `crates/rendering/src/lib.rs`
- `crates/rendering/src/camera.rs`

**Acceptance Criteria:**
- Camera follows player entity smoothly (lerp)
- Mouse look orbits camera around player
- Camera collision prevents clipping through walls
- Shoulder swap (left/right) via keybind (placeholder)
- Distance scales with FOV change

---

### T6: Physics Integration — Avian3d Player Controller

**Where:** `crates/game/src/physics.rs`
**How:** Integrate Avian3d with a kinematic character controller using the move-and-slide pattern from Avian 0.6.

**PlayerBundle:**
- `RigidBody::Kinematic`
- `Collider::capsule(0.3, 0.9)` — radius, half-height (standing)
- `LinearVelocity` — set by movement system
- `MovementState` — Standing
- `Player` marker
- `ThirdPersonCamera::target`

**MovementSystem:**
- Read `ActionState<PlayerAction>::axis_pair(Move)` → normalized Vec2
- Transform input direction by camera yaw → world-space Vec3
- Map to `LinearVelocity` on kinematic body
- Sprint: multiply speed by 1.5x
- Crouch: shrink collider height, halve speed
- Prone: shrink collider further, quarter speed

**Speed constants (m/s):**
- Walk: 3.0
- Sprint: 5.0
- CrouchWalk: 1.5
- ProneCrawl: 0.8

**Expected Result:** Player capsule moves with WASD, orients with mouse, sprints with shift, crouches with C, goes prone with Z. Physics collision with ground plane works.

**Files to create:**
- `crates/game/src/physics.rs`
- `crates/game/src/player.rs` — PlayerBundle, spawn_player_system

**Acceptance Criteria:**
- WASD moves player in camera-relative direction
- Shift toggles sprint (speed increase)
- C toggles crouch (height reduction + speed decrease)
- Z toggles prone (further height/speed reduction)
- Player collides with ground plane (doesn't fall through)
- No jittery movement (fixed timestep 1/60)

---

### T7: Audio Integration — Basic Footsteps & Ambient

**Where:** `crates/audio/`
**How:** Set up `bevy_audio` (built-in) for basic 2D sounds. Create a simple system that plays footstep sounds on movement.

**AudioPlugin:**
- Load footstep audio assets on startup
- `footstep_system` — runs when player `MovementState` changes or on distance-traveled threshold
- Placeholder ambient loop

**Asset setup:**
- `assets/audio/footstep_dirt_1.ogg` ... `_4.ogg`
- `assets/audio/ambient_test.ogg`

**For Phase 0,** use simple bevy_audio (not kira yet — kira integration in Phase 2). Generate or download placeholder OGG files (250ms clicks for footsteps, 10s hum for ambient).

**Expected Result:** Footsteps play when player moves. Ambient loop plays. Audio stops on pause.

**Files to create:**
- `crates/audio/src/lib.rs`
- `crates/audio/src/footsteps.rs`
- `crates/audio/src/ambient.rs`

**Acceptance Criteria:**
- Footsteps play at regular intervals while moving
- Different sounds for walk/sprint/crouch/prone (different interval rates)
- Ambient audio plays in main menu and InGame
- Volume controlled by `GameSettings.master_volume`

---

### T8: Asset Loading & State Management

**Where:** `crates/game/src/states/`
**How:** Create clean state transitions. Use `bevy_asset_loader` for loading screen.

**States flow:**
```
MainMenu ──(start)──> Loading ──(assets loaded)──> InGame
                                   ↑
                              (return to menu)
```

**Loading screen:**
- Resource collection via `AssetCollection` derive macro
- Progress bar (simple Bevy UI)
- On complete: transition to InGame, spawn player

**Expected Result:** App boots to a placeholder main menu screen, transitions to a loading screen, then transitions to InGame with player spawned.

**Files to create:**
- `crates/game/src/states/mod.rs`
- `crates/game/src/states/main_menu.rs`
- `crates/game/src/states/loading.rs`
- `crates/game/src/states/ingame.rs`

**Acceptance Criteria:**
- MainMenu shows title text "SOCOM Tactical Shooter"
- Pressing Space/Enter transitions to loading
- Loading screen shows asset progress
- Loading complete → InGame with player spawned

---

### T9: Greybox Test Level

**Where:** `assets/models/test_level.glb`
**How:** Create a minimal level for testing movement and camera. Either:
- (A) Procedurally generate a room from code using Bevy shapes (Cube, Plane)
- (B) Create a simple GLB in Blender with walls, pillars, slopes, and stairs

Recommend: (A) for speed in Phase 0, (B) for Phase 1.

**Spawn obstacles:**
- 4 walls forming a 20m x 20m room
- 3 pillars (1m cubes) at various positions
- 1 ramp (sloped plane, 30°)
- 2 stair steps (0.5m height each)

**Expected Result:** Player can move around the room, bump into walls, walk up the ramp, step up stairs. Camera doesn't clip through walls.

**Files to create:**
- `crates/game/src/level.rs`
- Optional: `assets/models/test_level.glb`

**Acceptance Criteria:**
- Room boundaries constrain player movement
- Pillars block movement and camera
- Ramp is climbable (avians slide angle > 45° only)
- Stairs are traversable (step height 0.3m)
- Camera collision pushes in against walls

---

### T10: CI Pipeline

**Where:** `.github/workflows/ci.yml`
**How:** Standard Rust CI with:
- `cargo check` — verify compilation
- `cargo clippy` — lint (deny warnings)
- `cargo test` — unit tests
- `cargo fmt --check` — formatting
- `cargo sort --check` — dependency ordering (if using)

**Trigger:** Push to main, PR to main

**Expected Result:** CI passes on first PR.

**Files to create:**
- `.github/workflows/ci.yml`
- `clippy.toml` — linter config
- `rustfmt.toml` — formatter config

**Acceptance Criteria:**
- CI runs `cargo check`, `clippy`, `test`, `fmt`
- All pass on clean state
- Fails if lint warning or test failure introduced

---

### T11: Developer Tooling Setup

**Where:** Project root
**How:** Set up helpful dev tools:

- `justfile` — command runner with recipes for: `build`, `run`, `check`, `clippy`, `test`, `watch`
- `.vscode/settings.json` — Rust analyzer settings, formatter on save
- `.gitignore` — Rust-standard + `assets/` ignored (we track asset sources separately)

**Expected Result:** `just run` builds and runs. `just check` runs CI checks locally. VSCode has Rust analyzer configured.

**Files to create:**
- `justfile`
- `.vscode/settings.json`
- `.gitignore`

**Acceptance Criteria:**
- `just` lists all recipes
- `just run` = `cargo run`
- `just check` = `cargo check && cargo clippy && cargo fmt --check`
- `.gitignore` covers target/, .DS_Store, *.blend1, etc.

---

## Acceptance Criteria (Phase 0 Complete)

- [ ] `cargo build` succeeds on clean checkout
- [ ] `cargo run` opens window with 3D scene
- [ ] WASD moves player in 3D space, camera follows
- [ ] Mouse orbit controls camera
- [ ] Sprint/crouch/prone toggles work
- [ ] Camera collision prevents wall clipping
- [ ] Footstep audio plays on movement
- [ ] App boots: MainMenu → Loading → InGame
- [ ] Greybox room has walls, pillars, ramp, stairs
- [ ] CI passes all checks
- [ ] `just` commands work
- [ ] Core crate has zero Bevy dependencies

---

## Key Files to Read Before Starting Phase 0

These are the reference docs you SHOULD read before implementing:

1. **Bevy 0.18 Book:** https://bevyengine.org/learn/book/getting-started/
   - Focus: Plugin architecture, App builder, States, Systems
2. **Avian 0.6 Book:** https://avianphysics.github.io/avian-book/
   - Focus: Move-and-slide character controller, Collision layers
3. **leafwing-input-manager docs:** https://docs.rs/leafwing-input-manager/0.20.0/
   - Focus: Action-like derive, InputMap, ActionState
4. **Bevy examples (local):**
   - `examples/3d/3d_scene.rs` — basic scene setup
   - `examples/3d/top_down_camera.rs` — camera rig pattern

---

## Risk Registry

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Avian 0.6 move-and-slide API changed from docs | Medium | High | Pin exact version; test immediately |
| leafwing + Avian conflict on system ordering | Medium | Medium | Explicit system sets with `after()` |
| Camera collision raycast doesn't work with Avian colliders | Medium | High | Use `bevy_mod_raycast` or Avian's `spatial_query` |
| Bevy 0.18 to 0.19 upgrade breaks ecosystem | Low (Phase 0) | High | Pin to 0.18.1; don't chase RCs |
| WASM target doesn't compile | Low | Low | Ignore WASM for Phase 0 |

---

## Rollback Strategy

If any task cannot be completed in 4 hours total dev time, mark it BLOCKED and move to the next available task. The critical path is:
1. T1 (workspace) → T2 (bevy app loop) → T4 (input) → T5 (camera) → T6 (physics) → T8 (states)
2. T3 (core types) is a dependency for T4/T5/T6
3. T7 (audio), T9 (level), T10 (CI), T11 (tooling) are parallelizable off the critical path

---

> **Start executing with:** `/start-work`
> **Plan file:** `.omo/plans/phase-0-foundation.md`
