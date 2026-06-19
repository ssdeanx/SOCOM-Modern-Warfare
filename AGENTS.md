# AGENTS.md — SOCOM Tactical Shooter

## Workspace Architecture

Multi-crate Cargo workspace (5 crates, 68 source files). Dependency direction:

```
game (binary) — 52 files across 16 module directories
  ├── core (path)      — Pure data types. ZERO Bevy dep. 3 files.
  ├── input (path)     — leafwing-input-manager bindings. 3 files.
  ├── rendering (path) — 1st/3rd person camera rig. 2 files.
  └── audio (path)     — bevy_audio footstep/ambient. 3 files.
```

**Rule:** `core` must never depend on Bevy. All other crates depend on `core` with `features = ["bevy"]` to derive `Component` on its types.

## Crate Responsibilities

| Crate | Depends On | Purpose |
|-------|-----------|---------|
| `core` (3 files) | serde, glam | `Player`, `Health`, `MovementState`, `Team`, `Weapon`, `WeaponSlot`, `Shoulder`, `GameSettings`, `InputMapping`, `Paused`, `SensitivityMultiplier` |
| `input` (3 files) | bevy 0.18.1, leafwing 0.20, core | `PlayerAction` enum (15 variants), `InputPlugin` with keyboard+gamepad bindings |
| `rendering` (2 files) | bevy 0.18.1, core, input, avian3d 0.6.1 | `ThirdPersonCamera` with 1st/3rd person toggle, ADS zoom, freelook, shoulder swap, collision raycast, FOV management |
| `audio` (3 files) | bevy 0.18.1, core | `FootstepPlugin` (5 surfaces), `AmbientPlugin` — bevy_audio |
| `game` (52+ files) | bevy 0.18.1, avian3d 0.6.1, leafwing, all above | **16 directories + 10 root files.** Everything gameplay: combat, physics, AI, drones, weapons, gear, progression, HUD, menus, missions, squad, tactical |

## Game Module Map (16 directories)

| Directory | Files | Purpose |
|-----------|-------|---------|
| `combat/` | 8 | shooting, damage, death, reload, weapon_bob, weapon_model, weapon_state, impacts, mod |
| `weapons/` | 8 | chassis, caliber, barrel, sight, underbarrel, magazine, stock, CompleteWeapon |
| `hud/` | 9 | elements, systems, xp_notification, stamina_bar, achievement_popup, kill_feed, squad_status, mod |
| `physics/` | 5 | player_movement, enemy_movement, stance, layers, mod |
| `gear/` | 5 | items, inventory, attachments, workshop, mod |
| `progression/` | 5 | xp, stats, achievements, specializations, mod |
| `feedback/` | 4 | hit_marker, vignette, enemy_fx, mod |
| `states/` | 4 | mod, main_menu, loading, ingame |
| `ai/` | 3 | mod, enemy (FSM), teammate |
| `controls/` | 3 | mod, stance (transition timers), turn_rate |
| `squad/` | 3 | mod, orders (command dispatch), formation |
| `tactical/` | 3 | mod, command_wheel, cover, suppression |
| `menu/` | 3 | mod, settings, keybinds |
| `ammo_type/` | 1 | AmmoType enum (FMJ, HP, AP, Tracer) + LoadedAmmo component |
| `breathing/` | 1 | Breathing component + hold-breath system |
| `drones/` | 1 | Drone system (Recon UAV + FPV Strike) |
| `missions/` | 1 | MissionState resource, objective tracking |
| `stamina/` | 1 | Stamina drain/regen + sway/spread mods |
| `weapon_handling/` | 1 | WeaponWeight (Light/Medium/Heavy) + handling stats |
| **(root)** | 10 | main, player, level, messages, pause, console, camera_control, settings, settings_applier, save_load |

## Current Phase: Phase 2 Complete — Pre-Asset

All core gameplay systems are implemented. Remaining work before first playable build:
- Placeholder audio assets (.ogg files)
- bevy_hanabi VFX configuration
- Character/weapon models (Blender MCP pipeline)
- Post-processing pipeline

## Key Technology Versions (locked)

| Dep | Version | Notes |
|-----|---------|-------|
| bevy | **0.18.1** | NOT 0.19 RC. Pinned across ALL crates. |
| avian3d | **0.6.1** | Pinned across all crates using it. |
| leafwing-input-manager | 0.20.x | `Actionlike`, `InputMap::insert()`, `insert_dual_axis()` |
| serde + ron | 1.x / 0.12 | Core + save serialization |
| glam | 0.29 | Matches bevy 0.18.1 |
| bevy_hanabi | 0.18.0 | GPU particles (wired, not configured) |
| bevy_replicon | 0.41.0-rc.1 | Networking (Phase 3) |
| lightyear | 0.26.4 | Rollback networking (Phase 3) |
| kira | 0.12 | Audio middleware (deps only, using bevy_audio) |

## Bevy 0.18.1 Quirks Checklist

- **Messages** not Events: `MessageReader`/`MessageWriter` derive `#[derive(Message)]`, register with `app.add_message::<T>()`
- **Mouse look:** `Res<AccumulatedMouseMotion>` not `EventReader<MouseMotion>`
- **Audio:** `(AudioPlayer(handle), PlaybackSettings::ONCE)` not `Res<Audio>.play()`
- **UI:** `Text + TextFont + TextColor + Node`, NOT `TextBundle`/`TextStyle`/`Style`
- **Despawn:** `commands.entity(e).despawn()` (always recursive)
- **Projection:** `&mut bevy::camera::Projection`, match on `Perspective(ref mut persp)`
- **Fullscreen:** `WindowMode::BorderlessFullscreen(MonitorSelection::Current)`
- **Camera3d FOV:** On `Projection::Perspective(PerspectiveProjection)`, NOT on `Camera3d`

## State Machine

```
MainMenu ──(New Game button)──> Loading ──(0.5s timer)──> InGame
    ↑                                                       │
    └──────────────────(Escape)─────────────────────────────┘
```

States: `AppState` enum in `crates/game/src/states/mod.rs`

## All System Messages (14)

| Message | Source → Consumer |
|---------|------------------|
| `DamageMessage` | shooting → damage, stats, feedback |
| `DeathMessage` | death_check → player_death, stats, xp, effects |
| `WeaponFiredMessage` | shooting → stats |
| `PlayerDamagedMessage` | damage → vignette, suppression |
| `HitConfirmedMessage` | shooting → hit_marker, audio |
| `XpGainedMessage` | xp system → HUD |
| `LevelUpMessage` | xp system → HUD, gear unlock |
| `AchievementUnlockMessage` | achievements → HUD popup |
| `SquadOrderMessage` | command_wheel → squad AI |
| `SquadStatusMessage` | squad AI → HUD |
| `CoverStateMessage` | cover_detection → movement, weapons |
| `SuppressionMessage` | suppression → feedback, weapons |
| `ItemPickupMessage` | loot → inventory |
| `ItemEquipMessage` | gear UI → inventory, weapons |

## Controls

| Input | Action |
|-------|--------|
| WASD | Camera-relative movement |
| Mouse | Camera look (pitch/yaw) |
| Left Click | Fire weapon |
| Right Click | Aim down sights |
| Shift | Sprint / Hold breath (while ADS) |
| C | Crouch toggle |
| Z | Prone toggle |
| Space | Jump / Detonate FPV drone |
| R | Reload |
| Q | Shoulder swap (left/right) |
| E | Interact |
| Tab | Command wheel |
| V / MMB | Toggle 1st/3rd person |
| 1 / 2 | Primary / Sidearm |
| U | Deploy/recall Recon Drone |
| J | Deploy/recall FPV Strike Drone |
| Escape | Pause / Menu |
| Backtick | Dev console |
| F5 | Quick save |
| F9 | Quick load |
| F1 | Inspector (bevy-inspector-egui) |

## Build Status

- `cargo check` — **0 errors** across all 5 crates
- `cargo clippy` — clean (78 dead_code warnings — expected for unused systems awaiting full gameplay loop)
- `cargo fmt` — consistent formatting
- `cargo tree -p socom-core` — confirms zero Bevy deps

## Agents Working Here

- **Before touching code:** Check `.omo/plans/` and `.omo/drafts/` for current plan context
- **Build commands:** `cargo check`, `cargo clippy`, `cargo fmt`
- **Watch mode:** `cargo watch -x check -x clippy`
- **Dependency edits:** `cargo add / cargo rm` (cargo-edit installed)
- **Always verify:** Bevy 0.18.1 pinned correctly, NOT 0.18.0 or 0.19
- **All external packages in Cargo.toml must be wired into the app** (not just in deps)
- **Keep files modular:** Max ~200 lines per file, single responsibility
- **Messages for communication:** Never use Events, always `#[derive(Message)]` + `app.add_message()`
