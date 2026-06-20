# Game Crate (`socom-game`)

**Path:** `crates/game/`  
**Files:** 88+ source files across 16 module directories + 10 root files  
**Dependencies:** `bevy 0.18.1`, `avian3d 0.6.1`, `leafwing 0.20`, `core`, `input`, `rendering`, `audio`, `bevy_hanabi`, `bevy-inspector-egui`  
**Purpose:** Everything gameplay — binary crate that ties all other crates together

## Architecture

The game crate is the **binary entry point** (`main.rs`) and contains all gameplay logic organized into 16 module directories plus 10 root-level files. It registers all other crate plugins, initializes resources, sets up the state machine, and runs the Bevy app loop.

```
socom-game (binary crate)
│
├── main.rs           — Entry point, plugin registration, message registration
├── player.rs         — PlayerBundle + PlayerPlugin
├── level.rs          — Procedural test level spawn
├── messages.rs       — 14 game message types
├── pause.rs          — PausePlugin (Escape toggle + menu UI)
├── console.rs        — Developer console (Backtick)
├── camera_control.rs — Shoulder swap, freelook, sensitivity update
├── settings.rs       — SettingsPlugin (load/save/apply GameSettings)
├── settings_applier.rs — Runtime settings system
├── save_load.rs      — SaveManager (RON save files)
├── audio_relay.rs    — Game message → kira audio bridge
│
├── ai/               — Enemy AI FSM + Teammate AI
├── ammo_type/        — AmmoType enum (FMJ/HP/AP/Tracer)
├── breathing/        — Hold-breath system
├── combat/           — Shooting, damage, death, destruction, VFX
├── controls/         — Stance transitions, turn rate
├── drones/           — 4 drone types (Recon, FPV, Grenade, Mine)
├── feedback/         — Hit markers, damage vignette, enemy FX
├── gear/             — Items, inventory, weapons workshop, equipment
├── hud/              — Health bar, ammo, stamina, XP, kill feed
├── menu/             — Main menu, settings, keybinds
├── missions/         — Mission objectives, progress tracking
├── physics/          — Player/enemy movement, collision layers
├── progression/      — XP, stats, achievements, specializations
├── squad/            — Teammate squad orders + formation
├── stamina/          — Stamina drain/regen system
├── states/           — App state machine (MainMenu → Loading → InGame)
├── tactical/         — Command wheel, cover detection, suppression
├── weapon_handling/  — Weapon weight classes + handling modifiers
└── weapons/          — CompleteWeapon assembly + attachment types
```

## Root Module Details

### `main.rs`
The application entry point. Configures:
- Window: 1280×720, title "SOCOM Tactical Shooter"
- Plugins: DefaultPlugins, PhysicsPlugins, HanabiPlugin, PostProcessingPlugin
- Dev tools: FPS overlay, bevy-inspector-egui (F1)
- Registers 14 message types
- Adds all internal plugins, state plugins, and direct gameplay plugins
- App state machine: MainMenu → Loading → InGame

### `player.rs`
Defines `PlayerBundle` — the complete set of components for the player entity:
- `Player`, `Health` (100 HP), `MovementState::Standing`
- `WeaponSlot` (M4A1 primary + M1911 sidearm)
- `WeaponState` + `OffhandWeaponState`
- `Stamina`, `WeaponHandling`, `WeaponBobState`
- Kinematic rigidbody + capsule collider
- Camera with full post-processing stack (Bloom, SSAO, DoF, Tonemapping)

### `messages.rs`
Defines 14 game message types (complementing `DamageMessage` + `DeathMessage` in combat module):
- Combat: `WeaponFiredMessage`, `PlayerDamagedMessage`, `HitConfirmedMessage`
- Progression: `XpGainedMessage`, `LevelUpMessage`, `AchievementUnlockMessage`
- Squad: `SquadStatusMessage`
- Tactical: `CoverStateMessage`, `SuppressionMessage`
- Gear: `ItemPickupMessage`, `ItemEquipMessage`, `EquipmentUsedMessage`
- Equipment: `GrenadeDetonatedMessage`, `MeleeHitMessage`

### `level.rs`
Spawns the procedural greybox test level: ground plane, four walls, four pillars, a ramp, five stairs, two enemies, one teammate. All entities tagged with `LevelEntity` + `IngameEntity` for cleanup on respawn.

### `pause.rs`
Escape key toggles `Paused` resource. Shows a pause overlay with Resume, Save Game (F5), Load Game (F9), Main Menu, and Quit buttons. Cursor grab mode is released when paused.

### `console.rs`
Developer console toggled with Backtick key. Supports commands: help, god, noclip, killall, timescale, spawn, tp. UI shows output history and input line.

### `camera_control.rs`
- `shoulder_swap_system` — Q key toggles camera shoulder
- `freelook_system` — Middle-mouse hold enables freelook
- `turn_rate_update_system` — Updates `SensitivityMultiplier` resource based on stance, weapon weight, stamina

### `settings.rs`
Loads `GameSettings` from `~/.socom/settings.ron` on startup, applies audio volume, saves defaults if no file exists.

### `save_load.rs`
RON-based save system. `SaveData` struct contains progression, stats, inventory, achievements, and settings. F5 saves, F9 loads. File stored at `~/.socom/save.ron`.

### `audio_relay.rs`
Bridges game messages to the kira audio host. Listens for `WeaponFiredMessage`, `HitConfirmedMessage`, and `EquipmentUsedMessage` and routes them to the SFX bus with appropriate sound file paths.
