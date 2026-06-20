# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] — 2026-06-19

### Added

#### Full Kira Audio Host (P5)
- **`KiraHostPlugin`** (302 lines): Standalone kira 0.12 wrapper as Bevy resource
- **Bus hierarchy:** Master → SFX / Ambient / UI / Voice sub-tracks
- **Spatial audio listener:** Follows main 3D camera each frame via mint types
- **Occlusion API:** Pre-attached low-pass filter on SFX bus, `set_occlusion_filter()` for dynamic cutoff
- **Playback API:** `play_sfx()`, `play_ambient()`, `play_ui()`, `play_voice()` with graceful missing-file handling
- **Scene transitions:** `stop_all()` / `resume_all()` for pausing all buses

#### Destruction System (P5.5) — 6 files, 971 lines
- **`DestructionState` component:** State machine (Pristine→Damaged→Breached→Destroyed) with 13 material types
- **Material penetration table:** 13 materials × 7 calibers lookup with threshold-based penetration
- **Bullet penetration system:** Passes reduced damage through thin materials after penetration
- **Explosion damage system:** Spherical falloff from `GrenadeDetonatedMessage` to all destructible entities
- **Debris spawning:** Per-material debris counts (5-40 entities), timed auto-cleanup
- **Glass fracture system:** 24 pre-scored crack points, 20-40 shards on shatter
- **Vehicle damage states:** 4 states (Operational→Disabled→Burning→Wreck) with speed/behavior rules
- **Building collapse:** `CollapseDebris` component with velocity and dust cloud spawning
- **`DestructionTransitionMessage`:** Emitted on every state transition for FX/audio hooks

#### Post-Processing Stack (P4.3-P4.4)
- **Screen Space Ambient Occlusion (SSAO):** `ScreenSpaceAmbientOcclusion::default()` on camera
- **Depth of Field:** `DepthOfField::default()` on camera for cinematic focus effects

#### Weapon Audio Relay (B0)
- **`AudioRelayPlugin`:** Bridges `WeaponFiredMessage`, `HitConfirmedMessage`, `EquipmentUsedMessage` to kira SFX bus
- **File mapping:** Weapons routed to specific .ogg paths (m4a1_fire.ogg, ak47_fire.ogg, etc.)

#### Logging System
- **`RUST_LOG` support:** Set env var for filtered tracing (`socom=debug,warn` by default)
- Uses Bevy's built-in tracing infrastructure with env-filter

#### Sniper Rifle Chassis (A19)
- **M24 SWS:** Bolt-action 7.62mm NATO, 5rd internal, 800m range, 1.2s cycle
- **L96A1:** Bolt-action 7.62mm NATO, 10rd detachable, 900m range, 1.4s cycle
- **`WeaponWeight::Sniper`:** 0.45x movement speed, 0.50s ADS time

#### Advanced Drones (A18)
- **Grenade Drone (H key):** Quadcopter with 4x frag hardpoints, WASD flight, SPACE to drop
- **Mine Drone (N key):** 3x AP mine dispenser, G key to deploy at current position
- **`GrenadeDroneBundle` / `MineDroneBundle`:** Spawn with dedicated physics colliders

#### Equipment System (A16) — 9 new files
- **Throwables:** 5 grenade types (Frag, Flashbang, Smoke, TearGas, Flare) with physics projectiles and fuse timers
- **Deployables:** C4 (remote detonation), Claymore (directional 90° cone tripwire), Breaching Charge
- **Melee:** Combat knife (2m range, 60° cone, 100 damage), throws `DamageMessage`
- **Equipment inventory:** 5-slot cycling with G-tap / G-hold
- **C4 detonation:** Backspace key detonates all placed C4

#### Medic & Healing System (A17)
- **Self-heal:** Bandage (+25 HP), Medkit (+75 HP), Splint (+40 HP), Energy Drink (+15 HP)
- **Bleed-out:** 30s timer, headshot = instant death (no revive)
- **Revive system:** Approach downed teammate + E key = revive with selected med item
- **Health component expanded:** `armor`, `is_downed`, `bleed_out_remaining`, `revive_progress`

### Changed

- **ControlsPlugin** — Registered in main.rs, stance transition system now active
- **Player camera** — Now spawns with `ScreenSpaceAmbientOcclusion` + `DepthOfField` components
- **Build target:** Removed 3.2GiB of old artifacts during cleanup
- **`#[expect(dead_code)]`** annotations added across 30+ files (down from 60→36 warnings)

### Technical Stack

| Component | Choice | Version |
|-----------|--------|---------|
| Engine | Bevy | 0.18.1 |
| Physics | Avian3d | 0.6.1 |
| Input | leafwing-input-manager | 0.20 |
| Serialization | serde + ron | 1.x / 0.12 |
| VFX | bevy_hanabi | 0.18.0 |
| Audio (primary) | kira | 0.12 |
| Audio (legacy) | bevy_audio | built-in |
| Networking | bevy_replicon + lightyear | 0.41 / 0.26 |
| Debug | bevy-inspector-egui | 0.36 |

### Project Statistics

- **Source files:** 100+ across 5 crates
- **Module directories:** 16 in game crate + 3 other crates
- **System messages:** 18 registered
- **Build status:** 0 errors, 0 clippy errors
- **Remaining warnings:** 36 (weapon attachment system awaiting CompleteWeapon integration)

## [0.3.0] — 2026-06-19

### Added

#### bevy_hanabi VFX (P3) — combat/vfx.rs
- **Muzzle flash:** Yellow/orange burst at weapon muzzle (64 particles, 0.1s)
- **Bullet impact:** Orange/white sparks at hit location (32 particles, 0.25s)
- **Hit marker:** Red flash at damage point (8 particles, 0.12s)
- **Death explosion:** Burst of 6 particles outward with gravity-affected trajectory (1s)
- **Tracer round:** Yellow-green glowing sphere trail (16 particles, 0.3s)
- All effects use `ColorOverLifetimeModifier` + `SizeOverLifetimeModifier` + auto-cleanup

#### Post-Processing (P4.1-P4.2)
- **ACES filmic tone mapping:** `Tonemapping::AcesFitted` on camera spawn
- **Bloom:** `Bloom::default()` for bright effects (muzzle flash, explosions)
- **PostProcessingProfile:** Intensity multiplier component for future expansion

### Fixed

- **post_processing.rs:** Changed `single()` → `iter_mut()` for Tonemapping write access
- **drones/mod.rs:** Added missing `GrenadeDroneBundle` and `MineDroneBundle`
- **healing.rs:** `get_single_mut()` → `single_mut()` for Bevy 0.18 API
- **DroneType pattern matching:** Added GrenadeDrone + MineDrone variants to non-exhaustive matches

## [0.2.0] — 2026-06-19

### Added

#### Weapons System (8 files)
- **`weapons/` module:** Modular weapon system with 7 sub-modules:
  - `chassis.rs`: 6 weapon platforms (incl. M24, L96A1 snipers)
  - `caliber.rs`: 7 calibers (9mm → .50 BMG) with stat multipliers
  - `barrel.rs`: 5 barrel types with stat modifiers
  - `sight.rs`: 5 sight types (incl. SniperScope)
  - `underbarrel.rs`: 4 grip types
  - `magazine.rs`: 4 magazine types
  - `stock.rs`: 4 stock types
  - `CompleteWeapon`: Combines chassis × all attachments → final computed stats

#### Drone System
- **Recon UAV** (U key): High-altitude, 120 battery, 40s flight, auto-return
- **FPV Strike Drone** (J key): Fast, 30 battery, 10s flight, detonate (Space)

#### Expanded HUD (9 subsystems)
- Health bar, stance, ammo counter, crosshair, weapon name
- XP notifications, level-up banners, stamina bar
- Achievement popups, kill feed, squad status, objective tracker

#### Gameplay Systems
- Breathing system (hold breath steadies aim)
- Mission system (5 objective types)
- Ammo type system (FMJ, HP, AP, Tracer)
- 1st/3rd person camera toggle with smooth interpolation
- ADS zoom in both perspectives (FOV reduction)
- Gear integration (inventory → combat, workshop → weapon)
- Save/load (F5/F9, pause menu)

### Project Statistics (v0.2.0)

- **Source files:** 68 across 5 crates
- **Build status:** 0 errors

### Known Limitations

- ⚠️ Full `cargo run` or `cargo build` requires ~10GB free disk space for debug dependencies
- Audio is placeholder — kira host wired but needs .ogg files
- No visible character/weapon models (awaiting Blender MCP pipeline — Phase 6-8)
- Networking deps (bevy_replicon/lightyear) present but not wired
- Modular weapon system awaits CompleteWeapon integration into player loadout
- SSAO requires TAA or Msaa::Off to function optimally
