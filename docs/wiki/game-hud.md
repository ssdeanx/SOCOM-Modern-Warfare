# Game Module: `hud/` — Heads-Up Display

**Path:** `crates/game/src/hud/`  
**Files:** 8 — `mod.rs`, `elements.rs`, `systems.rs`, `xp_notification.rs`, `stamina_bar.rs`, `achievement_popup.rs`, `kill_feed.rs`, `squad_status.rs`  
**Purpose:** In-game UI — health, ammo, stamina, XP, achievements, kill feed, squad status

## Module Map

```
hud/
├── mod.rs             — HudPlugin
├── elements.rs        — HUD element spawning + cleanup
├── systems.rs         — Health bar, stance, ammo, weapon name, crosshair
├── stamina_bar.rs     — Stamina bar UI + update
├── xp_notification.rs — XP gain + level up popups
├── achievement_popup.rs — Achievement unlock popup
├── kill_feed.rs       — Kill feed entries
└── squad_status.rs    — Squad member status + objective text
```

## HudPlugin

Registers systems in 3 groups:
1. **OnEnter(InGame):** `spawn_hud`, `spawn_stamina_bar`, `spawn_squad_status`, `spawn_objective_text`
2. **OnExit(InGame):** `cleanup_hud`
3. **Update (InGame):** Health bar, stance text, ammo text, weapon name, crosshair, stamina bar
4. **Update (InGame):** XP notifications, level-up notifications, popup lifetimes
5. **Update (InGame):** Kill feed, squad status, objective text

## HUD Elements

### Health Bar
- Green-to-red gradient based on health ratio
- Numerical HP display

### Ammo Display
- Current magazine count / total reserve
- Reload indicator

### Stance Indicator
- Text showing current stance: STANDING, SPRINTING, CROUCHING, PRONE

### Weapon Name
- Name of currently equipped weapon

### Crosshair
- Visibility toggled based on context (ADS, etc.)

### Stamina Bar
- Yellow bar that depletes during sprint
- Flashes when exhausted

### XP Notifications
- "+50 XP" floating text on kills/objectives
- Fade-out animation

### Level Up Popup
- "LEVEL UP!" announcement
- Shows new level number

### Achievement Popups
- Achievement unlock notification on discovery

### Kill Feed
- Kill entries with weapon name and victim
- Auto-cleanup after timeout

### Squad Status
- Teammate health bars
- Alive/down indicator
- Objective text display
