# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — 2026-06-19

### Added

#### Weapons System (8 files)
- **`weapons/` module:** Modular weapon system with 7 sub-modules:
  - `chassis.rs`: 4 weapon platforms (M4A1, MP5SD, M1911, AK-47) with class, caliber, full base stats
  - `caliber.rs`: 7 calibers (9mm → .50 BMG) with damage/penetration/velocity/recoil/range multipliers
  - `barrel.rs`: 5 barrel types (Standard, Suppressor, Compensator, Extended, Short) with stat modifiers
  - `sight.rs`: 5 sight types (Iron, RedDot, Holo, ACOG, SniperScope) with ADS spread/time/zoom
  - `underbarrel.rs`: 4 grip types (None, Vertical, Angled, Bipod) with recoil/spread modifiers
  - `magazine.rs`: 4 magazine types (Standard, Extended, QuickDraw, Drum) with capacity/reload/weight
  - `stock.rs`: 4 stock types (Standard, Folding, Precision, NoStock) with recoil/sway/ADS
  - `CompleteWeapon`: Combines chassis × all attachments to compute final stats (damage, fire rate,
    hip/ADS spread, recoil, weight, ADS speed, sway, max range)

#### Drone System
- **`drones/` module:** Dual drone types:
  - **Recon UAV** (U key): High-altitude surveillance, long battery (120s), auto-return on low battery
  - **FPV Strike Drone** (J key): Fast explosive drone, manual detonation (Space) or proximity detonation,
    one-time use, 200 damage in 8m radius
  - Battery management, camera-relative flight controls, movement physics (velocity lerp)

#### Expanded HUD (7 additional subsystems)
- **XP Notifications:** Popup on XP gain (+50 XP), level-up banners with auto-cleanup
- **Stamina Bar:** Small bar below health, updates in real-time with stamina ratio
- **Achievement Popups:** Golden notification when achievements unlock, 4-second display
- **Kill Feed:** Top-right kill/death messages with 5-second auto-cleanup
- **Squad Status:** Team member health, current order display, DEAD/OK status
- **Objective Tracker:** Mission objective progress text at top of screen

#### Gameplay Systems
- **Breathing system** (`breathing/`): Hold breath (Aim+Sprint) steadies weapon sway up to 50%,
    drains stamina at 8/s, 2-second cooldown between holds
- **Mission system** (`missions/`): Objective tracking with 5 objective types
    (EliminateAll, ReachLocation, DefendPosition, CollectIntel, Extract), auto-completion detection
- **Ammo type system** (`ammo_type/`): 4 ammo types (FMJ, HP, AP, Tracer) with damage/penetration/spread
    multipliers, `LoadedAmmo` component

#### Camera Overhaul
- **1st/3rd Person Toggle** (V key or Middle Mouse): Smooth perspective interpolation,
    separate FOV for each mode (70° 3rd, 80° 1st)
- **ADS Zoom:** FOV reduction on right-click, works in both perspectives
- **Perspective State Resource:** Global `PerspectiveState` queryable by other systems
- **Fixed camera collision:** Raycast-based push-in for 3rd person, disabled in 1st person

#### Gear Integration
- **Inventory → Combat wire:** `PlayerInventory.weapon_damage_bonus()` applied in shooting system
- **Workshop → Weapon wire:** Fitted attachments (WeaponWorkshop) modify active weapon stats:
    damage, spread, and magazine capacity
- **`PlayerInventory`** with slot-based equipment (5 slots: primary, sidearm, helmet, body armor, tactical)

#### Save/Load
- **F5 Save / F9 Load:** Keyboard shortcuts for quick save/load
- **Pause menu buttons:** Save Game / Load Game buttons added to pause overlay
- **SaveData struct:** Complete serialization of progression, stats, inventory, achievements, settings

#### Physics
- **120Hz fixed timestep:** Physics runs at 120 ticks/second for smooth tactical gameplay

### Modified

- **Rendering crate:** Complete camera rewrite with perspective system, ADS zoom, improved collision
- **HUD module:** Expanded from 2 files to 9 files with full gameplay feedback systems
- **Main menu:** Updated with Settings, Controls, and proper navigation
- **Pause menu:** Save/Load game buttons added
- **Shooting system:** Now reads inventory damage bonus from equipped gear
- **Weapon handling:** Weight classes affect movement speed, ADS time, and sway amplitude
- **Stamina system:** Exhaustion affects weapon sway and spread, not just movement speed
- **Turn rate:** Stance + weapon weight + stamina modulate mouse sensitivity

### Technical Stack (Updated)

| Component | Choice | Version |
|-----------|--------|---------|
| Engine | Bevy | 0.18.1 |
| Physics | Avian3d | 0.6.1 |
| Input | leafwing-input-manager | 0.20 |
| Serialization | serde + ron | 1.x / 0.12 |
| VFX | bevy_hanabi | 0.18.0 |
| Audio | bevy_audio (+ kira in deps) | built-in |
| Networking | bevy_replicon + lightyear (Phase 3 ready) | 0.41/0.26 |
| Debug | bevy-inspector-egui | 0.36 |
| Loading | iyes_progress | 0.17 |

### Project Statistics

- **Source files:** 68 across 5 crates (was 42)
- **Module directories:** 16 in game crate (was 9)
- **Lines of Rust:** ~8,500+
- **Build status:** 0 errors, all crates clean

### Known Limitations

- Audio assets need `.ogg` files (footsteps, weapons, ambient)
- No visible character models — capsule colliders only until Blender assets
- bevy_hanabi wired but no particle effects configured yet
- kira in dependencies but bevy_audio used currently
- Networking (bevy_replicon/lightyear) in deps but not active
- No level loading from files — procedural level only
