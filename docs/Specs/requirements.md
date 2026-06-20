---
title: "SOCOM Tactical Shooter — Product Requirements Document"
version: "3.1.0"
date: "2026-06-19"
author: "Sam / Quicksilver"
status: "Active — Phase 3 Active, VFX + Post-Processing Complete"
license: "MIT"
---

# Product Requirements Document

## 1. Executive Summary

SOCOM Tactical Shooter is a modern reimagining of the classic PS2 tactical shooter rebuilt for PC with AAA production values using Rust and Bevy 0.18.1. It targets authentic squad-based tactical combat inspired by Arma 3, Squad, and Ghost Recon.

**Current Status:** All core gameplay systems implemented and compiling (0 errors, 0 clippy errors). Phase 3 active: VFX (5 bevy_hanabi effects) and post-processing (ACES + Bloom) complete. Pre-asset — awaiting audio files, 3D models.

**Development Philosophy:** Enterprise-grade architecture with zero Bevy dependency in core crate, message-driven inter-system communication, single-responsibility files (max ~200 lines), complete systems built before asset integration.

## 2. Target Audience & Personas

| Persona | Description | Goals |
|---------|-------------|-------|
| SOCOM Veteran (Primary) | Age 30-45, played original SOCOM on PS2 | Over-shoulder camera, squad commands, tactical pacing, nostalgia |
| Tactical Shooter Fan (Primary) | Plays Arma, Squad, Hell Let Loose | Realistic movement, suppression, stamina management, weapon weight |
| Competitive Player (Secondary) | Plays R6 Siege, CS2 | Round-based modes (5v5), ranked play, weapon balance, skill gap |
| Casual Player (Secondary) | Limited time, wants pick-up-and-play | Training range, bot matches, quick play, simple controls |
| MilSim Enthusiast (Tertiary) | Authentic military simulation | Realistic ballistics, gear weight, team coordination, stamina |
| Modder (Tertiary) | Wants to create custom content | GLTF scene loading, RON weapon configs, SDK documentation |
| Developer/Contributor | Open source contributor | Clean architecture, CI/CD, test coverage, modular code |
| Content Creator | Streams/makes content | Spectator mode, replay system, cinematic camera tools |

## 3. Comprehensive User Stories

### Movement & Controls
- As a SOCOM veteran, I want over-shoulder third-person camera with shoulder swap (Q) for authentic feel.
- As an Arma player, I want to toggle between 1st and 3rd person (V/MMB) with smooth interpolation.
- As a tactical fan, I want realistic stance transitions (0.3s crouch, 0.8s prone) so positioning has weight.
- As a competitive player, I want responsive controls with zero input lag so gunfights feel fair.
- As a disabled player, I want fully rebindable controls and adjustable sensitivity.
- As a MilSim fan, I want weapon weight to affect movement speed (0.55x-1.0x) and turn rate.
- As a speedrunner, I want consistent deterministic physics (120Hz fixed timestep).

### Combat & Weapons
- As a SOCOM veteran, I want visible weapon model in 1st/3rd person with muzzle flash.
- As a gun enthusiast, I want 8-component modular weapons (chassis x caliber x barrel x sight x grip x mag x stock) with real stat tradeoffs.
- As a competitive player, I want balanced weapons with clear TTK values.
- As a MilSim fan, I want 4 ammo types (FMJ/HP/AP/Tracer) with different damage/penetration/spread.
- As a modder, I want weapons defined in RON config files for custom content.

### Health & Damage
- As a tactical fan, I want no health regen — permanent damage until medic revives.
- As a player, I want visual damage feedback (red vignette, directional indicator, sound).
- As a MilSim fan, I want limb damage multipliers (headshot=instant kill, leg hit=slow movement).
- As a player, I want suppression to blur vision and increase weapon sway under fire.

### AI & Teammates
- As a SOCOM veteran, I want command wheel with 4 orders: Move, Engage, Suppress, Regroup.
- As a single-player fan, I want AI teammates that take cover, flank enemies, and revive me.
- As a player, I want enemies with realistic vision cones (120deg, 40m), patrol routes, suppression reactions.

### Drones
- As a modern warfare fan, I want Recon UAV (U key) for scouting with battery management.
- As a tactical player, I want FPV Strike Drone (J key) for precision explosive attacks.
- As a player, I want camera-relative drone controls with velocity-based flight physics.

### Progression & Gear
- As an RPG fan, I want XP (50/kill), leveling (100xp x level), unlockable weapons/attachments.
- As a completionist, I want 7 achievements and 5 gear rarities (Common through Legendary).
- As a tactical fan, I want 4 specializations (Assault/Medic/Engineer/Recon) with real gameplay impact.

### Game Modes
- As a competitive player, I want TDM (5v5, no-respawn, first to 5 rounds, best of 9).
- As a tactical fan, I want Demolition (attack/defend bomb sites, halftime switch after 4 rounds).
- As a SOCOM veteran, I want CTF (1-life, capture flag OR eliminate all enemies to win round).
- As a casual player, I want Training Range with any weapon vs static/moving targets.
- As a solo player, I want SP + Bots in all modes.
- As a player, I want 5-6 minute rounds with best-of-9 format (first to 5 round wins).

### Multiplayer
- As a competitive player, I want dedicated servers, low-latency netcode (<50ms).
- As a party player, I want to squad up with friends, join lobbies, use voice chat.
- As a player, I want skill-based matchmaking with persistent profile (K/D, wins, rank).

### UI & UX
- As a player, I want clean HUD (health, stamina, ammo, stance, crosshair, minimap).
- As a new player, I want interactive tutorial for movement, combat, squad commands, drones.
- As a player, I want main menu: Play (mode select), Training, Loadout/Gunsmith, Settings.
- As a player, I want video (resolution/fullscreen/quality), audio (master/SFX/music/voice), controls (sensitivity/keybinds).
- As a player, I want visual feedback: kill feed, damage vignette, XP popups, hit markers.

## 4. Game Modes Specification

### 4.1 Training Range
| Property | Specification |
|----------|--------------|
| Players | 1 (solo practice) |
| Time Limit | Unlimited |
| Respawns | Immediate, infinite |
| Targets | Static silhouettes (10m/25m/50m), pop-up (reaction timer), moving lateral (3m/s @ 20m), moving advancing (2m/s @ 30m) |
| Weapon Locker | All weapons + all attachments unlocked |
| Features | Ammo regeneration, distance markers every 10m, accuracy/DPS tracker, drone recharge station |

### 4.2 Team Deathmatch (No Respawn)
| Property | Specification |
|----------|--------------|
| Players | 5v5 (competitive) / 4v4 / 3v3 |
| Round Time | 5-6 minutes (configurable in custom games) |
| Win Condition | Eliminate all enemies OR last team standing when timer expires |
| Match Win | First to 5 round wins (best of 9) |
| Respawns | None per round. Next round = fresh |
| Overtime | If tied 4-4, sudden death round (no time limit) |
| Economy | Round-based purchase from performance (kills, assists, obj) |

### 4.3 Demolition (Attack / Defend)
| Property | Specification |
|----------|--------------|
| Players | 5v5 |
| Round Time | 5-6 minutes |
| Attackers | Plant bomb at A or B site, defend until detonation (45s fuse) |
| Defenders | Prevent plant OR defuse (7s) after plant OR eliminate all attackers |
| Halftime | Switch sides after 4 rounds |
| Match Win | First to 5 rounds (first to 9 in competitive overtime) |
| Win (Attack) | Bomb detonates OR all defenders eliminated |
| Win (Defend) | Bomb defused OR time expires OR all attackers eliminated |

### 4.4 Capture the Flag (1 Life)
| Property | Specification |
|----------|--------------|
| Players | 5v5 |
| Round Time | 5-6 minutes |
| Objective | Capture enemy flag and return to your base. Prevent enemy from capturing yours |
| Match Win | First to 5 flag captures OR first to 5 round wins |
| Win | Flag carrier reaches own base OR all enemies eliminated |
| Flag Rules | Carrier drops flag on death. 10s team cooldown before same team can pick up |

### 4.5 Equipment & Throwables Specification

All equipment selected in pre-round loadout, assigned to Utility gear slot. Cycle with G tap, deploy with G hold. Limited uses per life. All stats are grounded in real-world equivalents.

#### Throwables
| Equipment | Fuse | Effect Radius | Duration | Damage | Max Carry | Description |
|-----------|------|---------------|----------|--------|-----------|-------------|
| Fragmentation Grenade | 4.0s (3.5-4.5s random) | 5m kill / 15m casualty | Instant | 100->5 falloff | 2 | M67-style. Cooking possible (hold before throw). 30mm fragment pen through thin cover. Cook 2-3s for airburst |
| Smoke Grenade | 1.5s (on surface contact) | 8m cloud | 15s | None | 2 | M18-style. Blocks LOS. Thermal sees partially through. Can fake objectives |
| Flashbang | 1.5s | 10m blind / 5m deafen | Blind 3s, deafen 5s | 0 | 2 | M84-style. Facing away = 1s blind. Total white screen + 95dB ringing. Disorients aim 1s |
| Stun Grenade | 1.5s | 8m | Slow 5s, blur 5s | 0 | 2 | Movement -50%, turn rate -70%, spread x3. Blur distorts vision |
| Impact Grenade | Contact | 3m kill / 8m casualty | Instant | 150->10 falloff | 1 | M72-style. No cooking. Predictable, smaller radius, higher direct |

#### Deployables
| Equipment | Trigger | Blast | Damage | Arm | Max | Description |
|-----------|---------|-------|--------|-----|-----|-------------|
| Claymore Mine | Tripwire laser | 6m cone x 2m wide | 200->20 falloff | 3s | 2 | M18A1. Directional, 700 steel balls. Shot to detonate. NVG sees laser |
| AP Mine | Pressure plate | 4m radius | 150->10 falloff | 2s | 2 | M14-style. Buried (slight bulge). Diffuse 5s hold E. Triggers on any enemy |
| C4 Charge | Remote detonator (F) | 8m radius | 500->20 falloff | Instant | 2 | M112. Any surface. Wall breach: 2m hole. Vehicle: instant disable. Shot to detonate |
| Proximity Sensor | Enemy in 10m | Detection only | N/A | Instant | 2 | Alerts minimap. Ground (footsteps) or wall (door breach). 60s battery |

#### Melee Weapons
| Weapon | Primary | Heavy (1.2s windup) | Throw | Description |
|--------|---------|---------------------|-------|-------------|
| Combat Knife | 50 dmg (2-hit body) | 100 dmg, instant kill from behind/headshot, 20m lunge | N/A | M9/Ka-Bar. Fast: 0.3s. Inspection spin |
| Throwing Knife | 50 dmg body | 100 dmg headshot | Silent projectile, 15m effective, 30m max, retrievable | 3 carried. Hold E to retrieve from surface/corpse |

### 4.6 Medic & Healing System

**No health regen.** Damage is permanent. Healing takes time and resources. Headshots = instant kill (no revive possible).

#### Healing Items
| Item | Heal | Use Time | Animation | Max Carry | Cost |
|------|------|----------|-----------|-----------|------|
| Bandage | 25 HP | 3s | One-hand wrap, move at crouch | 3 | Free (standard) |
| Medkit | 75 HP | 6s | Two-hand, must be stationary/prone | 1 | $800 buy phase |
| Defibrillator | Revive 50 HP | 4s charge | Apply to downed teammate, LOUD buzz | 2 | Medic only |
| Surgery Kit | Restore 100% | 10s | Full treatment, stationary/prone, cannot cancel | 1 | Engineer only |

#### Healing Rules
- **Self:** Bandage/medkit at normal speed. Cannot self-defib
- **Teammate:** 1.5x faster (bandage 2s, medkit 4s). Can heal damaged teammate (not just revive)
- **Under fire:** Cannot heal while taking damage. Hit during heal = interrupt + item wasted
- **Bleed-out:** 30s timer after down. Headshot = instant death (no timer). Teammate revives within timer. Timer pauses during revive attempt

#### Specialization Healing Bonuses
| Spec | Bandage | Medkit | Revive |
|------|---------|--------|--------|
| None | 25 HP / 3s | 75 HP / 6s | 50 HP / 4s |
| Medic | 40 HP / 2.5s | 100 HP / 5s | 100 HP / 2.5s |
| Assault | 25 HP / 3s | 75 HP / 6s | 50 HP / 4s |
| Engineer | 30 HP / 2.5s | 85 HP / 5s | 60 HP / 3s |
| Recon | 20 HP / 3s | 65 HP / 6s | 40 HP / 4s |

#### Damage & Lethality (by source)
| Source | Body | Headshot | Limb | Notes |
|--------|------|----------|------|-------|
| Rifle 5.56 (M4A1, G36C) | 30 | 150 (instant kill) | 18 | Standard assault rifle damage |
| Rifle 7.62 (AK-47, SCAR-H) | 40 | 200 (instant kill) | 24 | Higher caliber, more damage |
| Sniper (M24, L96A1) | 100 | 300 (instant kill) | 60 | One-shot to body at any range |
| Pistol 9mm (MP5SD) | 20 | 100 (instant kill) | 12 | SMG/pistol caliber |
| Pistol .45 ACP (M1911) | 25 | 125 (instant kill) | 15 | Heavy pistol |
| Shotgun (per pellet x8) | 15 | 75 per pellet | 10 | 8 pellets. Close headshot = kill |
| Frag grenade | 100 | 100 | 75 | No headshot bonus for explosions |
| AP Mine | 150 | 150 | 112 | Higher base damage |
| C4 | 500 | 500 | 375 | Lethal in entire radius |
| Knife (primary) | 50 | 100 | 35 | Heavy attack: 100 body, instant behind |
| Throwing knife | 50 | 100 | 35 | Silent, retrievable |
| Fall damage | 5/m after 3m | N/A | N/A | Measured from fall_start_y to landing |
| Fire (env) | 10/s | 10/s | 10/s | 2s lingering after leaving |

## 5. Phase Roadmap (10 Phases)

### Phase 0 — Foundation (Complete)
Tasks: Workspace, engine, core types, input, camera, basic movement, state machine, dev tools
- [x] Multi-crate Cargo workspace (5 crates: core, input, rendering, audio, game)
- [x] Greybox test level: 20x20m room with walls, 4 pillars, ramp, 2 stair steps
- [x] Third-person chase camera: mouse orbit, lerp follow, 6m distance, 70 FOV
- [x] WASD movement + Sprint/Crouch/Prone toggles with per-stance speeds
- [x] Keyboard + gamepad bindings via leafwing-input-manager 0.20
- [x] State machine: MainMenu -> Loading (0.5s) -> InGame
- [x] Core crate with ZERO Bevy dependency (verify via cargo tree)
- [x] Developer tools: clippy, fmt, watch, audit, cargo-edit, typos-cli

### Phase 1 — Core Systems (Complete)
Tasks: MoveAndSlide, camera-relative movement, camera collision, shooting, damage/death/respawn, enemy AI, teammate AI, HUD, reload, console
- [x] MoveAndSlide character controller (ramps, stairs, walls, slopes > 45deg)
- [x] Camera-relative movement (W = camera forward, S = backward, A/D = strafe)
- [x] Camera collision raycast (Avian3d spatial query, soft push inward)
- [x] Hitscan shooting from camera center (300m range, fire rate capped)
- [x] Health/damage system: DamageMessage -> apply -> DeathMessage -> 3s respawn
- [x] Enemy AI FSM: Patrol (waypoints) -> Detection (120deg vision, 40m, LOS raycast) -> Engage (fire at intervals, reload)
- [x] AI teammate: Follow player (2.5m), engage visible enemies, reload when empty
- [x] HUD: health bar (red, bottom-left), stance text, ammo counter (mag/reserve), crosshair, weapon name
- [x] Weapon swap (1 primary / 2 sidearm), reload (R) with magazine management
- [x] Dev console (backtick): help, god, noclip, killall, timescale [0.1-10], tp [x y z]

### Phase 2 — Advanced Systems (Complete)
Tasks: Camera overhaul, modular weapons, gear, stamina, sway, turn rate, cover, suppression, command wheel, drones, breathing, missions, ammo types, progression, save/load, physics, expanded HUD
- [x] 1st/3rd person toggle (V/MMB) with smooth perspective interpolation (0.12 lerp/frame)
- [x] ADS zoom in both perspectives: -15 FOV, 0.5x spread mult, 0.7x speed mult
- [x] 8-file modular weapon system: chassis, caliber, barrel, sight, underbarrel, magazine, stock, CompleteWeapon
- [x] Weapon weight: Light 1.0x / Medium 0.75x / Heavy 0.55x speed, affects ADS time and sway
- [x] Gear: 5-slot inventory (Primary/Sidearm/Helmet/Armor/Tactical), 5 rarities (Common->Legendary), attachment workshop
- [x] Gear->Combat wiring: inventory damage bonus, workshop stat modifiers applied to active weapon
- [x] Stamina: 100 max, 25/s sprint drain, 15/s regen after 1.5s, exhaust = 0.6x speed, 2.5x sway, 2.0x spread
- [x] Weapon sway + bob per stance with stamina modifiers
- [x] Turn rate limiting: stance (0.3x prone -> 1.0x stand) x weight (2.0-weight_mult) x stamina (0.6x if exhausted)
- [x] Cover detection: 4-direction raycast at 0.6m, emits CoverStateMessage
- [x] Suppression: 20/hit build, 2s decay, increases weapon spread
- [x] Command wheel (Tab): 4 orders - Move, Engage, Suppress, Regroup
- [x] Squad formation system: 3 positions (left, right, rear) relative to player
- [x] Recon Drone (U): 120 battery, 3/s drain, 40s flight, 8m altitude, auto-return at 15%, WASD flight
- [x] FPV Strike Drone (J): 30 battery, 3/s drain, 10s flight, 25 m/s, detonate (Space), proximity auto-detonate, 200 dmg in 8m
- [x] Breathing (Aim+Sprint): hold breath steadies aim up to 50%, drains 8 stamina/s, 2s cooldown
- [x] Missions: 5 objective types (EliminateAll/ReachLocation/DefendPosition/CollectIntel/Extract), auto-completion
- [x] Ammo types: FMJ (1.0x/1.0x), HP (1.25x/0.5x), AP (0.85x/1.8x), Tracer (0.95x/1.0x) damage/pen
- [x] Progression: 50 XP/kill, level = sqrt(xp/100), 7 achievements, 4 specializations
- [x] Save/Load: F5 quick save, F9 quick load, pause menu buttons, full RON serialization
- [x] Physics at 120Hz fixed timestep via Time<Fixed>
- [x] Expanded HUD: XP popups (2s), level-up banners (3s), stamina bar, achievement popups (4s), kill feed (5s), squad status, objective text

#### Phase 2 Systems — Currently In Progress (Completed and merged to main)
>- [x] Equipment system: All throwables, deployables, melee — 24 subtasks complete
>- [x] Medic & healing system: 14 subtasks complete
>- [x] Sniper rifle chassis: M24 + L96A1 with scope expansion
>- [x] Advanced drones: Grenade Drone (H) + Mine Drone (N) with dedicated systems
>- [ ] Grenade Drone (H key): Implementation complete — quadcopter with 4x frag hardpoints, WASD flight, SPACE to drop *(HUD counters pending)*
>- [ ] Mine Drone (N key): Implementation complete — 3x AP mine dispenser, LINE/TRIANGLE/CIRCLE patterns, G to deploy *(HUD counters pending)*

### Phase 3 — Asset Integration & Polish (NEXT — 8 subtasks)
**Goal:** First playable build with visual/audio assets

P3.1 Placeholder Audio (est. 2 hrs)
- [ ] Generate 15 footstep .ogg files (gravel/concrete/metal/grass/dirt x walk/sprint/crouch)
- [ ] Generate 12 weapon .ogg files (M4A1/MP5SD/M1911/AK-47 fire, suppressed, dry)
- [ ] Generate 4 ambient .ogg files (outdoor wind, indoor hum, drone buzz, radio static)
- [ ] Generate 8 UI .ogg files (menu click, hover, match start/end, kill confirmed, chat ping)
- [ ] Verify all files load without warnings

P3.2 Procedural Weapon Models (est. 3 hrs)
- [ ] Create box-based weapon mesh hierarchy (receiver, barrel, sight, grip, mag, stock)
- [ ] Attach weapon model to camera in 1st person (bottom-right, ADS position)
- [ ] Attach weapon model to player in 3rd person (holster position, firing position)
- [ ] Weapon swap animates/hides primary vs sidearm
- [ ] Shoulder mirror (weapon swaps sides with camera)

P3.3 bevy_hanabi VFX ✅ Complete (est. 4 hrs)
- [x] Muzzle flash effect (yellow point light + expanding sphere burst, 0.05s)
- [x] Bullet impact sparks (orange/white particles, hit normal direction)
- [x] Hit marker (enemy: red flash on damage, 0.08s)
- [x] Death explosion (6 particles bursting outward, 1s lifetime, gravity)
- [x] Tracer round (glowing sphere, 0.3s trail)

P3.4 Post-Processing ✅ Complete (est. 2 hrs)
- [x] Tone mapping (ACES filmic curve)
- [x] Bloom for explosions and muzzle flash
- [ ] SSAO for tactical depth perception (requires bevy crate feature)
- [ ] Depth of field for ADS blur (requires bevy crate feature)

P3.5 Kira Audio Integration (est. 4 hrs)
- [ ] Wire kira as primary audio backend
- [ ] Create bus hierarchy: Master > SFX/Ambient/UI/Voice
- [ ] Spatial audio for footsteps and weapon fire
- [ ] Audio occlusion for suppressed gunfire

P3.6 Character Models via Blender MCP (est. 8 hrs)
- [ ] Player operator: tactical gear, rigged for animation/ragdoll, ~7K tris
- [ ] Enemy insurgent + mercenary variants
- [ ] AI teammate with distinct visual
- [ ] Export to GLTF with armature + materials

P3.7 Weapon Models via Blender MCP (est. 6 hrs)
- [ ] M4A1 with attachment points (barrel, sight, grip, mag, stock)
- [ ] MP5SD integral suppressed
- [ ] M1911 pistol
- [ ] AK-47 with wood furniture
- [ ] Export each attachment variant as separate GLTF

P3.8 Level Art (est. 4 hrs)
- [ ] Replace greybox geometry with GLTF static meshes
- [ ] Modular tile system: 1m floor tiles, wall segments, corner pieces
- [ ] Lighting setup: directional sun, ambient light probes
- [ ] Collision mesh generation for static level geometry

### Phase 4 — Training & Practice (6 subtasks)
**Goal:** Practice mode with target variety and bot matches

P4.1 Training Range Map (est. 3 hrs)
- [ ] Open area with shooting lanes (10m/25m/50m/100m marked)
- [ ] Cover positions (low wall, corner, window) for practice
- [ ] Drone flight zone with obstacle course

P4.2 Target System (est. 3 hrs)
- [ ] Static paper silhouettes at varied distances
- [ ] Steel plate targets with ping feedback
- [ ] Headshot-only targets for precision practice
- [ ] Pop-up targets on random timers (0.5-3s exposure)
- [ ] Moving targets: lateral (3m/s), advancing (2m/s), random path

P4.3 Weapon Locker UI (est. 2 hrs)
- [ ] Full weapon list: all chassis + all attachments
- [ ] Select any combination, see computed stats
- [ ] Ammo regeneration toggle (infinite / finite)

P4.4 Aim Trainer (est. 2 hrs)
- [ ] Accuracy percentage per session
- [ ] DPS meter (damage per second sustained)
- [ ] Reaction time tracker (target appear -> hit)

P4.5 Drone Practice (est. 2 hrs)
- [ ] Dedicated drone recharge station
- [ ] Target markers for recon drone marking
- [ ] FPV dummy targets for detonation practice

P4.6 SQB — Squad Bots Mode (est. 4 hrs)
- [ ] Play any game mode vs AI bots
- [ ] Bot difficulty: Easy (0.5x accuracy/speed) / Medium (1.0x) / Hard (1.5x)
- [ ] Bot count: fill empty player slots

### Phase 5 — Competitive Game Modes (9 subtasks)
**Goal:** 3 fully playable competitive modes with rounds

P5.1 Team Deathmatch (est. 4 hrs)
- [ ] Round manager: 5v5, 5-6 min timer, sudden death
- [ ] Win condition: eliminate all or time-up standing
- [ ] Scoreboard: kills, deaths, assists, damage
- [ ] Best of 9 match with side swap

P5.2 Demolition (est. 5 hrs)
- [ ] Bomb plant mechanic (hold E on site, 3s plant time)
- [ ] Bomb defuse mechanic (hold E on bomb, 7s defuse time)
- [ ] Two bomb sites: A and B with distinct layout
- [ ] Halftime switch after 4 rounds
- [ ] Bomb timer (45s after plant), defuse win/explosion win

P5.3 Capture the Flag (est. 4 hrs)
- [ ] Flag pickup/drop/capture mechanics
- [ ] Flag carrier speed penalty (0.85x)
- [ ] 10s team cooldown on dropped flag
- [ ] Capture zone at each team's base

P5.4 Round Timer System (est. 2 hrs)
- [ ] Configurable round time (3-10 min)
- [ ] Overtime logic: sudden death at match point tie
- [ ] Round start countdown (5 seconds)
- [ ] Round end screen (win/loss, stats)

P5.5 Win/Loss Conditions (est. 2 hrs)
- [ ] Validate win conditions per mode
- [ ] Draw handling (time expires, both alive, compare kills/damage)
- [ ] Match point logic (round X of 9 reporting)

P5.6 Pre-Round Buy Phase (est. 3 hrs)
- [ ] Buy menu at round start (F2 or auto)
- [ ] Economy system: money per kill (300), per win (1500), per loss (2000)
- [ ] Weapon/gear pricing tiers (pistol = free, rifle = 2000-4000)

P5.7 Spectator Mode (est. 3 hrs)
- [ ] Free camera (WASD + mouse orbit)
- [ ] Player follow (cycle through players)
- [ ] Overview camera (top-down tactical view)

P5.8 Kill Cam (est. 2 hrs)
- [ ] 3-second replay from killer's perspective
- [ ] Show health/weapon/distance info

P5.9 Round MVP / Best Play (est. 3 hrs)
- [ ] Track best play: multi-kill, clutch, wallbang
- [ ] Auto-detect highlight moments
- [ ] MVP screen at match end

### Phase 6 — Multiplayer & Networking (10 subtasks)
**Goal:** Fully functional multiplayer with dedicated servers

P6.1 Server Browser
- [ ] Server list UI (name, map, mode, players, ping)
- [ ] Dedicated server executable (headless)
- [ ] Server configuration: max players, map rotation, mode

P6.2 Netcode (lightyear)
- [ ] Client-server connection establishment
- [ ] State synchronization (player positions, health, weapons)
- [ ] Input prediction and server reconciliation
- [ ] Lag compensation (rewind-based hit registration)

P6.3 ECS Replication (bevy_replicon)
- [ ] Replicated components on player/weapon/health
- [ ] Server authority over damage and physics
- [ ] Client-side interpolation for smooth movement

P6.4 Player Auth & Profile
- [ ] Player authentication (simple token or Steam)
- [ ] Profile persistence: stats, rank, cosmetics
- [ ] Player name and appearance customization

P6.5 Party System
- [ ] Invite to party (in-game or overlay)
- [ ] Party leader controls: start search, kick, invite
- [ ] Join lobby together

P6.6 Matchmaking
- [ ] Skill rating (elo/MMR) per player
- [ ] Team balancing by average MMR
- [ ] Region-based ping matching

P6.7 Voice Chat
- [ ] In-game voice (Opus codec)
- [ ] Channels: team (all), squad (fireteam)
- [ ] Push-to-talk (default) and open mic options

P6.8 Anti-Cheat
- [ ] Server-side validation of player state
- [ ] Client-side integrity checks
- [ ] Report player system

P6.9 Server Configuration
- [ ] Map rotation list
- [ ] Max players (2-20)
- [ ] Round timer, win limit
- [ ] Friendly fire toggle

P6.10 Client Prediction
- [ ] Client-side movement prediction
- [ ] Visual interpolation of other players
- [ ] Hit registration: client predicts, server confirms

### Phase 7 — Production Polish (9 subtasks)
**Goal:** AAA-quality UI/UX and performance

P7.1 Main Menu
- [ ] Animated background (3D scene or video)
- [ ] Mode select: Play (with sub-menu), Training, Gunsmith/Loadout, Settings
- [ ] Version number + build info

P7.2 Player Profile
- [ ] Stats overview: K/D, win rate, accuracy, time played
- [ ] Rank/level display with XP bar
- [ ] Achievement showcase (earned/locked)
- [ ] Match history (last 20 matches)

P7.3 Video Settings
- [ ] Resolution dropdown (native, 720p, 1080p, 1440p, 4K)
- [ ] Refresh rate (60/120/144/240)
- [ ] Display mode (windowed, borderless, fullscreen)
- [ ] V-sync toggle
- [ ] FPS cap (30/60/120/144/unlimited)
- [ ] Quality presets (Low/Medium/High/Ultra)
- [ ] Gamma/brightness slider

P7.4 Audio Settings
- [ ] Master volume (0-100%)
- [ ] SFX volume
- [ ] Music volume
- [ ] Voice volume
- [ ] HRTF toggle (headphone spatial audio)
- [ ] Mute when tabbed out

P7.5 Controls Settings
- [ ] Rebindable keys for every action
- [ ] Conflict detection (highlight overlapping binds)
- [ ] Reset to defaults button
- [ ] Mouse sensitivity slider (0.1-10.0)
- [ ] Invert Y toggle
- [ ] Controller layout diagram

P7.6 Accessibility
- [ ] Colorblind mode (protanopia/deuteranopia/tritanopia)
- [ ] Screen shake intensity (0-100%)
- [ ] UI scale (small/medium/large)
- [ ] Subtitle toggle for VO
- [ ] High-contrast crosshair options

P7.7 Tutorial System
- [ ] Interactive tutorial: movement (WASD, sprint, crouch, prone, jump)
- [ ] Combat tutorial: shooting, ADS, reload, weapon swap
- [ ] Squad tutorial: command wheel, orders, team play
- [ ] Drone tutorial: deploy, fly, detonate, recall
- [ ] Skip tutorial option

P7.8 Post-Processing Stack
- [ ] Bloom: weapon muzzle flash, explosions, glowing objects
- [ ] SSAO: tactical depth cues around cover
- [ ] Depth of field: ADS blur + spectator/cinematic
- [ ] Chromatic aberration: injury/suppression effect

P7.9 Performance Optimization
- [ ] LOD system for character and weapon models
- [ ] Occlusion culling for level geometry
- [ ] Draw call batching for static meshes
- [ ] Shadow map resolution options
- [ ] Texture quality options

### Phase 8 — Campaign & Missions (8 subtasks)
**Goal:** 10-15 mission single-player campaign with co-op

P8.1 Campaign Framework
- [ ] Mission select screen (linear or radial tree)
- [ ] 10-15 interconnected missions with narrative
- [ ] Difficulty selection (Recruit/Veteran/Elite)

P8.2 Mission Briefings
- [ ] Pre-mission briefing screen (text + map)
- [ ] Primary and secondary objectives displayed
- [ ] Loadout selection: primary, sidearm, gear, drone type
- [ ] Insertion point selection (if multiple entry points)

P8.3 Narrative & VO
- [ ] Mission radio chatter (teammates, command, enemy)
- [ ] Story cutscenes (in-engine)
- [ ] Enemy taunts and callouts (proximity-based)

P8.4 Checkpoint Save
- [ ] Auto-save at objective completions
- [ ] Manual save at any time (F5)
- [ ] Save includes: position, health, ammo, objective state, AI states

P8.5 Failure Conditions
- [ ] Player death = restart from last checkpoint
- [ ] Time limit (if applicable)
- [ ] Objective failed (VIP killed, target escaped)

P8.6 Bonus Objectives
- [ ] Intel collectibles (hidden documents, laptops)
- [ ] Speed run bonus (complete under time threshold)
- [ ] Pacifist bonus (minimum kills, stealth approach)

P8.7 Mission Scoring
- [ ] Accuracy percentage
- [ ] Stealth bonus (no alarms triggered)
- [ ] Objectives completed (primary + bonus)
- [ ] Time bonus (faster = higher score)
- [ ] S-rank / A-rank / B-rank / C-rank per mission

P8.8 Co-op Campaign
- [ ] 2-4 player co-op (online)
- [ ] Shared progression (host's save)
- [ ] Player drop-in/drop-out
- [ ] Revive system: downed -> bleed out -> respawn at checkpoint

### Phase 9 — Modding & Workshop (7 subtasks)
**Goal:** Fully moddable game with Steam Workshop

P9.1 Custom Maps
- [ ] GLTF scene loading from external maps/ directory
- [ ] Collision mesh auto-generation
- [ ] Spawn point configuration via RON
- [ ] Lighting from scene file

P9.2 Custom Weapons
- [ ] RON config: chassis, caliber, stat values
- [ ] Attachment point definitions
- [ ] Weapon model GLTF references

P9.3 Custom Missions
- [ ] JSON mission descriptor format
- [ ] Objective scripting (limited DSL)
- [ ] Enemy placement, spawn triggers

P9.4 Mod Scanning
- [ ] Scan maps/ weapons/ missions/ directories on startup
- [ ] Enumerate installed mods in main menu
- [ ] Conflict detection (duplicate IDs)
- [ ] Enable/disable per mod

P9.5 Workshop Integration
- [ ] Steam Workshop API connection
- [ ] Upload mod tool (in-game or CLI)
- [ ] Auto-download subscribed mods

P9.6 Custom Scripting
- [ ] Limited Lua VM for game mode logic
- [ ] Sandboxed: no file system, no network
- [ ] Hook into game events (player killed, round start, objective complete)

P9.7 SDK Documentation
- [ ] API reference for RON configs
- [ ] GLTF export guide (Blender -> game)
- [ ] Example mods with source
- [ ] Video tutorial series

### Phase 10 — Post-Launch Live Service (5 subtasks)
**Goal:** Ongoing content and community engagement

P10.1 Content Drops
- [ ] New weapon packs (seasonal, 4-6 weapons each)
- [ ] New maps for each game mode (2-3 per season)
- [ ] New gear and attachments

P10.2 Ranked Seasons
- [ ] 3-month competitive seasons
- [ ] Elo/MMR rating system
- [ ] Season rewards: weapon skins, player titles, badges
- [ ] Leaderboards: global, regional, friend-based

P10.3 Battle Pass
- [ ] Free + premium track
- [ ] 100 tiers per season
- [ ] Rewards: cosmetics (weapon skins, character outfits, drone skins)
- [ ] XP boosts and currency

P10.4 Community Engagement
- [ ] Community map contests (featured in rotation)
- [ ] Player feedback surveys
- [ ] Dev blog / patch notes

P10.5 Maintenance
- [ ] Anti-cheat signature updates
- [ ] Performance optimization per patch
- [ ] Bug fix releases (as needed)
- [ ] Server binary updates

## 6. Technical Requirements

### Engine & Platform
| Requirement | Target | Notes |
|-------------|--------|-------|
| Game Engine | Bevy 0.18.1 | Pinned across ALL crates. NOT 0.18.0 or 0.19 |
| Physics | Avian3d 0.6.1 | XPBD solver, MoveAndSlide character controller |
| Rendering | WGPU (Vulkan/DX12/Metal) | Abstracted backend, PBR pipeline |
| Physics Rate | 120Hz fixed timestep | Time<Fixed> resource configured in main.rs |
| Target FPS | 60+ at 1080p (9600K + GTX 1060) | Phase 7 optimization target |
| Target OS | Windows 10/11 | Linux/Mac at Phase 7 |
| Window | 1280x720 bordered default | Configurable in Phase 7 settings |

### Rust Toolchain
| Tool | Version |
|------|---------|
| Rust Edition | 2021 |
| MSRV | 1.80+ |
| Target | x86_64-pc-windows-msvc |

### Build Requirements
- `cargo check` — 0 errors across all 5 crates (CI enforced)
- `cargo clippy` — 0 lint errors (dead_code OK for pre-asset state)
- `cargo fmt --check` — consistent formatting
- `cargo test` — all unit/integration tests pass
- `cargo audit` — no advisories before dependency bumps
- Source files: max 200 lines, single responsibility

### Full Dependency Stack
| Package | Version | Purpose | Phase |
|---------|---------|---------|-------|
| bevy | 0.18.1 | Game engine | 0 |
| avian3d | 0.6.1 | XPBD physics | 1 |
| leafwing-input-manager | 0.20 | Action-based input | 0 |
| serde + ron | 1.x / 0.12 | Serialization | 0 |
| bevy_hanabi | 0.18.0 | GPU particle VFX | 3 |
| kira | 0.12 | Audio middleware | 3 |
| bevy_replicon | 0.41.0-rc.1 | ECS networking | 6 |
| lightyear | 0.26.4 | Rollback netcode | 6 |
| lightyear_avian3d | 0.26.4 | Physics rollback | 6 |
| bevy-inspector-egui | 0.36 | Runtime debug UI | 0 |
| iyes_progress | 0.17.0-rc.1 | Loading screen | 3 |
| bevy_common_assets | 0.17.0-rc.1 | RON/JSON asset loading | 9 |

## 7. Controls Reference

| Input | Action | Context |
|-------|--------|---------|
| WASD | Movement (camera-relative) | Gameplay |
| Mouse | Camera look (pitch/yaw) | Gameplay |
| Left Click | Fire weapon | Gameplay |
| Right Click | Aim Down Sights (hold) | Gameplay |
| Shift | Sprint / Hold breath (while ADS) | Gameplay |
| C | Crouch toggle | Gameplay |
| Z | Prone toggle | Gameplay |
| Space | Jump / Detonate FPV drone | Gameplay |
| R | Reload weapon | Gameplay |
| E | Interact (pickup, plant, defuse) | Gameplay |
| Q | Shoulder swap (left/right) | Gameplay |
| Tab | Command wheel (hold/release) | Gameplay |
| 1 | Primary weapon | Gameplay |
| 2 | Sidearm weapon | Gameplay |
| V / MMB Click | Toggle 1st/3rd person | Gameplay |
| MMB Hold | Freelook (freeze camera yaw/pitch) | Gameplay |
| U | Deploy/recall Recon Drone | Gameplay |
| J | Deploy/recall FPV Strike Drone | Gameplay |
| Escape | Pause / Menu | Gameplay |
| Backtick (`) | Dev console | Any |
| F5 | Quick save | SP Gameplay |
| F9 | Quick load | SP Gameplay |
| F1 | World Inspector (debug) | Any |
| Enter | Chat | Multiplayer |

## 8. Non-Functional Requirements

| Metric | Target | Phase |
|--------|--------|-------|
| Build errors | 0 at all times, CI-enforced | 0 |
| Source file size | < 200 lines per file | 0 |
| FPS (mid-range) | 60+ at 1080p | 7 |
| Load time (cold) | < 5 seconds | 7 |
| Input latency | < 50ms (keyboard to render) | 0 |
| Memory (gameplay) | < 1 GB RAM | 7 |
| Network bandwidth | < 100 Kbps per client | 6 |
| Max players/server | 20 (10v10) | 6 |
| Binary size (release) | < 50 MB (no assets) | 0 |
| Playable session | 10+ minutes without crash | 3 |

## 9. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation | Responsible Phase |
|------|--------|------------|-----------|-------------------|
| Bevy 0.18 -> 0.19 breaking changes | Critical (entire codebase) | Medium | Pin version. Audit migration guide before any upgrade | 0 |
| Physics jank with character controller | High (movement feel) | Medium | Avian MoveAndSlide with comprehensive ramp/stairs testing | 1 |
| AI pathfinding complexity | High (AI behavior) | Medium | FSM + waypoints for Phase 1-2. Upgrade to navmesh in Phase 8 | 1 |
| Audio asset licensing costs | Low (budget) | Medium | AI-generated prototypes Phase 3. Licensed foley in Phase 7 | 3 |
| Multiplayer netcode complexity | Critical (core feature) | High | lightyear provides rollback. Start simple (client-server) | 6 |
| Performance with 10v10 full server | Medium (playability) | Medium | Profile early with LODs + occlusion culling. Optimize render passes | 7 |
| Modding API stability | Medium (ecosystem) | Low | Version API. Provide migration guides for breaking changes | 9 |
| Asset pipeline delays | Medium (timeline) | High | Blender MCP provides direct pipeline. Fall back to procedural | 3 |

## 12. Destruction & Damage System Design

**Philosophy:** Battlefield-grade destruction where every bullet, explosion, and collision leaves permanent, realistic damage on the environment. No gimmick effects — damage propagates through materials logically, affects gameplay (cover degrades, walls become penetrable), and persists for the duration of the match/round.

### 12.1 Destruction Principles

| Principle | Implementation |
|-----------|----------------|
| **Ballistic Penetration** | Bullets pass through thin materials (wood, drywall, sheet metal) with damage falloff. Thick materials (concrete, armor plate) stop rounds but show craters |
| **Progressive Damage** | Assets have 3-4 damage states (Pristine -> Scratched/Dented -> Breached/Cracked -> Destroyed/Collapsed). State transitions are permanent and do not revert |
| **Material-Property Driven** | Damage behavior defined by material type (concrete: chips + dust, wood: splinters + cracks, metal: dents + sparks + holes, glass: crack pattern -> shatter -> fall) |
| **Gameplay Impact** | Destroyed cover exposes players. Breached walls create new sightlines. Collapsed buildings block paths. Burning vehicles block sightlines + deal damage |
| **Networked State** | Damage state is a replicated component. Server determines when state transitions occur. Client predicts + reconciles |

### 12.2 Damage State System

Every destructible asset has a standardized damage state component:

```rust
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct DamageState {
    pub state: u8,                    // 0=pristine, 1=damaged, 2=breached, 3=destroyed
    pub damage_mask: u32,             // Bitmask of hit locations (32 zones per asset)
    pub bullet_holes: Vec<Vec3>,      // Accumulated bullet impact positions (capped at 64)
    pub scorch_mask: f32,             // 0.0-1.0 burn coverage
    pub structural_integrity: f32,    // 0.0-1.0 remaining health
    pub debris_spawned: bool,         // Whether debris entities exist
}

impl DamageState {
    pub fn apply_damage(&mut self, amount: f32, material: MaterialType, position: Vec3) -> bool {
        self.structural_integrity = (self.structural_integrity - amount).max(0.0);
        self.bullet_holes.push(position);
        if self.bullet_holes.len() > 64 { self.bullet_holes.remove(0); }
        
        let new_state = match self.structural_integrity {
            s if s > 0.66 => 0, // Pristine
            s if s > 0.33 => 1, // Damaged
            s if s > 0.10 => 2, // Breached
            _ => 3,              // Destroyed
        };
        self.state = new_state;
        new_state >= 2 // Returns true if gameplay-relevant change
    }
}
```

### 12.3 Destruction Physics & VFX

| Event | Effect | Performance Budget |
|-------|--------|-------------------|
| Bullet hit (concrete) | Chips fly, dust puff, small crater decal | 50 particles, 0.5s |
| Bullet hit (wood) | Splinters spray, crack decal, penetration hole | 30 particles, 0.3s |
| Bullet hit (metal) | Sparks, ring decal, dent, possible penetration | 40 particles, 0.4s |
| Bullet hit (glass) | Crack pattern radiates, then shatter + fall if hit again | 100 particles, 1.0s |
| Explosion (near wall) | Large crater, dust cloud, chunks fly, structural damage | 500 particles, 3.0s |
| Explosion (vehicle) | Fireball, black smoke, parts detach, wreck state | 1000 particles, 5.0s |
| Building collapse | Progressive sag -> dust cloud -> rubble pile | 2000 particles, 8.0s |
| Glass window break | Pre-fracture -> shatter along score lines -> falling shards | 200 particles, 1.5s |
| Tire destruction | Rubber shreds, rim exposes, vehicle tilts | 50 particles, 1.0s |
| Wall breach (C4) | 2m x 2m hole, exposed rebar/studs, dust cloud | 800 particles, 4.0s |

### 12.4 Destruction by Asset Category

| Category | Damage States | Destruction Triggers | Debris Type |
|----------|---------------|---------------------|-------------|
| Buildings | 4 (Pristine -> Damaged -> Breached -> Collapsed) | Explosives (C4/RPG), heavy vehicle impact, structural failure | Rubble piles, dust, exposed rebar |
| Walls (modular) | 4 | Breached by C4 (2m hole), degraded by sustained fire | Chunks, dust, reinforcement visible |
| Windows/Glass | 2 (Intact -> Broken) | Any bullet impact near glass. Breaks along score lines | Falling shards, frame empty |
| Doors | 3 (Locked -> Open -> Broken) | Breached by shotgun/C4, kicked open | Hanging hinges, splintered wood |
| Vehicles | 4 (Operational -> Disabled -> Burning -> Wreck) | Engine/transmission hits, fuel tank explosion, sustained fire | Burning wreck, parts scattered, black smoke |
| Tires | 2 (Inflated -> Flat) | Any bullet hit to tire area | Shredded rubber, rim visible |
| Barrels (metal) | 3 (Pristine -> Dented -> Ruptured) | Bullets cause leaks, explosions detonate | Ruptured metal, leaking contents |
| Barrels (explosive) | 2 (Intact -> Exploded) | Any damage to red barrel = detonation | Fireball, fragments, crater |
| Wooden crates | 3 (Intact -> Splintered -> Destroyed) | Bullets penetrate, explosions destroy | Wood splinters, contents spill |
| Sandbags | 3 (Full -> Split -> Empty) | Sustained fire causes sand to leak | Sand piles, torn fabric |
| Fences | 2 (Standing -> Broken) | Vehicle impact, explosions | Broken posts, sagging wire |
| Trees | 2 (Standing -> Fallen) | Explosions, heavy vehicle impact | Fallen trunk, splintered stump |
| Signs/lamps | 2 (Standing -> Broken) | Bullets, explosions, vehicle collision | Bent pole, broken glass |

### 12.5 Destruction Performance Budget

| Metric | Limit |
|--------|-------|
| Max damage state transitions per frame | 10 |
| Max accumulated bullet holes per asset | 64 |
| Max bullet holes visible on terrain | 500 |
| Max scorch marks visible | 100 |
| Max debris entities active | 200 |
| Max destroyed vehicle wrecks per map | 10 |
| Building collapse animation time | 3-8 seconds |
| Damage state LOD (fade below) | LOD1 (50m+) show state 0/1/2 only. LOD2 (100m+) show billboard |

---

## 13. Complete Asset Pipeline — Everything Needed

This section inventories EVERY 3D model, texture, audio file, particle effect, and animation needed for AAA tactical shooter production. Each asset is specified with real-world scale, texture resolution variants (1K/2K/4K), poly budgets, production dependencies, damage states, and detailed physical description to ensure grounded, Battlefield-quality realism.

### Texture Resolution Standards

| Resolution | Usage | Quality Tier |
|------------|-------|-------------|
| **4K (4096x4096)** | Hero assets: player characters, weapon first-person view, menu backgrounds | Ultra |
| **2K (2048x2048)** | Primary standard: environments, weapons, vehicles, characters, props | High (default) |
| **1K (1024x1024)** | Small props, decals, UI icons, particle sprites, crowd characters | Medium |
| **512x512** | Shadow maps, light cookies, very small props | Low |

All textures use PBR (Physically Based Rendering) pipeline:
- **Albedo** (base color) — 4K/2K/1K, sRGB, .png or .ktx2
- **Normals** (surface detail) — 4K/2K/1K, linear, .ktx2 (BC5/DXT5nm)
- **RMA** (Roughness/Metalness/AO packed) — 4K/2K/1K, linear, single channel
- **Displacement** (tessellation) — 2K/1K, linear, optional
- **Damage mask** — 1K, single channel (optional, for procedural damage blending on hero assets)

### Real-World Scale & Convention

| Property | Standard |
|----------|----------|
| 1 unit in engine | 1 meter in real world |
| Character height (eye) | 1.65m (standing) |
| Character height (top of head) | 1.80m (male operator) |
| Grid tile | 1m x 1m (modular system base) |
| Y-axis | Up |
| Forward | -Z (Bevy default) |
| UV mapping | 0-1 unwrapped, no overlaps (bake-friendly) |
| LOD reduction | LOD0 = full, LOD1 = 50%, LOD2 = 25%, LOD3 = billboard |
| Collision | Separate simple-convex per object, not auto-generated |
| Damage states per asset | 2-4 (Pristine -> Damaged -> Breached -> Destroyed) |
| Debris prefabs | Per material: concrete chunk, wood splinter, metal scrap, glass shard |

### Production Dependency Chain

Assets must be created in this order due to dependencies:
```
Phase 3.6 (Characters P0)     Phase 3.7 (Weapons P0)     Phase 3.8 (Environment P0)
    |                                |                           |
    v                                v                           v
Skeleton + Rig                 Weapon Base Meshes         Modular Tile Set
    |                                |                           |
    v                                v                           v
Skin + Shaders                 Attachment Variants         Building Kits
    |                                |                           |
    v                                v                           v
Animation Retarget             Gunsmith UI Prep             Props + Decals
    |                                |                           |
    v                                v                           v
Full Anim Set                  Phase 3.3 (VFX)              Natural Assets
    |                                |                           |
    v                                v                           v
Phase 10 (Voice)               Particle Effects             Lighting Probes
                                      |
                                      v
                              Destruction debris meshes
                             (per material type per size)
```

### Texture Resolution Standards

| Resolution | Usage | Quality Tier |
|------------|-------|-------------|
| **4K (4096x4096)** | Hero assets: player characters, weapon first-person view, menu backgrounds | Ultra |
| **2K (2048x2048)** | Primary standard: environments, weapons, vehicles, characters, props | High (default) |
| **1K (1024x1024)** | Small props, decals, UI icons, particle sprites, crowd characters | Medium |
| **512x512** | Shadow maps, light cookies, very small props | Low |

All textures use PBR (Physically Based Rendering) pipeline:
- **Albedo** (base color) — 4K/2K/1K, sRGB, .png or .ktx2
- **Normals** (surface detail) — 4K/2K/1K, linear, .ktx2 (BC5/DXT5nm)
- **RMA** (Roughness/Metalness/AO packed) — 4K/2K/1K, linear, single channel
- **Displacement** (tessellation) — 2K/1K, linear, optional

### Real-World Scale & Convention

| Property | Standard |
|----------|----------|
| 1 unit in engine | 1 meter in real world |
| Character height (eye) | 1.65m (standing) |
| Character height (top of head) | 1.80m (male operator) |
| Grid tile | 1m x 1m (modular system base) |
| Y-axis | Up |
| Forward | -Z (Bevy default) |
| UV mapping | 0-1 unwrapped, no overlaps (bake-friendly) |
| LOD reduction | LOD0 = full, LOD1 = 50%, LOD2 = 25%, LOD3 = billboard |
| Collision | Separate simple-convex per object, not auto-generated |

### Production Dependency Chain

Assets must be created in this order due to dependencies:
```
Phase 3.6 (Characters P0)     Phase 3.7 (Weapons P0)     Phase 3.8 (Environment P0)
    |                                |                           |
    v                                v                           v
Skeleton + Rig                 Weapon Base Meshes         Modular Tile Set
    |                                |                           |
    v                                v                           v
Skin + Shaders                 Attachment Variants         Building Kits
    |                                |                           |
    v                                v                           v
Animation Retarget             Gunsmith UI Prep             Props + Decals
    |                                                          |
    v                                                          v
Full Anim Set                  Phase 3.3 (VFX)              Natural Assets
    |                                |                           |
    v                                v                           v
Phase 10 (Voice)               Particle Effects             Lighting Probes
```

### 13.1 Character Models

All characters use a **shared humanoid skeleton** (UE4 Mannequin / Mixamo compatible) for cross-character animation retargeting. Rig includes 65 bones minimum: spine (5), neck (2), head (1), arms (4 per side + fingers simplified), legs (4 per side). IK targets for both feet, both hands, and head. Aim constraint on upper spine + neck + head.

Characters are photorealistic PBR with 2K base textures (4K for hero characters). Skin features subsurface scattering approximation. Clothing has fabric weave, dirt/sweat/wear maps. Characters have 2 damage states (Alive -> Dead ragdoll transition). Dead state uses a separate ragdoll prefab with rigidbody-per-bone.

| Model | Variants | Target Tris | Skeleton | LODs | Textures | Scale (height) | Realism Description | Damage States | Priority |
|-------|----------|-------------|----------|------|----------|----------------|---------------------|---------------|----------|
| **Player Operator (Male)** | Base: Multicam uniform, plate carrier, FAST helmet, M4 slung, sidearm holstered. Variants: +backpack, +NVG, +ghillie partial | 8,000-10,000 | Full (65 bones) | LOD0/1/2 | 4K hero | 1.80m | US Army Special Forces impression. Crye Precision G3 combat shirt + pants in Multicam, JPC 2.0 plate carrier with 6 mags + IFAK + radio, Ops-Core FAST helmet with Comtac III headset, M4 carbine slung on QD mount, M1911 in Safariland drop-leg holster, Salomon hiking boots. Wear: carbon buildup on rifle, holster wear on pistol, dust/sweat on fabric, minor gear scuffs | 2 (Alive/Dead ragdoll) | P0 |
| **Player Operator (Female)** | Same gear options as male. Distinct face model, smaller frame, adjusted gear fit | 7,500-9,000 | Full (65 bones) | LOD0/1/2 | 4K hero | 1.70m | Same loadout as male operator but scaled for female proportions. Face: caucasian with short braided hair visible under helmet. Slightly narrower shoulder width, shorter torso. Gear fits differently on frame (carrier sits differently) | 2 | P1 |
| **Enemy Insurgent** | Light gear: untucked shirt, pants, chest rig, AK-47 slung, headwrap/shemagh | 5,000-6,000 | Full (55 bones) | LOD0/1 | 2K | 1.75m | Syrian/Iraqi insurgent impression. Loose-fitting man dress shirt (untucked, sweat-stained), olive/khaki pants, leather/rope belt, Type 56 chest rig (6 mag pockets + 4 grenade pouches, worn fabric), keffiyeh headwrap in red/white or black/white, AK-47 slung with cloth strap, sandals or worn combat boots. Skin: darker complexion, stubble, weathered face texture. Wear: shirt faded from sun, gear has sand/dirt accumulation | 2 | P0 |
| **Enemy Mercenary** | Heavy gear: plate carrier, tactical helmet, combat uniform, AK/M4 slung | 7,000-8,000 | Full (60 bones) | LOD0/1 | 2K | 1.85m | Russian private military contractor impression (Wagner-style). EMR camouflage (Flora/SS-Leto pattern) combat suit, 6B45-style plate carrier with full coverage (scratched ceramic plates visible), 6B47 helmet with cover (camo netting optional), AK-74M with sidefolder stock slung, radio on carrier, utility belt with grenade pouches + knife. Wear: helmet scuffed from use, plate carrier has abrasion marks from vehicle/cover, uniform knees worn from kneeling | 2 | P0 |
| **Enemy Terrorist** | Traditional/religious garb, hidden weapon, light chest rig | 5,500-6,500 | Full (55 bones) | LOD0/1 | 2K | 1.73m | Religious extremist impression. Long traditional robe/thawb (white or gray, fabric detail visible), head covering/kufi, sandals, hidden weapon holster under robe, light chest rig visible only during combat (minimal 2-3 mag capacity). Skin: darker complexion, full beard. Distinct from insurgent: more traditional clothing, less military gear. Wear: robe has dust/dirt at hem, sandals worn | 2 | P1 |
| **Enemy PMC** | Western high-end tactical, full NVG, signature weapon | 7,500-8,500 | Full (60 bones) | LOD0/1 | 2K | 1.82m | US-based private security contractor (Blackwater/Academi style). 5.11 tactical pants + polo shirt (dark navy/gray), Crye AVS plate carrier with full PALS webbing, Ops-Core FAST helmet with GPNVG-18 NVG mount (NVG up/down animated), Comtac III headset, HK416 or Sig MCX slung, thigh-mounted holster (Glock 19), utility belt with dump pouch + IFAK + TQ. Distinct from player: no military patch, commercial/cleaner gear, no national identification. Wear: minimal (professional), some holster wear, clean gear | 2 | P1 |
| **AI Teammate** | Same gear as player but different camo pattern + distinct patch/squad markings | 7,500-8,500 | Full (65 bones) | LOD0/1/2 | 4K hero | 1.80m | Same base as player operator but with distinctive squad markings (colored armband, different camo: Woodland vs player's Multicam, or Desert if player is Woodland). Helmet has distinct cover/band. Carry handle/optic differs from player for visual distinction. Facial features: different face from player. Purpose: immediately distinguishable from player on the battlefield | 2 | P0 |
| **Civilian (Male)** | Casual clothing, no gear | 3,000-4,000 | Simple (40 bones) | LOD0/1 | 1K | 1.75m | Non-combatant. Button-up shirt (solid color, untucked) + jeans/dark pants + closed-toe shoes. 3 color variants per shirt. Generic face, shorter haircut. No tactical gear, no weapon visible. Hands: empty. Purpose: populate urban maps, crowd scenes. Damage state: ragdoll death only (no combat behavior) | 1 (no combat damage) | P3 |
| **Civilian (Female)** | Casual clothing, no gear | 3,000-4,000 | Simple (40 bones) | LOD0/1 | 1K | 1.65m | Non-combatant. Blouse/simple dress or shirt + skirt/pants. 3 color variants. Generic face, longer hair (tied or loose). No tactical gear. Purpose: populate urban maps with civilians | 1 | P3 |
| **Pilot** | Flight suit, helmet, survival vest | 5,000-6,000 | Full (55 bones) | LOD0 | 2K | 1.78m | Downed pilot rescue mission character. CWU-27P flight suit (olive/tan, Nomex fabric texture visible), HGU-56/P helmet with visor (visors up/down two versions), PRC-112 survival radio on vest, survival vest with flares + knife + water pouch, GP-24 sidearm holster (M9/1911). Wear: flight suit has zipper wear, helmet scuffed from cockpit. Purpose: VIP extraction missions | 2 | P2 |
| **Zombie (Future/Siege)** | Military + civilian variants | 5,000 | Full (50 bones) | LOD0/1 | 2K | 1.78m | Future game mode. Decomposed/fresh corpse. Torn military uniform or civilian clothes. Blood saturation, exposed wounds/bone on face/arms, decayed skin tone (green-gray). Eyes: clouded/milky. Mouth: open/agape. Gait deformed. Not for base game, spec here for planning | 3 (Alive -> Damaged -> Dead) | Future |
| **Ghost (Future/Covert)** | Full black stealth gear, suppressed weapons | 6,000 | Full (55 bones) | LOD0/1 | 2K | 1.78m | Future DLC/special mode. Full black Crye suit, lightweight plate carrier (no markings), Ops-Core helmet with PSQ-36 NVG, suppressed MP5SD/HK416, no visible national identification or patches. Distinct: matte/non-reflective materials, light absorption texture | 2 | Future |
| **Total: 12 chars** | **~36 variants** | | | **~32 LODs** | | | | |
| Player Operator (Male) | Base + 3 gear variations | 8,000 | Full humanoid | LOD0/LOD1/LOD2 |
| Player Operator (Female) | Base + 3 gear variations | 7,500 | Full humanoid | LOD0/LOD1/LOD2 |
| Enemy Insurgent | Light gear, shemagh, AK | 5,000 | Full humanoid | LOD0/LOD1 |
| Enemy Mercenary | Plate carrier, helmet,战术 | 7,000 | Full humanoid | LOD0/LOD1 |
| Enemy Terrorist | Middle-eastern, kefiyah | 5,500 | Full humanoid | LOD0/LOD1 |
| Enemy PMC | Western tactical, NVG mount | 7,500 | Full humanoid | LOD0/LOD1 |
| AI Teammate | US-style, distinct camo | 7,500 | Full humanoid | LOD0/LOD1/LOD2 |
| Civilian (Male) | Shirt/pants, 3 color var | 3,000 | Simple humanoid | LOD0/LOD1 |
| Civilian (Female) | Dress/pants, 3 color var | 3,000 | Simple humanoid | LOD0/LOD1 |
| Pilot | Flight suit, helmet | 5,000 | Full humanoid | LOD0 |
| **Total: 10 characters** | **~30 variants** | | | **27 LOD files** |

#### Gear Attachments (equippable on characters)
Real-world scale, 1K-2K textures. All gear snaps to character skeleton sockets (Head, Spine, Spine1, Spine2, Hip, Thigh, Calf).

| Equipment | Variants | Description | Scale (cm) | Textures | Polys | Prod Priority |
|-----------|----------|-------------|------------|----------|-------|---------------|
| FAST Helmet | Base, NVG mount, NVG up/down | Special forces high-cut, rail system | 30 x 25 x 18 | 2K | 800 | P1 |
| ACH Helmet | Standard, camo cover | Standard issue, MICH cut | 30 x 25 x 20 | 2K | 700 | P1 |
| PASGT Helmet | Standard | Older, larger profile | 32 x 26 x 22 | 1K | 500 | P2 |
| Headset | Comtac III, M32, earplug | Communication/ear protection | 15 x 10 x 8 | 1K | 300 | P1 |
| Plate Carrier | JPC, IOTV, CPC, Slick | Different protection levels/weight | 45 x 35 x 10 | 2K | 1,200 | P1 |
| Chest Rig | 3-mag, 6-mag, AK config | Lightweight, no plates | 40 x 30 x 5 | 1K | 600 | P1 |
| Backpack | Assault 20L, Rucksack 60L, Medic | Capacity via model size | 45 x 30 x 20 | 2K | 800 | P1 |
| Med Pouch | IFAK, M9 bandage | Attaches to carrier MOLLE | 15 x 10 x 5 | 1K | 200 | P2 |
| Radio | MBITR, PRC-152 | Antenna, channel display | 10 x 5 x 3 | 1K | 200 | P2 |
| Drop Pouch | Single, double, triple | Magazine dump pouch | 20 x 15 x 5 | 1K | 150 | P3 |
| Pistol Holster | Drop-leg, belt, thigh | Sidearm retention | 20 x 10 x 5 | 1K | 300 | P2 |
| NVG | PVS-15, GPNVG-18, PSQ-36 | Night vision device, up/down position | 15 x 10 x 8 | 2K | 500 | P1 |
| Uniform | 6 patterns x 3 regions (top/pants/hat) | Multicam, Woodland, Desert, Urban, Black, Flecktarn | N/A (texture) | 2K | N/A | P1 |
| **Total: 13 types** | **~60 files** | | | | **~7,250 polys total** | |

### 12.2 Weapon Models

**Realism standard:** Photorealistic PBR with accurate real-world proportions, surface wear (holster marks, carbon buildup, scratched anodizing), and manufacturer-correct geometry. Textures: 2K for all weapons (4K for first-person view weapons). Scale: real-world centimeters.

Each weapon chassis requires separate models for every attachment variant so they display correctly in the gunsmith/workshop UI. Attachments must share consistent rail attachment points (Picatinny MIL-STD-1913 for sights/grips, M4 feed-lug pattern for barrels).

#### M4A1 Platform (Real width: 7cm, Length: 100cm stock extended / 90cm collapsed)
| Component | Variants | Tris | Textures | Realism Notes | Prod Priority |
|-----------|----------|------|----------|---------------|---------------|
| Upper Receiver | Standard, carry handle, flat-top | 1,000-1,200 | 2K (4K 1st person) | Forging seams, rail wear, carbon at gas port | P0 |
| Lower Receiver | Standard, ambi, marked | 700-900 | 2K | Trigger guard, selector markings, serial plate | P0 |
| Barrel | Standard, Suppressor, Compensator, Extended, Short | 400-600 each | 2K | Rifling visible at muzzle, thread protector | P0 |
| Handguard | M4 style, RIS, MLOK, Quad-rail | 600-800 each | 2K | Rail covers, heat shield, QD sling mount | P0 |
| Sight | Iron (rear diopter + front post), RedDot (reflex), Holo (EOTech), ACOG (4x), Sniper Scope (6x), CCO, Magnifier (3x) | 200-800 each | 1K-2K | Lens glass reflection, reticle illumination dot | P0 |
| Grip | A2 (standard), Vertical, Angled, Bipod, Stubby, HandStop | 200-500 each | 1K | Texture pattern, finger grooves | P0 |
| Magazine | USGI 30rd steel, PMAG 30rd, PMAG 40rd, PMAG 60rd (Drum), C-Mag 100rd | 300-600 each | 2K | Witness holes, floor plate, feed lips | P0 |
| Stock | Collapsible (4-pos), CTR, STR, PRS (precision), MOE, Fixed A2, No Stock | 300-600 each | 2K | Buttpad, QD cup, storage tubes | P0 |
| Bolt Carrier | Standard, lightweight | 300 | 1K | Gas key, firing pin retainer | P1 |
| Charging Handle | Standard, extended (ambidextrous) | 150 | 1K | Texture on latch | P1 |
| **Total M4A1** | **~35 files** | | | | |

#### MP5SD Platform (Real width: 5cm, Length: 78cm, Integral suppressor)
| Component | Variants | Tris | Textures | Realism Notes | Priority |
|-----------|----------|------|----------|---------------|----------|
| Upper Receiver | Standard integral suppressor, welded | 800-1,000 | 2K | Suppressor vent holes, welded seams | P0 |
| Lower Receiver | Standard, ambi, +SF (Navy trigger group) | 500-700 | 2K | Grip angle, trigger guard | P0 |
| Barrel | Integral (fixed), extended | 250-350 | 1K | Ported barrel inside suppressor | P1 |
| Handguard | Standard, wide (M203 mount), MLOK | 400-600 | 2K | Heat shield | P0 |
| Sight | Iron (drum rear), RedDot mount rail, Optic rail | 200-400 | 1K | Drum rear sight rotates | P1 |
| Grip | Standard (integral), finger groove, vertical | 200-300 | 1K | Integrated into lower | P1 |
| Magazine | 15rd, 30rd (curved), 40rd (straight) | 250-350 | 2K | Curved vs straight distinguishable | P0 |
| Stock | Fixed (A2), Collapsible (A3), Folding, Retractable (A3), No Stock | 300-500 each | 2K | Folding mechanism visible | P0 |
| **Total MP5SD** | **~20 files** | | | | |

#### M1911 Platform (Real width: 3cm, Length: 22cm 5in barrel)
| Component | Variants | Tris | Textures | Realism Notes | Priority |
|-----------|----------|------|----------|---------------|----------|
| Frame | Standard, railed, officer (3.5in) | 600-800 | 2K | Beavertail grip safety, slide release, thumb safety | P0 |
| Slide | Standard, compensated, optics cut (RMR) | 400-600 | 2K | Serrations, ejection port, loaded chamber indicator | P0 |
| Barrel | 5in standard, 4.25in commander, threaded | 200-300 | 1K | Muzzle crown, thread protector | P1 |
| Grip | Wood (checkered), rubber (wrap-around), G10 (textured), Magwell flared | 150-250 | 1K | Screw holes, medallion | P1 |
| Magazine | 7rd (flush), 8rd (extended), 10rd (competition) | 150-200 | 1K | Witness holes | P1 |
| Sight | Standard 3-dot, night sight (tritium), suppressor height (tall) | 50-80 | 1K | Tritium vials visible | P2 |
| **Total M1911** | **~16 files** | | | | |

#### AK-47 Platform (Real width: 7cm, Length: 88cm fixed stock)
| Component | Variants | Tris | Textures | Realism Notes | Priority |
|-----------|----------|------|----------|---------------|----------|
| Receiver | Stamped (Type 3), milled, sidefolder trunnion | 800-1,000 | 2K | Rivets, stamped dimples, selector markings (RUS) | P0 |
| Barrel | Standard, extended (RPK), short (AKS-74U), suppressed | 400-500 each | 2K | Front sight block, gas block, cleaning rod | P0 |
| Handguard | Wood (laminated), polymer (black), railed (Zenit), MLOK | 400-600 each | 2K | Wood grain, heat shield, rail sections | P0 |
| Sight | Iron (hooded front, tangent rear 100-1000m), red dot side mount, scope rail | 200-600 | 1K-2K | Battlefield zero marking, windage drum | P0 |
| Grip | Wood (bakelite), polymer (black), RPK (thicker) | 150-250 | 1K | Texture, palm swell | P1 |
| Magazine | Steel 30rd, Bakelite 30rd (orange-red), polymer 30rd (black), 40rd (RPK), 75rd drum | 300-500 each | 2K | Feed lips, magazine catch notch | P0 |
| Stock | Wood fixed, polymer fixed, AK-74M sidefolder (triangle), AR buffer tube adapter | 300-500 each | 2K | Buttstock trapdoor, cheek weld | P0 |
| Dust Cover | Standard, railed (Zenit B-33) | 250-350 | 1K | Rail slots, retention latch | P1 |
| **Total AK-47** | **~28 files** | | | | |

#### Future Weapons (Phase 10 DLC Packs — Model Files Needed)
| Weapon | Component Count | Est. Tris | Textures | Description |
|--------|----------------|-----------|----------|-------------|
| MK18 CQB | 22 | ~3,500 | 2K | 10.3in barrel, RIS II rail, close-quarters AR |
| SCAR-H | 28 | ~4,000 | 2K | 7.62 battle rifle, FN SCAR, reciprocating CH |
| M249 SAW | 16 | ~4,500 | 2K | Squad automatic weapon, box/drum fed, folding bipod |
| Remington 870 | 14 | ~2,500 | 2K | Pump shotgun, tube magazine, various stock/forend |
| G36C | 22 | ~3,000 | 2K | Compact carry-handle rifle, HK G36 variant |
| Kriss Vector | 16 | ~2,500 | 2K | Super V recoil system, .45 ACP SMG |
| Desert Eagle | 10 | ~2,000 | 2K | .50 AE hand cannon, gas-operated, large frame |
| Crossbow | 12 | ~2,000 | 2K | Silent, scope, tactical rail |
| **8 future weapons** | **~140 files** | | | |

**Total weapon files: ~240+** (Phase 3 base ~100 + Phase 10 DLC ~140)

### 10.3 Drone Models
| Model | Variants | Tris | Details |
|-------|----------|------|---------|
| Recon UAV | Base, damaged, exploded | 1,200 | 4-rotor quadcopter, camera payload, antenna, landing skids |
| FPV Strike Drone | Base, detonated | 600 | Racing quad frame, 4 small rotors, explosive payload (cylindrical) |
| Rotor Blade | Per drone (x8 for recon, x4 for FPV) | 50 each | Transparent with rotation animation |
| Camera Gimbal | Recon drone underslung | 200 | 2-axis gimbal with lens |
| **Total drone files** | **~8 files** | | |

### 10.4 Vehicle Models
| Vehicle | Variants | Tris | Features |
|---------|----------|------|----------|
| Technical (pickup + DShK) | Intact, destroyed, burning | 8,000 | Open bed, mounted HMG, civilian chassis |
| MRAP (RG-33) | Intact, destroyed | 15,000 | V-hull armor, turret, 6-wheel |
| Humvee (M1151) | Soft top, hard top, up-armored, destroyed | 10,000 | 4-door, weapon turret ring, cargo |
| UH-60 Black Hawk | Armed, transport, crashed | 25,000 | 2 pilots + 12 passengers, door guns, rotor animation |
| MH-6 Little Bird | Assault, AH-6 attack | 12,000 | 2 pilots + 6 passengers, rocket pods |
| Mi-24 Hind | Attack, transport | 30,000 | 2 pilots + 8 passengers, wing hardpoints |
| C-130 Hercules | Flyover only (low poly) | 5,000 | Background, skybox |
| A-10 Warthog | CAS flyover, static wreck | 8,000 | BRRRRT, GAU-8 visible |
| Civilian Sedan | 4 variants | 4,000 | Destroyable, traffic |
| Civilian SUV | 2 variants | 5,000 | Destroyable, traffic |
| Civilian Pickup | 2 variants | 4,500 | Destroyable, traffic |
| **Total vehicles** | **~20 files** | | |

### 10.5 Environment / Level Art

#### Modular Architecture Tiles
| Tile Type | Sizes | Materials | Count |
|-----------|-------|-----------|-------|
| Floor tile | 1m, 2m, 4m square | Concrete (new/broken), wood planks, gravel, dirt, asphalt | 15 |
| Wall segment | 2m x 4m, 4m x 4m | Concrete, brick, wood, sheet metal, sandbag, cinderblock | 12 |
| Corner piece | Inside 90, outside 90 | Per wall material | 8 |
| Door frame | Single, double, arched, vault | Metal, wood, heavy blast | 6 |
| Window frame | Small, large, barred, boarded | Per wall material | 8 |
| Stairs | Straight, L-shaped, spiral | Concrete, metal grate | 6 |
| Ramp | Concrete, dirt, metal | Various slopes | 4 |
| Roof tile | Flat, sloped, peaked | Concrete, tile, sheet metal | 6 |
| Ceiling tile | Drop ceiling, exposed beam | Office, warehouse, bunker | 4 |
| Column | Square, round, I-beam | Various sizes | 6 |
| Fence | Chain link, wood slat, wrought iron | 3 heights | 6 |
| **Total tiles** | **~80 files** | | |

#### Building Kits (prefabricated shells)
| Building | Interior | Floors | Count |
|----------|----------|--------|-------|
| 2-story concrete | Full interior, furniture | 2 | 1 |
| 3-story urban | Full interior, roof access | 3 | 1 |
| Shack (wood) | Small interior | 1 | 1 |
| Watchtower (wood) | Platform, ladder | 3m height | 1 |
| Bunker (concrete) | Room + firing ports | 1 | 1 |
| Warehouse | Large open, mezzanine | 2 | 1 |
| Construction site | Scaffolding, partial walls, dirt floor | 4 stories | 1 |
| Ruined building | Destroyed variant of 2-story | 1-2 | 1 |
| Mosque | Distinct architecture | 1 | 1 |
| Gas station | Canopy, shop interior | 1 | 1 |
| **Total buildings** | **~10 files** | | |

#### Environment Props
| Prop | Variants | Tris | Count |
|------|----------|------|-------|
| Barrel (55gal metal) | New, rusty, dented, burning, explosive (red) | 200 | 5 |
| Barrel (plastic) | Blue, white, yellow | 150 | 3 |
| Wooden crate | Small, medium, large, open, destroyed | 100-400 | 5 |
| Ammo crate | 5.56, 7.62, .50 cal, grenade | 300-500 | 4 |
| Plastic container | Small, large, stackable | 200 | 2 |
| Sandbag | Single, stack (3), wall (5), corner | 50-200 | 4 |
| Jersey barrier | Concrete, water-filled | 600 | 2 |
| Concrete block | 1m, 2m, L-shape | 400 | 3 |
| Tire | Car, truck, stack | 100-300 | 3 |
| Fuel can | Red (gas), yellow (diesel), olive (water) | 200 | 3 |
| Generator | Small (camp), large (industrial) | 800 | 2 |
| AC unit | Window, rooftop | 500 | 2 |
| Street lamp | Metal pole, wood pole, wall mount | 600 | 3 |
| Traffic cone | Standard, tall, reflective | 100 | 1 |
| Barrier (metal) | Highway, crowd control | 500 | 2 |
| Parking meter | Single | 200 | 1 |
| Fire hydrant | Standard | 300 | 1 |
| Mailbox | Standard, cluster | 200 | 2 |
| Bench | Wood, metal, concrete | 300 | 3 |
| Trash can | Metal, plastic | 200 | 2 |
| Signpost | Street sign, direction, warning | 150 | 3 |
| **Total props** | **~50 files** | | |

#### Natural Environment
| Asset | Variants | Tris | LODs | Count |
|-------|----------|------|------|-------|
| Ground texture | Sand, dirt, gravel, grass, mud, asphalt, concrete, rock, snow, tile, wood, metal | 2048x2048 | N/A | 50+ |
| Tree (Palm) | Small, medium, large, dead | 800-3,000 | 3 | 4 |
| Tree (Pine) | Small, medium, large, dead | 600-2,500 | 3 | 4 |
| Tree (Oak) | Small, medium, large, dead | 1,000-4,000 | 3 | 4 |
| Tree (Desert) | Cactus, Joshua, scrub | 400-1,000 | 2 | 3 |
| Bush | Small, large, round, spiky | 200-600 | 2 | 8 |
| Rock | Small (0.3m), medium (1m), large (2m), boulder (5m) | 100-2,000 | 2 | 8 |
| Grass patch | 0.5m circle, 1m square | 50-200 | 1 | 4 |
| Flowers | Small cluster, large patch | 100-300 | 1 | 4 |
| Log | Fallen tree, stump | 300-800 | 2 | 3 |
| **Total natural** | **~55 files** | | | |

#### Decals
| Decal | Variants | Resolution | Count |
|-------|----------|------------|-------|
| Bullet hole (concrete) | 4 variants | 128x128 | 4 |
| Bullet hole (wood) | 4 variants | 128x128 | 4 |
| Bullet hole (metal) | 4 variants | 128x128 | 4 |
| Bullet hole (dirt) | 2 variants | 128x128 | 2 |
| Blood splat (wall) | Small, medium, large, spray | 256x256 | 8 |
| Blood pool (floor) | Small, medium, large | 256x256 | 3 |
| Crack (concrete) | 4 variants | 256x256 | 4 |
| Graffiti | 8 variants | 256x256 | 8 |
| Mud footprint | Left, right, multiple | 128x128 | 4 |
| Burn mark | Small, large, vehicle | 256x256 | 3 |
| **Total decals** | **~45 files** | | |

### 10.6 Audio Assets — Complete Inventory

#### Weapon Sounds
| Weapon | Sound Type | Close | Medium | Far | Suppressed |
|--------|-----------|-------|--------|-----|------------|
| M4A1 | Fire | X | X | X | X |
| MP5SD | Fire | X | X | X | X (integral) |
| M1911 | Fire | X | X | X | X |
| AK-47 | Fire | X | X | X | X |
| M249 (future) | Fire | X | X | X | X |
| SCAR-H (future) | Fire | X | X | X | X |
| Remington 870 (future) | Fire | X | X | X | N/A |
| Desert Eagle (future) | Fire | X | X | X | X |
| Crossbow (future) | Fire | X | X | N/A | N/A |
| **Total weapon fire** | | **27 files** | | | |

#### Weapon Foley (per weapon)
| Sound | Description | Count |
|-------|-------------|-------|
| Reload start | Mag release, old mag out | 6 |
| Reload insert | New mag in, slap | 6 |
| Bolt release | Forward assist, bolt catch | 6 |
| Weapon draw | Unholster, unsling | 4 |
| Weapon holster | Holster, sling | 4 |
| Inspection | Weapon rotate, handle check | 4 |
| Dry fire | Trigger click (no ammo) | 6 |
| **Total foley** | **36 files** | |

#### Bullet Impacts
| Surface | Variants | Count |
|---------|----------|-------|
| Concrete | 4 | 4 |
| Wood | 4 | 4 |
| Metal | 4 | 4 |
| Dirt | 3 | 3 |
| Flesh | 3 | 3 |
| Water | 2 | 2 |
| Glass | 2 | 2 |
| Bullet whiz (near miss) | 4 | 4 |
| Shell casing bounce | 3 | 3 |
| **Total impacts** | **28 files** | |

#### Footsteps
| Surface | Walk | Run | Sprint | Crouch | Count |
|---------|------|-----|--------|--------|-------|
| Gravel | 3 | 3 | 3 | 2 | 11 |
| Concrete | 3 | 3 | 3 | 2 | 11 |
| Metal | 3 | 3 | 3 | 2 | 11 |
| Grass | 3 | 3 | 3 | 2 | 11 |
| Dirt | 3 | 3 | 3 | 2 | 11 |
| Mud | 2 | 2 | 2 | 1 | 7 |
| Wood | 2 | 2 | 2 | 1 | 7 |
| Tile | 2 | 2 | 2 | 1 | 7 |
| Water | 2 | 2 | 2 | 1 | 7 |
| Carpet | 2 | 2 | 2 | 1 | 7 |
| **Total footsteps** | **90 files** | | | |

#### Foley (Movement)
| Sound | Variants | Count |
|-------|----------|-------|
| Equipment rattle (sprint) | 4 | 4 |
| Equipment rattle (land) | 2 | 2 |
| Equipment rattle (vault) | 2 | 2 |
| Clothing movement (crouch) | 2 | 2 |
| Clothing movement (prone) | 2 | 2 |
| Fabric turn | 2 | 2 |
| **Total movement foley** | **14 files** | |

#### Ambient / Environmental
| Sound | Variants | Loop? | Count |
|-------|----------|-------|-------|
| Wind (light) | 2 | Yes | 2 |
| Wind (moderate) | 2 | Yes | 2 |
| Wind (strong/storm) | 2 | Yes | 2 |
| Rain (light) | 1 | Yes | 1 |
| Rain (moderate) | 1 | Yes | 1 |
| Rain (heavy/storm) | 1 | Yes | 1 |
| Thunder | 4 | No | 4 |
| Indoor HVAC hum | 2 | Yes | 2 |
| Electrical buzz | 2 | Yes | 2 |
| Distant machinery | 2 | Yes | 2 |
| Birds (day) | 2 | Yes | 2 |
| Birds (dusk) | 2 | Yes | 2 |
| Insects (night) | 2 | Yes | 2 |
| Distant combat | 3 | Yes | 3 |
| City traffic | 2 | Yes | 2 |
| Radio static | 2 | Yes | 2 |
| Drone buzz (Recon) | 2 | Yes | 2 |
| Drone buzz (FPV) | 2 | Yes | 2 |
| Water (river flow) | 2 | Yes | 2 |
| Water (ocean waves) | 2 | Yes | 2 |
| Fire (crackling) | 2 | Yes | 2 |
| Fire (burning building) | 2 | Yes | 2 |
| **Total ambient** | **42 files** | | |

#### UI Sounds
| Sound | Variants | Count |
|-------|----------|-------|
| Button hover | 2 | 2 |
| Button click | 2 | 2 |
| Button back | 1 | 1 |
| Error / invalid | 1 | 1 |
| Menu open | 2 | 2 |
| Menu close | 2 | 2 |
| Match countdown beep | 2 | 2 |
| Round start horn | 1 | 1 |
| Round end | win/loss | 2 |
| Match end | win/loss/mvp | 4 |
| Kill confirmed | 2 | 2 |
| Headshot kill | 2 | 2 |
| Double kill | 1 | 1 |
| Multi-kill (3-5) | 3 | 3 |
| Ace | 1 | 1 |
| XP gained | 2 | 2 |
| Level up | 1 | 1 |
| Achievement unlock | 1 | 1 |
| Chat message sent | 1 | 1 |
| Chat message received | 1 | 1 |
| Bomb plant beep | 1 | 1 |
| Bomb defuse progress | 1 | 1 |
| Bomb explosion | 1 | 1 |
| Bomb timer warning (10s) | 1 | 1 |
| **Total UI** | **36 files** | | |

#### Voice Assets
| Category | Lines | Per Character | Total |
|----------|-------|---------------|-------|
| Teammate callouts (enemy spotted, reloading, covering, moving, need backup, etc.) | 20 | 3 teammates | 60 |
| Command responses (affirmative, negative, moving, engaging, in position) | 10 | 3 teammates | 30 |
| Enemy callouts (spotted enemy, taking fire, falling back, grenade, contact!) | 15 | 2 enemy types | 30 |
| Radio chatter (mission brief, status report, extraction inbound, danger close) | 10 | 1 controller | 10 |
| Pain sounds (light hit, heavy hit, death) | 6 | 5 character types | 30 |
| Contact reports (contact front/left/right, multiple hostiles) | 8 | 3 teammates | 24 |
| **Total voice** | **~70 unique lines** | | **~184 files** |

### 10.7 Visual Effects (Particle Assets)

| Effect | Technique | Count | Phase |
|--------|-----------|-------|-------|
| Muzzle flash (M4A1) | Point light + sprite burst + glow | 1 | 3 |
| Muzzle flash (MP5SD) | Smaller flash + spark | 1 | 3 |
| Muzzle flash (M1911) | Sharp pop flash | 1 | 3 |
| Muzzle flash (AK-47) | Large flash + fire burst | 1 | 3 |
| Bullet impact (concrete) | Sparks + dust puff + chips | 4 | 3 |
| Bullet impact (wood) | Splinters + dust | 3 | 3 |
| Bullet impact (metal) | Sparks + ring | 3 | 3 |
| Bullet impact (dirt) | Dirt puff | 2 | 3 |
| Bullet impact (flesh) | Blood mist + spray | 2 | 3 |
| Bullet impact (water) | Water splash + ripple | 2 | 3 |
| Blood splatter (entry) | Small blood burst | 1 | 3 |
| Blood splatter (exit) | Large blood spray | 1 | 3 |
| Blood pool (floor) | Expanding pool | 1 | 3 |
| Explosion (grenade) | Fireball + smoke + debris + dust ring | 1 | 5 |
| Explosion (vehicle) | Large fireball + black smoke + parts | 1 | 5 |
| Explosion (drone FPV) | Mid fireball + shrapnel | 1 | 3 |
| Explosion (C4 planted) | Massive fireball + shockwave | 1 | 5 |
| Smoke (smoke grenade) | Expanding white cloud | 1 | 5 |
| Smoke (vehicle fire) | Black column, rising | 1 | 5 |
| Smoke (building fire) | Gray plume | 1 | 5 |
| Fire (muzzle ongoing) | Small flickering flame | 1 | 3 |
| Fire (barrel fire) | Medium flame + smoke | 1 | 3 |
| Fire (vehicle wreck) | Large flame + black smoke | 1 | 5 |
| Dust (footstep) | Per surface | 5 | 3 |
| Dust (vehicle) | Tire dust trail | 1 | 5 |
| Dust (explosion) | Ring expanding outward | 1 | 5 |
| Dust (building collapse) | Massive cloud | 1 | 8 |
| Glass (bullet crack) | Crack sprite on glass surface | 1 | 3 |
| Glass (break) | Shatter + falling shards | 1 | 3 |
| Water (bullet impact) | Small splash | 1 | 3 |
| Water (footstep) | Ripple ring | 1 | 3 |
| Water (dive) | Large splash | 1 | 5 |
| Water (surface ripple) | Continuous ripple on surface | 1 | 5 |
| Debris (concrete) | Concrete chunks flying | 3 | 3 |
| Debris (wood) | Wood splinters | 2 | 3 |
| Debris (dirt) | Dirt clods | 2 | 3 |
| Tracer (FMJ) | White/gray streak | 1 | 3 |
| Tracer (HP) | Yellow/orange streak | 1 | 3 |
| Tracer (AP) | Red streak | 1 | 3 |
| Tracer (Tracer) | Bright green streak + glow | 1 | 3 |
| Hit marker (enemy hit) | White X flash | 1 | 3 |
| Hit marker (kill) | Red X flash | 1 | 3 |
| Hit marker (headshot) | Gold X flash + shatter | 1 | 3 |
| Suppression effect | Full-screen edge blur + vignette darken | 1 | 2 |
| Damage vignette | Red edge crawl proportional to damage | 1 | 2 |
| Death effect | Desaturate + fade to black | 1 | 2 |
| Night vision | Green tint + overlay grain + glow bloom | 1 | 7 |
| Thermal vision | Heat signature color ramp | 1 | 7 |
| **Total VFX** | **~75 files** | | |

### 10.8 Animation Assets

| Animation | Characters | Types | Total Clips |
|-----------|-----------|-------|-------------|
| Idle | All humanoid | Standing / Crouched / Prone / Cover | 6 |
| Walk | All humanoid | Forward / Backward / Left / Right (per stance: stand/crouch/prone) | 12 |
| Run / Sprint | All humanoid | Forward only | 2 |
| Crouch Walk | All humanoid | Forward / Backward / Strafe | 4 |
| Prone Crawl | All humanoid | Forward only | 1 |
| Stance Transitions | All humanoid | Stand<->Crouch, Stand<->Prone, Crouch<->Prone, Sprint->All | 7 |
| Jump | All humanoid | Start / Airborne / Land | 3 |
| Vault | All humanoid | Low (0.5m) / High (1.0m) | 2 |
| Weapon Fire | All humanoid | Per weapon type (rifle/pistol/smg), per stance | 12 |
| Reload | All humanoid | Per weapon type (tactical/empty) | 8 |
| Weapon Swap | All humanoid | Primary to Sidearm / Sidearm to Primary | 2 |
| Aim Down Sights | All humanoid | Raise / Lower / Hold | 3 |
| Melee | All humanoid | Knife slash / Gun butt strike | 2 |
| Throw | All humanoid | Overhand grenade / Underhand flashbang | 2 |
| Death | All humanoid | Front / Back / Left / Right / Headshot / Explosion | 6 |
| Hit Reaction | All humanoid | Front / Back / Left / Right / Stagger | 5 |
| Holster/Unholster | All humanoid | Sling rifle / Draw pistol | 2 |
| Interact | All humanoid | Plant bomb / Defuse / Open door / Climb ladder | 4 |
| Drone Flight | Drones | Hover / Bank left/right / Ascend / Descend | 4 |
| Drone Detonate | FPV Drone | Explosion burst | 1 |
| Vehicle Idle | All vehicle | Engine running, wheels stationary | 1 per vehicle |
| Vehicle Drive | All vehicle | Wheels rotating (based on speed) | 1 per vehicle |
| Helicopter Rotor | All heli | Idle spin (low) / Flight spin (high) | 2 per heli |
| Vehicle Enter/Exit | All humanoid | Open door / Enter / Exit per vehicle type | 18 |
| **Total animations** | | | **~120 clips** |

**Animation retargeting requirements:**
- All humanoid characters must share the same bone hierarchy (UE4 Mannequin or Mixamo standard)
- Required IK targets: LeftFoot, RightFoot, LeftHand, RightHand, Head
- Aim constraint system: Upper spine + neck + head for independent aiming while moving
- Root motion for all locomotion (not in-place)

### 10.9 UI & Texture Assets

| Category | Assets | Resolution | Count |
|----------|--------|------------|-------|
| Crosshair | Dot, Cross, Circle, Chevron, custom color variants (red/green/white/yellow) | 64x64 | 20 |
| Weapon icons | Per weapon chassis (4 base + 8 future) | 256x256 | 12 |
| Attachment icons | Per attachment (barrel/sight/grip/mag/stock = 22 options) | 128x128 | 22 |
| Gear icons | Per equipment (helmet/armor/tactical = 13 types) | 128x128 | 13 |
| Drone icons | Recon UAV, FPV Strike | 128x128 | 2 |
| Objective markers | A site, B site, extraction, spawn, enemy last seen, waypoint | 64x64 | 8 |
| HUD elements | Health bar frame, stamina bar, ammo background, kill feed bg, crosshair base | 512x64 | 8 |
| Menu backgrounds | Main menu, settings, loading, match results | 1920x1080 | 6 |
| Button textures | Normal, hover, pressed, disabled, selected | 256x64 | 10 |
| Map tiles | Minimap terrain tiles (512x512 per tile) | 512x512 | TBD |
| Loading screens | Mission loading, map loading, connecting | 1920x1080 | 6 |
| Achievement cards | Per achievement (7) with locked/unlocked | 512x256 | 14 |
| Specialization icons | Assault, Medic, Engineer, Recon | 128x128 | 4 |
| Rarity frames | Common/Uncommon/Rare/Epic/Legendary border frames | 256x64 | 5 |
| Kill feed icons | Headshot, multi-kill, bomb plant, defuse | 32x32 | 6 |
| **Total UI textures** | | | **~135 files** |

### 10.10 Pipeline Summary — Total Asset Count

| Category | Phase 3 Minimum | Full Game |
|----------|----------------|-----------|
| Character models | 4 | 10 |
| Character gear/outfits | 10 | 60 |
| Weapon files | 60 | 225+ |
| Drone files | 4 | 8 |
| Vehicle models | 0 | 20 |
| Modular tiles | 30 | 80 |
| Building kits | 3 | 10 |
| Environment props | 20 | 50 |
| Natural environment | 20 | 55 |
| Decals | 15 | 45 |
| Audio files | 80 | 390+ |
| VFX particles | 20 | 75 |
| Animation clips | 30 | 120 |
| UI textures | 40 | 135 |
| **Grand total** | **~336 assets** | **~1,283 assets** |

## 13. Project Metrics (Current)

| Metric | Value |
|--------|-------|
| Source files | 91 across 5 crates |
| Module directories | 19 in game crate |
| Rust lines | ~9,000+ |
| Build errors | 0 |
| Clippy warnings | 78 (all dead_code, intentional pre-asset) |
| External packages | 12 (3 active, 5 wired-inactive, 4 Phase 6+ deps) |
| Game messages | 14 |
| Achievements | 7 |
| Specializations | 4 |
| Weapon chassis | 4 (M4A1, MP5SD, M1911, AK-47) |
| Weapon attachment types | 5 (barrel, sight, grip, mag, stock) |
| Attachment options | 22 total across all types |
| Calibers | 7 (9mm through .50 BMG) |
| Ammo types | 4 (FMJ, HP, AP, Tracer) |
| Drone types | 2 (Recon, FPV Strike) |
| Squad orders | 4 (Move, Engage, Suppress, Regroup) |
| HUD subsystems | 9 |
| Objective types | 5 |