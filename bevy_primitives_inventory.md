# Bevy ECS Primitives Inventory — SOCOM Game Codebase

> Generated from source at `C:\Users\ssdsk\projects\SOCOM`
> Date: 2026-06-20
> All line numbers refer to the `#[derive(...)]` or `pub fn` / `impl Plugin` line.

---

## `socom-core` (`crates/core/src`)

### Components (`#[derive(Component)]`)

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `components.rs` | 7 | `Player` | Marker for the player entity |
| 2 | `components.rs` | 17 | `MovementState` | Current movement stance (Standing/Sprinting/Crouching/Prone/InCover) |
| 3 | `components.rs` | 29 | `Health` | Health pool with armor, bleed-out, and revive support |
| 4 | `components.rs` | 87 | `Team` | Team affiliation (Player/Teammate/Enemy) |
| 5 | `components.rs` | 186 | `WeaponSlot` | Weapon slots for a character (primary + sidearm) |

### Resources (`#[derive(Resource)]`)

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `resources.rs` | 9 | `GameSettings` | Global game settings persisted between sessions (audio, controls, video) |
| 2 | `resources.rs` | 74 | `Paused` | Pause state — when true, gameplay systems are frozen |
| 3 | `resources.rs` | 81 | `SensitivityMultiplier` | Multiplier applied to mouse sensitivity each frame (stance/weapon/stamina modulated) |

### Run Conditions

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `resources.rs` | 92 | `is_not_paused` | Returns `true` when the game is NOT paused — uses with `.run_if()` on all gameplay systems |

---

## `socom-input` (`crates/input/src`)

### Plugins

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `lib.rs` | 11 | `InputPlugin` | Registers leafwing `InputManagerPlugin<PlayerAction>` and the default keybinding resource |

### Other (Actionlike enum, used as system param input)

| # | File | Line | Item | Description |
|---|------|------|------|-------------|
| 1 | `actions.rs` | 6 | `PlayerAction` (enum) | `Actionlike` enum of all player actions (Move, Look, Sprint, Crouch, Prone, Jump, Aim, Shoot, Reload, Melee, Interact, SwapPrimary, SwapSidearm, ShoulderSwap, etc.) |

---

## `socom-rendering` (`crates/rendering/src`)

### Components

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `camera.rs` | 71 | `ThirdPersonCamera` | Camera component: yaw/pitch, distance, shoulder, perspective, FOV, target entity |
| 2 | `post_processing.rs` | 12 | `PostProcessingProfile` | Marker for cameras that should receive post-processing effects |

### Resources

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `camera.rs` | 166 | `PerspectiveState` | Tracks the current camera perspective mode |

### Plugins

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `camera.rs` | 174 | `CameraPlugin` | Registers camera systems (look, follow, perspective toggle, FOV) |
| 2 | `post_processing.rs` | 23 | `PostProcessingPlugin` | Registers post-processing update system (tone mapping, bloom) |

### Systems

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `camera.rs` | ~359 | `camera_look_system` | Reads mouse motion, updates camera yaw/pitch with sensitivity modulation |
| 2 | `camera.rs` | ~269 | `camera_follow_system` | Positions camera: 3rd-person orbit or 1st-person eye level with collision avoidance |
| 3 | `camera.rs` | ~232 | `perspective_toggle_system` | Toggles 1st/3rd person on middle-mouse or V key |
| 4 | `camera.rs` | ~253 | `camera_fov_system` | Applies computed FOV based on perspective + ADS |
| 5 | `post_processing.rs` | 34 | `apply_post_processing_system` | Applies ACES filmic tone mapping to cameras with `PostProcessingProfile` |

---

## `socom-audio` (`crates/audio/src`)

### Components

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `footsteps.rs` | 17 | `FootstepTimer` | Tracks footstep audio cooldown per entity |

### Resources

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `kira_host.rs` | 40 | `KiraAudioState` | Holds kira audio handles, SFX/ambient/UI/voice bus state, mixer, and filter handles |

### Plugins

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `kira_host.rs` | 26 | `KiraHostPlugin` | Initialises the kira audio backend, spawns audio thread, registers startup/update systems |
| 2 | `footsteps.rs` | 23 | `FootstepPlugin` | Registers footstep audio systems |
| 3 | `ambient.rs` | 5 | `AmbientPlugin` | Registers ambient audio systems |
| 4 | `weapon_audio.rs` | 5 | `WeaponAudioPlugin` | Registers weapon audio relay systems (placeholder) |
| 5 | `lib.rs` | 12 | `AudioPlugin` | Meta-plugin that adds KiraHostPlugin + FootstepPlugin + AmbientPlugin + WeaponAudioPlugin |

---

## `socom-game` (`crates/game/src`)

### 1. MESSAGES (`#[derive(Message)]`)

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `combat/damage.rs` | 8 | `DamageMessage` | Fired when a bullet hits an entity; carries target, amount, source, hit_point, hit_normal |
| 2 | `combat/death.rs` | 11 | `DeathMessage` | Fired when an entity's health reaches zero |
| 3 | `messages.rs` | 9 | `WeaponFiredMessage` | Weapon was fired (shooter, weapon name, position, direction, hit status) |
| 4 | `messages.rs` | 18 | `PlayerDamagedMessage` | Player took damage (amount, source entity, hit point) |
| 5 | `messages.rs` | 25 | `HitConfirmedMessage` | Shot confirmed hit or miss (shooter, target, hit flag, hit point) |
| 6 | `messages.rs` | 35 | `XpGainedMessage` | Experience points were awarded |
| 7 | `messages.rs` | 40 | `LevelUpMessage` | Entity levelled up |
| 8 | `messages.rs` | 46 | `AchievementUnlockMessage` | Achievement was unlocked |
| 9 | `messages.rs` | 53 | `SquadStatusMessage` | Squad member status changed (alive/dead) |
| 10 | `messages.rs` | 61 | `CoverStateMessage` | Entity entered/exited cover |
| 11 | `messages.rs` | 67 | `SuppressionMessage` | Suppression level update for an entity |
| 12 | `messages.rs` | 76 | `ItemPickupMessage` | Item was picked up |
| 13 | `messages.rs` | 83 | `ItemEquipMessage` | Item was equipped or unequipped |
| 14 | `messages.rs` | 93 | `EquipmentUsedMessage` | Equipment (grenade/knife/medkit) was used |
| 15 | `messages.rs` | 101 | `GrenadeDetonatedMessage` | Grenade detonated (position, damage, radius) |
| 16 | `messages.rs` | 109 | `MeleeHitMessage` | Melee attack connected (attacker, target, damage) |
| 17 | `squad/orders.rs` | 19 | `SquadOrderMessage` | Squad order was issued (order type + source entity) |
| 18 | `combat/destruction/mod.rs` | 24 | `DestructionTransitionMessage` | Destructible entity changed destruction level (from → to state) |

### 2. COMPONENTS (`#[derive(Component)]`)

#### AI

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `ai/mod.rs` | 38 | `AiState` (enum) | AI state machine: Patrol / Alert / Engage |
| 2 | `ai/mod.rs` | 57 | `PatrolRoute` | List of waypoints an AI entity patrols between |
| 3 | `ai/mod.rs` | 79 | `VisionCone` | AI detection cone: FOV, range, suspicion level |

#### Combat

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `combat/damage.rs` | 29 | `Dead` | Marker for entities killed this frame (prevents double-processing) |
| 2 | `combat/death.rs` | 33 | `DeathScreenUI` | Marker for the death-screen UI text entity |
| 3 | `combat/impacts.rs` | 4 | `ImpactMarker` | Transient visual marker (bullet hole) that despawns after a timer |
| 4 | `combat/vfx.rs` | 21 | `VfxEffect` | Marker for a one-shot VFX particle effect with lifetime timer |
| 5 | `combat/vfx.rs` | 27 | `TracerProjectile` | Marker for a tracer projectile with lifetime timer |
| 6 | `combat/weapon_bob.rs` | 23 | `WeaponBobState` | Tracks weapon bob animation phase per weapon model |
| 7 | `combat/weapon_model.rs` | 9 | `WeaponModelRoot` | Root entity for the visible weapon model (child of camera) |
| 8 | `combat/weapon_model.rs` | 13 | `RifleModel` | Marker for the rifle model group |
| 9 | `combat/weapon_model.rs` | 17 | `PistolModel` | Marker for the pistol model group |
| 10 | `combat/weapon_model.rs` | 21 | `MuzzleFlashLight` | Marker for the weapon muzzle flash light entity |
| 11 | `combat/weapon_state.rs` | 7 | `WeaponState` | Per-entity weapon runtime state: ammo counts, fire cooldown, reload status |
| 12 | `combat/weapon_state.rs` | 42 | `OffhandWeaponState` | Persists the weapon state of the inactive weapon slot |

#### Controls / Stance

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `controls/stance.rs` | 16 | `StanceTransition` | Tracks stance transition state with timer |

#### Physics

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `physics/layers.rs` | 16 | `CharacterController` | Custom character controller: velocity, ground state, fall tracking |

#### Stamina / Breathing

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `stamina/mod.rs` | 27 | `Stamina` | Player stamina: current/max, regen timer, exhaustion flag |
| 2 | `breathing/mod.rs` | 7 | `Breathing` | Breath-hold mechanic: holding, hold timer, cooldown, steadiness |

#### Tactical

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `tactical/command_wheel.rs` | 66 | `CommandWheelUI` | Marker for the command wheel UI entity |
| 2 | `tactical/cover.rs` | 9 | `InCover` | Marks entity as being in cover with cover type and entity reference |
| 3 | `tactical/suppression.rs` | 8 | `Suppression` | Tracks suppression level per entity with decay timer |

#### Drones

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `drones/mod.rs` | 118 | `Drone` | Drone entity: type, battery, deployment, velocity, marked targets, ordnance counts |

#### Destruction

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `combat/destruction/mod.rs` | 63 | `DestructionState` | Per-entity structural destruction: level, health, material, bullet holes |
| 2 | `combat/destruction/debris.rs` | 11 | `Debris` | Marker for spawned debris entity with material and lifetime |
| 3 | `combat/destruction/glass.rs` | 14 | `GlassPane` | Glass pane with pre-scored crack lines for fracture effects |

#### Feedback / Visual

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `feedback/enemy_fx.rs` | 8 | `EnemyHurtFlash` | Marker for an enemy hurt-flash visual effect |
| 2 | `feedback/enemy_fx.rs` | 10 | `EnemyDeathParticle` | Marker for enemy death particle effect with velocity and timer |
| 3 | `feedback/hit_marker.rs` | 7 | `HitMarker` | Marker for the crosshair hit-marker UI element |
| 4 | `feedback/vignette.rs` | 4 | `DamageVignette` | Marker for the damage vignette overlay UI |

#### HUD

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `hud/elements.rs` | 4 | `HealthBarFill` | Marker for the health bar fill UI node |
| 2 | `hud/elements.rs` | 6 | `StanceText` | Marker for the stance indicator text |
| 3 | `hud/elements.rs` | 8 | `AmmoText` | Marker for the ammo counter text |
| 4 | `hud/elements.rs` | 10 | `Crosshair` | Marker for crosshair UI elements |
| 5 | `hud/elements.rs` | 12 | `WeaponNameText` | Marker for the weapon name text |
| 6 | `hud/elements.rs` | 14 | `HudElement` | Generic marker for any HUD entity (used for cleanup) |
| 7 | `hud/kill_feed.rs` | 5 | `KillFeedEntry` | Marker for a kill-feed message entity |
| 8 | `hud/squad_status.rs` | 6 | `SquadStatusText` | Marker for the squad status text UI |
| 9 | `hud/squad_status.rs` | 55 | `ObjectiveText` | Marker for the mission objective text UI |
| 10 | `hud/stamina_bar.rs` | 6 | `StaminaBarFill` | Marker for the stamina bar fill UI node |
| 11 | `hud/xp_notification.rs` | 4 | `XpPopup` | Marker for XP gain popup text entity |
| 12 | `hud/xp_notification.rs` | 6 | `LevelUpPopup` | Marker for level-up popup text entity |
| 13 | `hud/achievement_popup.rs` | 4 | `AchievementPopup` | Marker for achievement unlock popup text entity |

#### Menu / UI / State

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `console.rs` | 57 | `ConsoleUI` | Marker for the developer console UI container |
| 2 | `console.rs` | 59 | `ConsoleOutputText` | Marker for the console output text entity |
| 3 | `console.rs` | 61 | `ConsoleInputText` | Marker for the console input text entity |
| 4 | `menu/mod.rs` | 50 | `MenuUI` | Marker for main menu UI entities |
| 5 | `menu/mod.rs` | 53 | `MenuButton` (enum) | Button variant enum for main menu (NewGame, Settings, Controls, Quit) |
| 6 | `menu/settings.rs` | 117 | `SettingsPageUI` | Marker for settings page UI entities |
| 7 | `menu/keybinds.rs` | 74 | `KeybindsPageUI` | Marker for keybinds page UI entities |
| 8 | `pause.rs` | 46 | `PauseOverlay` | Marker for the pause overlay UI container |
| 9 | `pause.rs` | 48 | `PauseMenuButton` (enum) | Button variant enum for pause menu (Resume, SaveGame, LoadGame, MainMenu, Quit) |
| 10 | `states/ingame.rs` | 39 | `IngameEntity` | Marker for in-game entities (auto-despawned on state exit) |
| 11 | `states/loading.rs` | 18 | `LoadingUI` | Marker for loading screen UI entities |
| 12 | `states/loading.rs` | 21 | `LoadingBarFill` | Marker for loading bar fill UI node |
| 13 | `level.rs` | 10 | `LevelEntity` | Marker for level geometry/entities (despawned on respawn) |

### 3. RESOURCES (`#[derive(Resource)]`)

| # | File | Line | Struct | Description |
|---|------|------|--------|-------------|
| 1 | `console.rs` | 5 | `ConsoleState` | Dev console state: open/closed, input text, history, output |
| 2 | `combat/death.rs` | 17 | `RespawnState` | Tracks the player's death-respawn cycle (timer, is_dead flag) |
| 3 | `combat/vfx.rs` | 36 | `VfxAssets` | Cached handles to all particle effect assets (muzzle_flash, bullet_impact, etc.) |
| 4 | `combat/weapon_bob.rs` | 12 | `AdsState` | Shared ADS factor (0–1), spread_mult, and speed_mult — updated each frame |
| 5 | `combat/destruction/penetration.rs` | 26 | `MaterialPenetrationTable` | Lookup table mapping (MaterialType × Caliber) → PenetrationResult |
| 6 | `drones/mod.rs` | 189 | `DroneState` | Tracks which drone types are currently active (recon, FPV, grenade, mine) |
| 7 | `gear/inventory.rs` | 6 | `PlayerInventory` | Player's equipped gear (5 slots) + stash + credits |
| 8 | `gear/equipment_inventory.rs` | 26 | `EquipmentInventory` | Player's carried equipment (5 slots for throwable/deployable/melee items) |
| 9 | `gear/workshop.rs` | 4 | `WeaponWorkshop` | Weapon attachment workshop state: fitted attachments, library, UI open flag |
| 10 | `menu/mod.rs` | 18 | `MenuState` | Current menu page and settings subpage |
| 11 | `missions/mod.rs` | 58 | `MissionState` | Current mission: objectives list, completion status, name, briefing |
| 12 | `progression/xp.rs` | 19 | `PlayerProgression` | Tracks the player's level, XP, and total XP earned |
| 13 | `progression/stats.rs` | 10 | `PlayerStats` | Comprehensive player statistics (kills, deaths, shots, damage, etc.) |
| 14 | `progression/achievements.rs` | 45 | `AchievementTracker` | Tracks earned and in-progress achievements |
| 15 | `progression/specializations.rs` | 51 | `PlayerSpecialization` | Tracks the player's chosen specialization |
| 16 | `squad/orders.rs` | 26 | `ActiveOrders` | Tracks currently active orders per squad member |
| 17 | `tactical/command_wheel.rs` | 5 | `CommandWheelState` | Command wheel state: open/closed, selected index |

### 4. SYSTEMS — Public System Functions

> All listed functions take at least one Bevy system parameter (`Commands`, `Query`, `Res`, `ResMut`, `MessageReader`, `MessageWriter`, etc.).

#### AI

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `ai/enemy.rs` | 79 | `patrol_system` | Moves enemies between patrol waypoints |
| 2 | `ai/enemy.rs` | 146 | `detection_system` | Detects player within vision cone, builds suspicion |
| 3 | `ai/enemy.rs` | 226 | `engage_system` | Engages detected enemies with weapon fire |
| 4 | `ai/enemy.rs` | 348 | `enemy_death_system` | Despawns enemy entities when health reaches zero |
| 5 | `ai/teammate.rs` | 68 | `teammate_follow_system` | Makes teammate follow the player |
| 6 | `ai/teammate.rs` | 124 | `teammate_combat_system` | Teammate detects and engages enemies |
| 7 | `ai/teammate.rs` | 193 | `teammate_reload_system` | Teammate reloads weapon when magazine is empty |

#### Audio Relay

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `audio_relay.rs` | 33 | `weapon_fire_audio_relay` | Routes weapon fire messages to kira SFX bus |
| 2 | `audio_relay.rs` | 51 | `weapon_hit_audio_relay` | Routes hit confirmation messages to kira SFX bus |
| 3 | `audio_relay.rs` | 63 | `equipment_audio_relay` | Routes equipment usage messages to kira SFX bus |

#### Breathing

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `breathing/mod.rs` | 25 | `breathing_system` | Handles breath-hold mechanic (Aim+Sprint), drains stamina, increases steadiness |

#### Camera Control

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `camera_control.rs` | 33 | `turn_rate_update_system` | Updates sensitivity multiplier based on stance, weapon weight, stamina |
| 2 | `camera_control.rs` | 44 | `shoulder_swap_system` | Toggles camera shoulder on Q key |
| 3 | `camera_control.rs` | 65 | `freelook_system` | Toggles freelook on middle-mouse hold |

#### Combat — Damage / Death

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `combat/damage.rs` | 17 | `apply_damage_system` | Reads DamageMessages and subtracts health from target entity |
| 2 | `combat/damage.rs` | 36 | `death_check_system` | Detects zero HP, triggers bleed-out first, then final death |
| 3 | `combat/death.rs` | 38 | `handle_player_death` | Listens for DeathMessage targeting player, starts respawn timer, shows death screen |
| 4 | `combat/death.rs` | 71 | `respawn_system` | After respawn delay, resets player to spawn with full health + ammo |

#### Combat — Destructible Environment

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `combat/destruction/damage.rs` | 21 | `apply_explosion_damage_system` | Reads GrenadeDetonatedMessage, applies spherical falloff damage to destructibles |
| 2 | `combat/destruction/damage.rs` | 59 | `destruction_state_machine_system` | Transitions Pristine→Damaged→Breached→Destroyed based on structural HP |
| 3 | `combat/destruction/damage.rs` | 118 | `collapse_animation_system` | Applies downward velocity + dust on Destroyed transition |
| 4 | `combat/destruction/debris.rs` | 21 | `spawn_debris_for_transition` | Spawns debris entities when a structure is breached/destroyed |
| 5 | `combat/destruction/debris.rs` | 122 | `debris_lifetime_system` | Despawns debris entities after their lifetime expires |
| 6 | `combat/destruction/glass.rs` | 63 | `glass_fracture_system` | Handles glass fracture patterns on Damage transition |
| 7 | `combat/destruction/glass.rs` | 133 | `glass_debris_lifetime_system` | Manages glass shard cleanup after shatter |
| 8 | `combat/destruction/penetration.rs` | 166 | `bullet_penetration_system` | Computes bullet penetration through materials via lookup table |
| 9 | `combat/destruction/vehicles.rs` | 67 | `vehicle_damage_state_system` | Updates vehicle damage state machine based on health ratio |

#### Combat — Shooting / Reload / Impacts

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `combat/shooting.rs` | 61 | `shooting_system` | Main hitscan shooting system with spread, raycast, damage writing |
| 2 | `combat/reload.rs` | 12 | `weapon_swap_system` | Handles 1/2 key presses to swap between primary and sidearm |
| 3 | `combat/reload.rs` | 47 | `reload_input_system` | Manual reload triggered by pressing R |
| 4 | `combat/reload.rs` | 80 | `reload_tick_system` | Ticks down reload timer, refills magazine on completion |
| 5 | `combat/impacts.rs` | 10 | `impact_lifetime_system` | Despawns ImpactMarker entities when their timer expires |

#### Combat — VFX

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `combat/vfx.rs` | 46 | `build_vfx_assets` | Builds all VFX effect assets (called once during plugin setup) |
| 2 | `combat/vfx.rs` | 217 | `muzzle_flash_system` | Spawns muzzle flash particle at weapon firing position |
| 3 | `combat/vfx.rs` | 234 | `bullet_impact_system` | Spawns bullet impact particle at hit location |
| 4 | `combat/vfx.rs` | 253 | `hit_marker_system` | Spawns hit marker particle on confirmed hits |
| 5 | `combat/vfx.rs` | 270 | `death_explosion_system` | Spawns death explosion particle effect |
| 6 | `combat/vfx.rs` | 290 | `tracer_system` | Spawns tracer projectile visual |
| 7 | `combat/vfx.rs` | 313 | `cleanup_vfx_system` | Despawns VfxEffect entities when timer expires |
| 8 | `combat/vfx.rs` | 327 | `cleanup_tracer_system` | Despawns TracerProjectile entities when timer expires |

#### Combat — Weapon Model / Weapon Bob

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `combat/weapon_model.rs` | 141 | `spawn_weapon_model` | Spawns procedural rifle + pistol 3D models as children of camera |
| 2 | `combat/weapon_model.rs` | 199 | `weapon_model_swap_system` | Swaps visible weapon model on slot change |
| 3 | `combat/weapon_model.rs` | 226 | `weapon_shoulder_mirror_system` | Mirrors weapon model to correct shoulder side |
| 4 | `combat/weapon_model.rs` | 241 | `weapon_model_flash_system` | Toggles muzzle flash light on fire |
| 5 | `combat/weapon_bob.rs` | 56 | `weapon_bob_system` | Animates weapon bob and updates shared ADS state resource |
| 6 | `combat/weapon_bob.rs` | 102 | `ads_fov_system` | Applies ADS FOV modifier to camera |

#### Controls

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `controls/stance.rs` | 47 | `stance_transition_system` | Handles stance input with realistic transition timers |

#### Console

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `console.rs` | 73 | `console_toggle_system` | Toggles dev console on backtick key |
| 2 | `console.rs` | 91 | `console_input_system` | Handles keyboard input for the dev console |

#### Drones

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `drones/mod.rs` | 298 | `drone_control_system` | Handles recon/FPV drone input, movement, battery, and camera |
| 2 | `drones/mod.rs` | 475 | `apply_drone_explosion` | Applies explosive damage when FPV drone detonates |
| 3 | `drones/mod.rs` | 507 | `grenade_drone_system` | Controls grenade drone: ordnance drop, targeting, battery |
| 4 | `drones/mod.rs` | 628 | `mine_drone_system` | Controls mine drone: mine deployment, pattern, battery |

#### Feedback / Visual Effects

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `feedback/enemy_fx.rs` | 15 | `enemy_hurt_flash_system` | Spawns red flash at enemy position on damage |
| 2 | `feedback/enemy_fx.rs` | 43 | `hurt_flash_lifetime_system` | Despawns enemy hurt flash after duration |
| 3 | `feedback/enemy_fx.rs` | 67 | `enemy_death_effect_system` | Spawns death particles on enemy death |
| 4 | `feedback/enemy_fx.rs` | 106 | `death_particle_lifetime_system` | Despawns death particles after duration |
| 5 | `feedback/hit_marker.rs` | 15 | `hit_marker_detect_system` | Spawns "X" hit marker UI on enemy damage |
| 6 | `feedback/hit_marker.rs` | 55 | `hit_marker_lifetime_system` | Despawns hit marker after duration |
| 7 | `feedback/vignette.rs` | 9 | `damage_vignette_system` | Shows red vignette overlay based on health ratio |

#### Gear — Equipment

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `gear/equipment_inventory.rs` | 95 | `select_equipment_system` | Cycles equipment selection on input |
| 2 | `gear/deployable.rs` | 15 | `deploy_equipment_system` | Deploys equipment (claymore, etc.) on use |
| 3 | `gear/deployable.rs` | 122 | `deployable_arm_system` | Arms deployables after a delay |
| 4 | `gear/deployable.rs` | 135 | `claymore_detonation_system` | Detonates claymores on enemy proximity |
| 5 | `gear/healing.rs` | 14 | `self_heal_system` | Heals player when using medkit/bandage |
| 6 | `gear/healing.rs` | 64 | `bleed_out_system` | Ticks bleed-out timer for downed entities |
| 7 | `gear/healing.rs` | 85 | `revive_system` | Revives downed teammate on interaction |
| 8 | `gear/throwable.rs` | 15 | `throw_equipment_system` | Throws grenades and other throwable items |
| 9 | `gear/throwable.rs` | 141 | `fuse_timer_system` | Ticks grenade fuse timers and detonates |
| 10 | `gear/throwable.rs` | 165 | `c4_detonation_system` | Detonates C4 on input after arming |
| 11 | `gear/melee.rs` | 17 | `melee_attack_system` | Performs knife lunge melee attack |
| 12 | `gear/inventory.rs` | 52 | `track_damage_for_loot` | Reads damage messages for future loot drops |
| 13 | `gear/workshop.rs` | 61 | `weapon_modification_system` | Handles weapon workshop attachment UI/input |
| 14 | `gear/mod.rs` | 49 | `apply_workshop_to_weapon_system` | Applies workshop attachment modifiers to active weapon stats |

#### HUD

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `hud/elements.rs` | 17 | `spawn_hud` | Spawns all HUD elements (health bar, stance, ammo, crosshair, weapon name) |
| 2 | `hud/elements.rs` | 157 | `cleanup_hud` | Despawns all HUD entities |
| 3 | `hud/systems.rs` | 6 | `update_health_bar` | Updates health bar width based on current HP |
| 4 | `hud/systems.rs` | 19 | `update_stance_text` | Updates stance indicator text |
| 5 | `hud/systems.rs` | 38 | `update_ammo_text` | Updates ammo counter text |
| 6 | `hud/systems.rs` | 55 | `update_weapon_name` | Updates weapon name display |
| 7 | `hud/systems.rs` | 71 | `crosshair_visibility_system` | Hides crosshair when aiming down sights |
| 8 | `hud/kill_feed.rs` | 8 | `kill_feed_system` | Shows kill/death messages in the kill feed |
| 9 | `hud/kill_feed.rs` | 50 | `kill_feed_cleanup_system` | Despawns kill feed entries after timeout |
| 10 | `hud/xp_notification.rs` | 9 | `xp_notification_system` | Shows XP gain popup |
| 11 | `hud/xp_notification.rs` | 38 | `level_up_notification_system` | Shows level-up popup |
| 12 | `hud/xp_notification.rs` | 67 | `popup_lifetime_system` | Despawns XP/level-up popups after duration |
| 13 | `hud/squad_status.rs` | 9 | `spawn_squad_status` | Spawns squad status text UI |
| 14 | `hud/squad_status.rs` | 28 | `update_squad_status_system` | Updates squad member status and orders display |
| 15 | `hud/squad_status.rs` | 58 | `spawn_objective_text` | Spawns mission objective text UI |
| 16 | `hud/squad_status.rs` | 79 | `update_objective_text_system` | Updates mission objective progress text |
| 17 | `hud/stamina_bar.rs` | 9 | `spawn_stamina_bar` | Spawns stamina bar UI |
| 18 | `hud/stamina_bar.rs` | 39 | `update_stamina_bar` | Updates stamina bar width based on current stamina |
| 19 | `hud/achievement_popup.rs` | 7 | `achievement_popup_system` | Shows achievement unlock popup |
| 20 | `hud/achievement_popup.rs` | 36 | `ach_popup_lifetime_system` | Despawns achievement popup after duration |

#### Level

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `level.rs` | 33 | `spawn_test_level` | Spawns the greybox test level with geometry, enemies, and teammate |

#### Menu

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `menu/mod.rs` | 61 | `setup_main_menu` | Resets menu state to main page on state enter |
| 2 | `menu/mod.rs` | 65 | `release_cursor` | Releases cursor grab on menu enter |
| 3 | `menu/mod.rs` | 72 | `cleanup_menu` | Despawns menu UI entities on state exit |
| 4 | `menu/mod.rs` | 78 | `menu_navigation_system` | Handles button interactions in the menu |
| 5 | `menu/mod.rs` | 109 | `main_menu_ui_system` | Spawns UI for current menu page |
| 6 | `menu/settings.rs` | 13 | `spawn_settings_page` | Spawns settings page UI |
| 7 | `menu/keybinds.rs` | 5 | `spawn_keybinds_page` | Spawns keybinds page UI |

#### Missions

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `missions/mod.rs` | 88 | `update_objectives_system` | Updates mission objective progress on enemy kills |
| 2 | `missions/mod.rs` | 103 | `check_mission_system` | Checks if all objectives are completed |

#### Pause

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `pause.rs` | 18 | `toggle_pause_system` | Toggles pause on Escape key |
| 2 | `pause.rs` | 42 | `resume_from_pause` | Ensures paused is false on entering InGame state |
| 3 | `pause.rs` | 57 | `pause_menu_ui_system` | Manages pause overlay spawn/despawn and button interactions |

#### Physics

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `physics/player_movement.rs` | 63 | `player_movement_system` | Reads input + stance, computes velocity, applies MoveAndSlide, handles fall damage |
| 2 | `physics/enemy_movement.rs` | 25 | `enemy_movement_system` | Applies gravity and MoveAndSlide to non-player AI entities |
| 3 | `physics/stance.rs` | 6 | `player_stance_system` | Maps button presses directly to stance changes |

#### Progression

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `progression/xp.rs` | ~57 | `xp_event_listener` | Listens for kill/damage messages to award XP |
| 2 | `progression/stats.rs` | ~44 | `damage_event_listener` | Tracks damage dealt/taken and shots fired/hit |
| 3 | `progression/stats.rs` | ~76 | `death_event_listener` | Tracks kill/death counts |
| 4 | `progression/achievements.rs` | 57 | `achievement_checker` | Checks for achievement unlocks when stats change |

#### Save / Load

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `save_load.rs` | 125 | `auto_save_system` | Auto-saves on F5 key press |
| 2 | `save_load.rs` | 138 | `quick_load_system` | Loads save data on F9 key press |

#### Settings

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `settings.rs` | 18 | `load_settings` | Attempts to load settings from disk, falls back to defaults |
| 2 | `settings.rs` | 31 | `apply_audio_volume` | Applies master volume to all active audio sources |
| 3 | `settings_applier.rs` | 5 | `apply_settings_system` | Applies fullscreen/windowed mode settings |

#### Squad

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `squad/orders.rs` | 40 | `squad_order_dispatch_system` | Reads SquadOrderMessage and sets ActiveOrders |
| 2 | `squad/formation.rs` | 14 | `squad_formation_system` | Moves teammates to formation positions relative to player |

#### Stamina

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `stamina/mod.rs` | 56 | `stamina_system` | Updates stamina every frame based on movement state (sprint drain, regen) |

#### Tactical

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `tactical/command_wheel.rs` | 26 | `command_wheel_input_system` | Opens/closes command wheel on Tab, selects orders on 1–4 |
| 2 | `tactical/command_wheel.rs` | 69 | `command_wheel_ui_system` | Spawns command wheel radial UI when open |
| 3 | `tactical/cover.rs` | 27 | `cover_detection_system` | Detects nearby walls and marks player as in cover |
| 4 | `tactical/suppression.rs` | 24 | `suppression_system` | Applies suppression when player takes damage, emits SuppressionMessage |
| 5 | `tactical/suppression.rs` | 52 | `suppression_fx_system` | Decays suppression level over time |

#### State Transitions

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `states/loading.rs` | ~24 | `setup_loading_screen` | Spawns loading screen UI |
| 2 | `states/loading.rs` | ~94 | `check_loading_complete` | Ticks loading bar and transitions to InGame after delay |
| 3 | `states/ingame.rs` | ~42 | `setup_ingame` | Spawns directional + ambient light for the level |
| 4 | `states/ingame.rs` | ~65 | `cleanup_ingame` | Despawns all IngameEntity-tagged entities on state exit |
| 5 | `states/ingame.rs` | ~74 | `capture_cursor` | Locks and hides cursor when entering InGame state |
| 6 | `states/ingame.rs` | ~84 | `release_cursor` | Releases cursor when exiting InGame state |

#### Weapon Handling

| # | File | Line | Function | Description |
|---|------|------|----------|-------------|
| 1 | `weapon_handling/mod.rs` | 82 | `weapon_handling_system` | Computes weapon weight speed/ADS/sway multipliers based on equipped weapon |

### 5. PLUGINS

| # | File | Line | Plugin | Description |
|---|------|------|--------|-------------|
| 1 | `ai/mod.rs` | 17 | `AiPlugin` | Registers enemy + teammate AI systems |
| 2 | `ai/teammate.rs` | 53 | `TeammatePlugin` | Registers teammate follow/combat/reload systems |
| 3 | `audio_relay.rs` | 17 | `AudioRelayPlugin` | Registers audio relay systems bridging messages to kira |
| 4 | `breathing/mod.rs` | 56 | `BreathingPlugin` | Registers the breathing system |
| 5 | `camera_control.rs` | 15 | `CameraControlPlugin` | Registers shoulder swap, freelook, and turn-rate systems |
| 6 | `combat/mod.rs` | 37 | `CombatPlugin` | Registers all combat systems (damage, death, shooting, reload, VFX, weapon model) |
| 7 | `combat/destruction/mod.rs` | 130 | `DestructionPlugin` | Registers destruction systems (explosion, state machine, debris, glass, vehicles) |
| 8 | `combat/vfx.rs` | 346 | `VfxPlugin` | Registers VFX particle systems (muzzle flash, impact, tracer, etc.) |
| 9 | `console.rs` | 64 | `ConsolePlugin` | Registers dev console toggle + input systems |
| 10 | `controls/mod.rs` | 6 | `ControlsPlugin` | Registers stance transition system |
| 11 | `drones/mod.rs` | 754 | `DronePlugin` | Registers all drone systems (recon, FPV, grenade, mine) |
| 12 | `feedback/mod.rs` | 9 | `FeedbackPlugin` | Registers feedback systems (hit marker, vignette, enemy FX) |
| 13 | `gear/mod.rs` | 18 | `GearPlugin` | Registers all gear systems (equipment, inventory, workshop, healing, melee, throwable) |
| 14 | `hud/mod.rs` | 12 | `HudPlugin` | Registers all HUD systems (spawn, update, cleanup) |
| 15 | `level.rs` | 14 | `LevelPlugin` | Registers level spawn system on InGame state enter |
| 16 | `menu/mod.rs` | 33 | `MenuPlugin` | Registers main menu systems |
| 17 | `missions/mod.rs` | 107 | `MissionPlugin` | Registers mission objective tracking systems |
| 18 | `pause.rs` | 8 | `PausePlugin` | Registers pause toggle + menu UI systems |
| 19 | `physics/mod.rs` | 12 | `PhysicsPlugin` | Registers player + enemy movement and stance systems |
| 20 | `player.rs` | 86 | `PlayerPlugin` | Registers player spawn/despawn on InGame state |
| 21 | `progression/mod.rs` | 8 | `ProgressionPlugin` | Registers XP, stats, and achievement systems |
| 22 | `settings.rs` | 7 | `SettingsPlugin` | Registers settings load/save and audio volume systems |
| 23 | `squad/mod.rs` | 6 | `SquadPlugin` | Registers squad order dispatch and formation systems |
| 24 | `states/ingame.rs` | 15 | `InGamePlugin` | Meta-plugin bundling Player, Physics, Level, Combat, Tactical, Squad, Ai, and HUD plugins |
| 25 | `states/loading.rs` | 5 | `LoadingPlugin` | Registers loading screen UI and progress systems |
| 26 | `states/main_menu.rs` | 6 | `MainMenuPlugin` | Thin wrapper delegating to MenuPlugin |
| 27 | `tactical/mod.rs` | 7 | `TacticalPlugin` | Registers command wheel, cover, and suppression systems |

---

## Summary Totals

| Primitive | `socom-core` | `socom-input` | `socom-rendering` | `socom-audio` | `socom-game` | **Total** |
|-----------|:-----------:|:-------------:|:-----------------:|:-------------:|:------------:|:--------:|
| **Messages** | 0 | 0 | 0 | 0 | 18 | **18** |
| **Components** | 5 | 0 | 2 | 1 | 52 | **60** |
| **Resources** | 3 | 0 | 1 | 1 | 17 | **22** |
| **Systems** | 0 | 0 | 5 | 0 | ~120 | **~125** |
| **Plugins** | 0 | 1 | 2 | 5 | 27 | **35** |
