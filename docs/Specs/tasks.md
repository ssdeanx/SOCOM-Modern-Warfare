---
title: "SOCOM Tactical Shooter — Tasks & Milestones"
version: "2.0.0"
date: "2026-06-19"
author: "Sam / Quicksilver"
status: "Active — Phase 2 Complete, Pre-Asset"
license: "MIT"
metadata:
  timestamp: "2026-06-19T00:00:00Z"
  datestamp: "2026-06-19"
  completed_phase: "Phase 0 + 1 + 2 — All Core Systems"
  current_milestone: "Phase 3 — Asset Integration & Polish"
  next_milestone_eta: "TBD"
---

# Tasks & Milestones

## Milestone Overview

| Phase | Title | Status | Tasks |
|-------|-------|--------|-------|
| 0 | Foundation | ✅ Complete | 11 (T1-T11) |
| 1 | Core Systems | ✅ Complete | 12 (T12-T23) |
| 2 | Advanced Systems | ✅ Complete | 15 (A1-A15) |
| 3 | Asset Integration & Polish | 🟡 Active | See below |
| 4 | Production Ready | ⏳ Planned | — |

### Phase 2 — Advanced Systems (In Progress)

### A1 — Modular Weapon System (8 files) ✅
- [x] **weapons/chassis.rs:** 4 weapon platforms with class, caliber, base stats
- [x] **weapons/caliber.rs:** 7 calibers 9mm → .50 BMG with multipliers
- [x] **weapons/barrel.rs:** 5 barrel types with damage/recoil/spread
- [x] **weapons/sight.rs:** 5 sight types with ADS/spread/zoom
- [x] **weapons/underbarrel.rs:** 4 grip types with recoil/spread
- [x] **weapons/magazine.rs:** 4 mag types with capacity/reload
- [x] **weapons/stock.rs:** 4 stock types with recoil/sway/ADS
- [x] **weapons/mod.rs:** CompleteWeapon combines chassis × all attachments → final stats
- [ ] **Sniper rifle chassis (M24 SWS):** Add SNIPER_RIFLE class, M24 (bolt 7.62, 5rd int, 800m) and L96A1 (bolt 7.62, 10rd detach, 900m) with bipod and scope (6x/12x/16x) support

### A2 — Weapon Handling System ✅
- Weight classes: Light (1.0x speed), Medium (0.75x), Heavy (0.55x)
- ADS time per weight class: 0.12s / 0.25s / 0.40s
- Sway amplitude per weight class

### A3 — Stamina System ✅
- 100 max, drains at 25/s sprinting, regens at 15/s after 1.5s delay
- Exhausted: 0.6x speed, 2.5x sway, 2.0x spread
- Partial: tiers at 30% and 60% with progressive penalties

### A4 — Turn Rate Limiting ✅
- Stance: Prone 0.3x, Crouch 0.6x, Sprint 0.7x, Standing 1.0x
- Weapon weight: Heavy ≈ 0.75x turn rate
- Stamina exhaustion: 0.6x multiplier
- Combined value stored in SensitivityMultiplier resource

### A5 — Cover Detection + Suppression ✅
- Cover: raycast in 4 cardinal directions, 0.6m detection, emits CoverStateMessage
- Suppression: builds on damage taken (20 per hit), decays over 2s, affects spread

### A6 — Command Wheel + Squad Orders ✅
- Tab key opens radial menu with 4 orders: MOVE, ENGAGE, SUPPRESS, REGROUP
- SquadOrderMessage dispatched to all teammates
- ActiveOrders resource tracks per-entity state
- Formation system positions teammates relative to player

### A7 — Camera Overhaul: 1st/3rd Person + ADS ✅
- V key / Middle Mouse toggles between 1st and 3rd person
- Perspective interpolates smoothly (lerp factor 0.12/frame)
- ADS zoom works in both perspectives (-15° FOV)
- Separate base FOV for 3rd (70°) and 1st (80°) person
- Collision raycast for 3rd person, disabled in 1st person
- PerspectiveState resource queryable by other systems

### A8 — Drone System: Recon + FPV Strike ✅
- Recon UAV (U key): 120 battery, 40s flight, 8m altitude, auto-return
- FPV Strike (J key): 30 battery, 10s flight, 25 m/s, detonation (Space)
- Battery management, recharge when docked, velocity-based flight

### A9 — Breathing System ✅
- Hold breath (Aim + Sprint) steadies weapon up to 50% sway reduction
- Drains stamina at 8/s, 2s cooldown between holds

### A10 — Mission System ✅
- 5 objective types: EliminateAll, ReachLocation, DefendPosition, CollectIntel, Extract
- Auto-progresses on kill events, completion detection
- MissionState resource, HUD objective display

### A11 — Ammo Type System ✅
- 4 types: FMJ (balanced), HP (+25% dmg, -50% pen), AP (-15% dmg, +80% pen), Tracer (-5% dmg)
- LoadedAmmo component for tracking

### A12 — Expanded HUD (9 subsystems) ✅
- Health bar, stance indicator, ammo counter, crosshair, weapon name
- XP notifications (+50 XP popup with 2s timer)
- Level-up notifications (green "LEVEL UP!" with 3s timer)
- Stamina bar (small blue bar below health)
- Achievement popups (golden "Achievement:" with 4s display)
- Kill feed (top-right, shows "Killed Enemy" / "You died", 5s cleanup)
- Squad status (top-left, shows teammate health + orders)
- Objective text (top-center, shows mission progress)

### A13 — Gear + Progression Systems ✅
- 5-slot inventory: Primary, Sidearm, Helmet, BodyArmor, TacticalGear
- 5 rarities: Common → Legendary (1.0x → 1.6x stats)
- Attachment workshop with Sight/Barrel/Grip/Magazine/Stock slots
- Workshop modifiers apply to weapon stats (damage, spread, magazine)
- Inventory damage bonus applied in shooting system
- XP system: 50 XP/kill, level = sqrt(xp/100) scaling
- Stats tracking: kills, deaths, shots fired/hit, damage, accuracy
- 7 achievements: FirstBlood, DoubleTap, Survivor, Headhunter, Unstoppable, Gunsmith, Perfectionist
- 4 specializations: Assault, Medic, Engineer, Recon with stat modifiers

### A14 — Save/Load System ✅
- F5 quicksave, F9 quickload
- Pause menu Save/Load buttons
- Complete serialization: progression, stats, inventory, achievements, settings
- Save path: ~/.socom/save.ron

### A15 — Physics Configuration ✅
- Physics runs at 120Hz fixed timestep
- MoveAndSlide character controller with proper ramp/stairs
- Camera-relative movement

### A16 — Equipment System (Building)
**24 subtasks — see requirements.md 4.5 for full specs**
- EquipmentType enum: frag, smoke, flash, stun, impact, claymore, mine, C4, sensor, knife, throwing knife, bandage, medkit, defib, surgery
- EquipmentInventory resource: throwables/deployables/melee/healing slots
- G-tap cycle + G-hold deploy/throw
- 5 grenade types with fuse, effect radius, damage falloff
- 4 deployable types (claymore, mine, C4, sensor) with trigger systems
- Combat knife (primary+heavy+lung) + throwing knife (projectile+retrievable)
- Equipment trigger system: tripwire, pressure plate, remote detonation, proximity

### A17 — Medic & Healing System (Building)
**14 subtasks — see requirements.md 4.6 for full specs**
- Health component expansion: armor, is_downed, bleed_out, instant_death, heal_timer
- Bandage (25 HP/3s), Medkit (75 HP/6s), Defibrillator (revive 50 HP/4s), Surgery Kit (100%/10s)
- Self-heal + teammate heal + revive mechanics
- Bleed-out system (30s timer, headshot=instant death)
- Damage interrupt (2s block after hit, hit during heal = waste item)
- Specialization healing bonuses table
- Healing animations and HUD

### A18 — Advanced Drones (Building)
**6 subtasks — see design.md 5.5 for full specs**
- Grenade Drone (H): 4x frag hardpoints, waypoint flight, SPACE to drop
- Mine Drone (N): 3x mine dispenser, LINE/TRIANGLE/CIRCLE patterns, G to deploy
- Drone countermeasures: one-hit destroy, minimap detection (30m)
- Drone HUD: battery, payload, altitude, signal, destroyed indicator
- Auto-return logic (payload expended, low battery, out of range)

### A19 — Sniper Rifle Chassis (Building)
**6 subtasks — see weapons system**
- SNIPER_RIFLE class, SNIPER_WEIGHT weight class
- M24 SWS: bolt 7.62mm, 5rd internal, 800m, 1.2s cycle
- L96A1: bolt 7.62mm, 10rd detach, 900m, 1.4s cycle
- Scope expansion: 6x/12x/16x zoom, zeroing (100-800m Page Up/Down)
- Scope-in time 0.8s, hold breath sway reduction 70%
- Sniper damage: 100 body, 300 headshot, passes through thin cover

## Phase 3 — Asset Integration & Polish (NEXT)

**Total asset scope:** ~336 minimum needed for playable build / ~1,283 for full game.
See requirements.md Section 12 for complete asset-by-asset inventory.

### P1 — Placeholder Audio Assets
| ID | Task | Est. | Deps |
|----|------|------|------|
| P1.1 | Generate 15 footstep .ogg (gravel/concrete/metal/grass/dirt x walk/sprint/crouch) | 30m | None |
| P1.2 | Generate 12 weapon .ogg (M4A1, MP5SD, M1911, AK-47 fire/suppressed/dry) | 30m | None |
| P1.3 | Generate 4 ambient .ogg (wind, indoor, drone buzz, radio static) | 15m | None |
| P1.4 | Generate 8 UI .ogg (click, hover, match start/end, kill, chat ping, error) | 15m | None |
| P1.5 | Verify all files load without warnings | 10m | P1.1-P1.4 |

### P2 — Procedural Weapon Models
| ID | Task | Est. | Deps |
|----|------|------|------|
| P2.1 | Create box-based weapon mesh hierarchy (receiver, barrel, sight, grip, mag, stock) | 1h | None |
| P2.2 | Attach weapon to camera in 1st person (bottom-right, ADS position) | 1h | P2.1 |
| P2.3 | Attach weapon to player in 3rd person (holster + firing position) | 1h | P2.1 |
| P2.4 | Implement weapon swap animation (hide primary, show sidearm) | 30m | P2.2, P2.3 |
| P2.5 | Shoulder mirror (weapon flips sides with camera Q swap) | 30m | P2.2 |

### P3 — bevy_hanabi VFX
| ID | Task | Est. | Deps |
|----|------|------|------|
| P3.1 | Muzzle flash: yellow point light + expanding sphere burst (0.05s) | 1h | None |
| P3.2 | Bullet impact: orange/white sparks along hit normal | 1h | None |
| P3.3 | Hit marker flash: red sphere on enemy damage (0.08s) | 30m | None |
| P3.4 | Death explosion: 6 particles burst, 1s life, gravity-affected | 1h | None |
| P3.5 | Tracer: glowing sphere trail, 0.3s lifetime | 30m | None |

### P4 — Post-Processing
| ID | Task | Est. | Deps |
|----|------|------|------|
| P4.1 | ACES filmic tone mapping | 30m | None |
| P4.2 | Bloom for explosions + muzzle flash | 1h | P4.1 |
| P4.3 | SSAO for tactical depth perception | 1h | P4.1 |
| P4.4 | Depth of field for ADS blur | 30m | P4.1 |

### P5 — Kira Audio Integration
| ID | Task | Est. | Deps |
|----|------|------|------|
| P5.1 | Wire kira as primary audio backend replacing bevy_audio | 2h | None |
| P5.2 | Create bus hierarchy: Master > SFX/Ambient/UI/Voice | 1h | P5.1 |
| P5.3 | Spatial audio for footsteps + weapon fire | 2h | P5.1 |
| P5.4 | Audio occlusion for suppressed weapons | 1h | P5.2, P5.3 |

### P5.5 — Destruction System
| ID | Task | Est. | Deps |
|----|------|------|------|
| P5.5.1 | DestructionState component with damage accumulation + state machine | 3h | None |
| P5.5.2 | Material penetration table (8 materials x 6 calibers) | 2h | P5.5.1 |
| P5.5.3 | Ballistic penetration system (bullet passes through thin materials) | 4h | P5.5.2 |
| P5.5.4 | Explosion damage system (spherical falloff, structural transfer) | 3h | P5.5.1 |
| P5.5.5 | Debris spawning (per material type, 10 debris types) | 3h | P5.5.4 |
| P5.5.6 | Building collapse animation (progressive sag -> dust -> rubble) | 4h | P5.5.4 |
| P5.5.7 | Glass fracture + shatter system (pre-score lines, fall shards) | 2h | None |
| P5.5.8 | Vehicle damage states (4 states: operational -> disabled -> burning -> wreck) | 3h | P5.5.1 |
| P5.5.9 | Networked damage state sync (replicated component, state transitions only) | 2h | P5.5.1, Phase 6 |
| P5.5.10 | Destruction audio (5 material destruction sounds, 3 concurrent limit) | 1h | P5.5.4 |

### P6 — Character Models (Blender MCP)
| ID | Task | Est. | Deps |
|----|------|------|------|
| P6.1 | Player operator: Multicam uniform, plate carrier, FAST helmet, M4 slung, 1.80m, 8-10K tris, 4K textures, 65-bone skeleton | 4h | None |
| P6.2 | Enemy insurgent: loose shirt, chest rig, shemagh, AK slung, 1.75m, 5-6K tris, 2K textures, 55-bone skeleton | 3h | None |
| P6.3 | Enemy mercenary: full plate carrier, tactical helmet, EMR camo, AK-74M, 1.85m, 7-8K tris, 2K textures | 3h | None |
| P6.4 | AI teammate: same base as player operator, distinct squad marking + different camo pattern, 8-10K tris | 2h | P6.1 |
| P6.5 | Ragdoll death physics (rigidbody-per-bone prefab, collision/solver setup) | 2h | P6.1-P6.4 |

### P7 — Weapon Models (Blender MCP)
| ID | Task | Est. | Deps |
|----|------|------|------|
| P7.1 | M4A1 base + attachment variants (barrel/sight/grip/mag/stock) | 3h | None |
| P7.2 | MP5SD integral suppressed variant | 1.5h | None |
| P7.3 | M1911 pistol with rail attachment point | 1h | None |
| P7.4 | AK-47 with wood furniture | 1.5h | None |

### P8 — Level Art
| ID | Task | Est. | Deps |
|----|------|------|------|
| P8.1 | Replace greybox with GLTF static meshes | 2h | None |
| P8.2 | Modular tile system: 1m floor, wall segments, corners | 2h | None |
| P8.3 | Lighting: directional sun, ambient probes | 1h | P8.1 |
| P8.4 | Collision mesh generation for static geometry | 1h | P8.1 |

## Phase 4 — Training & Practice

### T4.1 Training Range Map
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.1.1 | Open area design with shooting lanes (10m/25m/50m/100m marked) | 1h | None |
| T4.1.2 | Cover positions (low wall, corner, window) | 1h | None |
| T4.1.3 | Drone flight zone with obstacle course | 1h | None |

### T4.2 Target System
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.2.1 | Static paper silhouettes at marked distances | 30m | None |
| T4.2.2 | Steel plate targets with audio ping feedback | 30m | None |
| T4.2.3 | Headshot-only precision targets | 30m | None |
| T4.2.4 | Pop-up targets on random timers (0.5-3s exposure) | 1h | None |
| T4.2.5 | Moving targets: lateral (3m/s), advancing (2m/s), random path | 1h | None |

### T4.3 Weapon Locker UI
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.3.1 | Full weapon list: all chassis + all attachments | 1h | None |
| T4.3.2 | Live computed stats display during selection | 1h | None |
| T4.3.3 | Ammo mode toggle (infinite/finite) | 30m | None |

### T4.4 Aim Trainer
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.4.1 | Accuracy percentage per session | 30m | None |
| T4.4.2 | DPS meter (damage per second) | 30m | None |
| T4.4.3 | Reaction time tracker (target appear to hit) | 30m | None |

### T4.5 Drone Practice
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.5.1 | Dedicated drone recharge station | 30m | None |
| T4.5.2 | Target markers for recon marking practice | 30m | None |
| T4.5.3 | FPV dummy targets for detonation practice | 30m | None |

### T4.6 SQB — Squad Bots Mode
| ID | Task | Est. | Deps |
|----|------|------|------|
| T4.6.1 | Bot difficulty system (Easy 0.5x / Medium 1.0x / Hard 1.5x) | 1h | None |
| T4.6.2 | Bot count: fill empty player slots with AI | 1h | None |
| T4.6.3 | Any game mode playable vs bots | 1h | T4.6.1, T4.6.2 |

## Phase 5 — Competitive Game Modes

### T5.1 Team Deathmatch
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.1.1 | Round manager: 5v5, 5-6 min timer, sudden death | 2h | None |
| T5.1.2 | Win condition: eliminate all OR time-up last standing | 1h | T5.1.1 |
| T5.1.3 | Scoreboard: kills, deaths, assists, damage | 1h | None |
| T5.1.4 | Best-of-9 match with round tracking | 1h | T5.1.1 |

### T5.2 Demolition
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.2.1 | Bomb plant mechanic (hold E on site, 3s plant) | 1.5h | None |
| T5.2.2 | Bomb defuse mechanic (hold E, 7s defuse) | 1.5h | T5.2.1 |
| T5.2.3 | Two bomb sites: A and B with distinct layout | 1h | None |
| T5.2.4 | Halftime switch after 4 rounds | 30m | None |
| T5.2.5 | Bomb timer (45s after plant), explosion/defuse win | 1h | T5.2.1, T5.2.2 |

### T5.3 Capture the Flag
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.3.1 | Flag pickup/drop/capture mechanics | 1.5h | None |
| T5.3.2 | Flag carrier speed penalty (0.85x) | 30m | T5.3.1 |
| T5.3.3 | 10s team cooldown on dropped flag | 30m | T5.3.1 |
| T5.3.4 | Capture zone at each team's base | 30m | None |

### T5.4 Round System
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.4.1 | Configurable round time (3-10 min) | 1h | None |
| T5.4.2 | Overtime sudden death at match tie | 30m | None |
| T5.4.3 | Round start countdown (5 seconds) | 30m | None |
| T5.4.4 | Round end screen (win/loss, stats) | 1h | None |

### T5.5 Pre-Round Buy Phase
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.5.1 | Buy menu at round start (F2 shortcut) | 1.5h | None |
| T5.5.2 | Economy: money per kill (300), win (1500), loss (2000) | 1h | None |
| T5.5.3 | Weapon/gear pricing tiers (pistol=free, rifle=2000-4000) | 1h | T5.5.1, T5.5.2 |

### T5.6 Spectator Mode
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.6.1 | Free camera (WASD + mouse orbit) | 1.5h | None |
| T5.6.2 | Player follow (cycle through living players) | 1h | None |
| T5.6.3 | Overview camera (top-down tactical) | 1h | None |

### T5.7 Kill Cam
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.7.1 | 3-second replay from killer's POV | 2h | None |
| T5.7.2 | Show health/weapon/distance info overlay | 30m | T5.7.1 |

### T5.8 Round MVP
| ID | Task | Est. | Deps |
|----|------|------|------|
| T5.8.1 | Track highlight moments (multi-kill, clutch, wallbang) | 1h | None |
| T5.8.2 | Auto-detect best play per match | 1h | T5.8.1 |
| T5.8.3 | MVP display screen at match end | 1h | T5.8.2 |

## Phase 6 — Multiplayer & Networking

### T6.1 Server Infrastructure
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.1.1 | Server list UI (name, map, mode, players, ping) | 2h | None |
| T6.1.2 | Dedicated server executable (headless binary) | 2h | None |
| T6.1.3 | Server config: max players, map rotation, mode, time limit | 1h | T6.1.2 |

### T6.2 Netcode (lightyear 0.26)
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.2.1 | Client-server connection establishment + handshake | 3h | None |
| T6.2.2 | State sync: player positions, health, weapons | 4h | T6.2.1 |
| T6.2.3 | Input prediction + server reconciliation | 4h | T6.2.2 |
| T6.2.4 | Lag compensation (rewind-based hit registration) | 3h | T6.2.2 |

### T6.3 ECS Replication (bevy_replicon 0.41)
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.3.1 | Define replicated components (Player, Health, Weapon, Transform) | 2h | None |
| T6.3.2 | Server authority over damage + physics | 2h | T6.3.1 |
| T6.3.3 | Client-side interpolation for smooth visuals | 2h | T6.3.1 |

### T6.4 Player Auth & Profile
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.4.1 | Player authentication (token or Steam ID) | 2h | None |
| T6.4.2 | Profile persistence: stats, rank, cosmetics | 2h | None |
| T6.4.3 | Player name + appearance customization | 1h | T6.4.2 |

### T6.5 Party System
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.5.1 | Invite to party (in-game or overlay) | 2h | None |
| T6.5.2 | Party leader controls (start search, kick, invite) | 1h | T6.5.1 |
| T6.5.3 | Join lobby as party | 1h | T6.5.1 |

### T6.6 Matchmaking
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.6.1 | Skill rating (elo/MMR) per player | 3h | None |
| T6.6.2 | Team balancing by average MMR | 2h | T6.6.1 |
| T6.6.3 | Region-based ping matching | 1h | None |

### T6.7 Voice Chat
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.7.1 | In-game voice using Opus codec | 3h | None |
| T6.7.2 | Channel system: team (all) / squad (fireteam) | 1h | T6.7.1 |
| T6.7.3 | Push-to-talk + open mic options | 1h | T6.7.1 |

### T6.8 Anti-Cheat
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.8.1 | Server-side state validation | 2h | None |
| T6.8.2 | Client integrity checks (file hash, memory scan) | 2h | None |
| T6.8.3 | Report player system | 1h | None |

### T6.9 Client Prediction
| ID | Task | Est. | Deps |
|----|------|------|------|
| T6.9.1 | Client-side movement prediction | 3h | T6.2.2 |
| T6.9.2 | Visual interpolation of other players | 2h | T6.9.1 |
| T6.9.3 | Hit registration: client predicts, server confirms | 2h | T6.9.1 |

## Phase 7 — Production Polish

### T7.1 Main Menu
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.1.1 | Animated background (3D scene or video) | 2h | None |
| T7.1.2 | Mode select: Play (sub-menu), Training, Gunsmith, Settings | 1h | None |
| T7.1.3 | Version number + build info footer | 30m | None |

### T7.2 Player Profile
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.2.1 | Stats overview: K/D, win rate, accuracy, time played | 1h | None |
| T7.2.2 | Rank/level display with XP progress bar | 30m | None |
| T7.2.3 | Achievement showcase (earned/locked with descriptions) | 1h | None |
| T7.2.4 | Match history (last 20 matches with stats) | 1h | None |

### T7.3 Video Settings
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.3.1 | Resolution dropdown (native, 720p, 1080p, 1440p, 4K) | 1h | None |
| T7.3.2 | Refresh rate (60/120/144/240) | 30m | None |
| T7.3.3 | Display mode (windowed, borderless, fullscreen) | 30m | None |
| T7.3.4 | V-sync toggle | 30m | None |
| T7.3.5 | FPS cap (30/60/120/144/unlimited) | 30m | None |
| T7.3.6 | Quality presets (Low/Medium/High/Ultra) | 1h | None |
| T7.3.7 | Gamma/brightness slider | 30m | None |

### T7.4 Audio Settings
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.4.1 | Master volume slider (0-100%) | 30m | None |
| T7.4.2 | SFX volume slider | 30m | None |
| T7.4.3 | Music volume slider | 30m | None |
| T7.4.4 | Voice volume slider | 30m | None |
| T7.4.5 | HRTF toggle (headphone spatial audio) | 1h | None |

### T7.5 Controls Settings
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.5.1 | Rebindable keys for every action | 2h | None |
| T7.5.2 | Conflict detection (highlight overlaps) | 1h | T7.5.1 |
| T7.5.3 | Reset to defaults button | 30m | T7.5.1 |
| T7.5.4 | Mouse sensitivity slider (0.1-10.0) | 30m | None |
| T7.5.5 | Invert Y toggle | 30m | None |

### T7.6 Accessibility
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.6.1 | Colorblind mode (protanopia/deuteranopia/tritanopia) | 2h | None |
| T7.6.2 | Screen shake intensity (0-100%) | 30m | None |
| T7.6.3 | UI scale (small/medium/large) | 1h | None |
| T7.6.4 | Subtitle toggle for VO | 30m | None |
| T7.6.5 | High-contrast crosshair options | 30m | None |

### T7.7 Tutorial
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.7.1 | Movement tutorial: WASD, sprint, crouch, prone, jump | 1h | None |
| T7.7.2 | Combat tutorial: shooting, ADS, reload, weapon swap | 1h | None |
| T7.7.3 | Squad tutorial: command wheel, orders | 1h | None |
| T7.7.4 | Drone tutorial: deploy, fly, detonate, recall | 1h | None |

### T7.8 Performance
| ID | Task | Est. | Deps |
|----|------|------|------|
| T7.8.1 | LOD system for characters + weapons | 2h | None |
| T7.8.2 | Occlusion culling for level geometry | 2h | None |
| T7.8.3 | Draw call batching for static meshes | 1h | None |
| T7.8.4 | Shadow map resolution options | 30m | None |
| T7.8.5 | Texture quality options | 30m | None |

## Phase 8 — Campaign & Missions

### T8.1 Campaign Framework
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.1.1 | Mission select screen (linear or radial tree) | 1h | None |
| T8.1.2 | 10-15 interconnected missions with narrative | 10h | None |
| T8.1.3 | Difficulty selection (Recruit/Veteran/Elite) | 30m | None |

### T8.2 Mission Briefings
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.2.1 | Pre-mission briefing screen (text + map) | 1h | None |
| T8.2.2 | Primary + secondary objectives display | 30m | None |
| T8.2.3 | Loadout selection: primary, sidearm, gear, drone | 1h | None |

### T8.3 Narrative
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.3.1 | Radio chatter system (teammate callouts, command orders) | 2h | None |
| T8.3.2 | In-engine cutscenes | 3h | None |
| T8.3.3 | Enemy proximity taunts and callouts | 1h | None |

### T8.4 Checkpoint Save
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.4.1 | Auto-save at objective completions | 1h | None |
| T8.4.2 | Save includes: position, health, ammo, obj state, AI states | 1h | None |
| T8.4.3 | Load from checkpoint on death | 30m | T8.4.1 |

### T8.5 Bonus Objectives
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.5.1 | Intel collectibles (hidden documents/laptops) | 1h | None |
| T8.5.2 | Speed run timer + bonus | 1h | None |
| T8.5.3 | Pacifist bonus (min kills, stealth) tracking | 1h | None |

### T8.6 Mission Scoring
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.6.1 | Accuracy percentage | 30m | None |
| T8.6.2 | Stealth bonus (no alarms) | 30m | None |
| T8.6.3 | Objectives completed (primary + bonus) | 30m | None |
| T8.6.4 | Time bonus (faster = higher score) | 30m | None |
| T8.6.5 | S/A/B/C rank per mission | 30m | T8.6.1-T8.6.4 |

### T8.7 Co-op Campaign
| ID | Task | Est. | Deps |
|----|------|------|------|
| T8.7.1 | 2-4 player co-op (online) | 4h | Phase 6 |
| T8.7.2 | Shared progression (host's save) | 1h | T8.7.1 |
| T8.7.3 | Player drop-in/drop-out | 2h | T8.7.1 |
| T8.7.4 | Revive system: down -> bleed -> respawn at checkpoint | 1h | T8.7.1 |

## Phase 9 — Modding & Workshop

### T9.1 Custom Maps
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.1.1 | GLTF scene loading from external maps/ directory | 2h | None |
| T9.1.2 | Collision mesh auto-generation from GLTF | 1h | T9.1.1 |
| T9.1.3 | Spawn point configuration via RON | 1h | T9.1.1 |

### T9.2 Custom Weapons
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.2.1 | RON config: chassis stats, caliber, attachment points | 2h | None |
| T9.2.2 | Weapon model GLTF references | 1h | T9.2.1 |
| T9.2.3 | Load and register custom weapons at startup | 1h | T9.2.1 |

### T9.3 Custom Missions
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.3.1 | JSON mission descriptor format spec | 1h | None |
| T9.3.2 | Objective scripting (limited DSL) | 3h | None |
| T9.3.3 | Enemy placement + spawn triggers | 1h | T9.3.1 |

### T9.4 Mod Management
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.4.1 | Scan maps/weapons/missions directories on startup | 1h | None |
| T9.4.2 | Enumerate installed mods in main menu UI | 1h | T9.4.1 |
| T9.4.3 | Conflict detection (duplicate IDs) | 30m | T9.4.1 |
| T9.4.4 | Enable/disable per mod | 30m | T9.4.2 |

### T9.5 Workshop
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.5.1 | Steam Workshop API integration | 4h | None |
| T9.5.2 | Upload tool (in-game or CLI) | 2h | None |
| T9.5.3 | Auto-download subscribed mods | 2h | T9.5.1 |

### T9.6 SDK
| ID | Task | Est. | Deps |
|----|------|------|------|
| T9.6.1 | API reference for RON configs | 2h | None |
| T9.6.2 | GLTF export guide (Blender -> game pipeline) | 1h | None |
| T9.6.3 | Example mods with source code | 2h | None |
| T9.6.4 | Video tutorial series | 4h | None |

## Phase 10 — Post-Launch Live Service

### T10.1 Content Drops
| ID | Task | Est. | Deps |
|----|------|------|------|
| T10.1.1 | New weapon packs (seasonal, 4-6 weapons each) | 8h per pack | Phase 2 |
| T10.1.2 | New maps for each game mode (2-3 per season) | 10h per map | Phase 5 |
| T10.1.3 | New gear + attachments | 4h per set | Phase 2 |

### T10.2 Ranked Seasons
| ID | Task | Est. | Deps |
|----|------|------|------|
| T10.2.1 | 3-month competitive season cycles | 2h | T6.6.1 |
| T10.2.2 | Season rewards: weapon skins, player titles, badges | 4h | None |
| T10.2.3 | Leaderboards: global, regional, friends | 3h | None |

### T10.3 Battle Pass
| ID | Task | Est. | Deps |
|----|------|------|------|
| T10.3.1 | Free + premium track system | 3h | None |
| T10.3.2 | 100 tiers per season | 2h | T10.3.1 |
| T10.3.3 | Reward system (skins, outfits, drone skins, XP boosts) | 4h | T10.3.1 |

### T10.4 Community
| ID | Task | Est. | Deps |
|----|------|------|------|
| T10.4.1 | Community map contests | 2h | Phase 9 |
| T10.4.2 | Player feedback surveys in-game | 1h | None |
| T10.4.3 | Dev blog / patch notes system | 1h | None |

### T10.5 Maintenance
| ID | Task | Recurring |
|----|------|-----------|
| T10.5.1 | Anti-cheat signature updates | Monthly |
| T10.5.2 | Performance optimization | Per patch |
| T10.5.3 | Bug fix releases | As needed |
| T10.5.4 | Server binary updates | Per patch |

## Acceptance Criteria (Phase 3 — Asset Ready)

- [ ] All 91 source files compile with 0 errors (verified via cargo check)
- [ ] cargo clippy passes with tolerable dead_code warnings
- [ ] Audio plays: footsteps (15 types), weapon fire (4 guns), ambient (4 loops), UI (8 sounds)
- [ ] Visual effects: muzzle flash, hit impacts, death particles, tracers
- [ ] Weapon models visible in both 1st and 3rd person
- [ ] Player can move, shoot, kill enemies, die, respawn (full loop)
- [ ] HUD shows: health, stamina, ammo, stance, crosshair, weapon name, objectives, squad status
- [ ] Drones deploy, fly, detonate/recall correctly
- [ ] Squad commands: Tab wheel opens, orders dispatch, teammates respond
- [ ] Save/Load: F5 persists state, F9 restores it
- [ ] Game is playable for 10+ minutes without crash
