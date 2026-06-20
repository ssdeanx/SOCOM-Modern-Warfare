# Game Module: `feedback/` — Visual Feedback

**Path:** `crates/game/src/feedback/`  
**Files:** 4 — `mod.rs`, `hit_marker.rs`, `vignette.rs`, `enemy_fx.rs`  
**Purpose:** Combat feedback effects — hit markers, damage vignette, enemy hurt/death FX

## Module Map

```
feedback/
├── mod.rs         — FeedbackPlugin
├── hit_marker.rs  — Hit marker detection + lifetime
├── vignette.rs    — Damage vignette overlay
└── enemy_fx.rs    — Enemy hurt flash + death particle effects
```

## FeedbackPlugin

Registers 7 systems in Update (gated by `is_not_paused`):
1. `hit_marker_detect_system` — Show hit marker on confirmed hits
2. `hit_marker_lifetime_system` — Fade/remove hit markers
3. `damage_vignette_system` — Red vignette on player damage
4. `enemy_hurt_flash_system` — Enemy entity flash white on hit
5. `hurt_flash_lifetime_system` — Remove hurt flash after duration
6. `enemy_death_effect_system` — Death explosion particles
7. `death_particle_lifetime_system` — Clean up death particles

## Hit Marker System
- Shows crosshair indicator when bullets connect with target
- Visual confirmation feedback for hitscan weapons

## Damage Vignette
- Red screen overlay intensity proportional to damage taken
- Fades out over time
- Indicates hit direction (future feature)

## Enemy FX
### Hurt Flash
- Enemy entity flashes white briefly when hit
- Duration-based (single frame pulse)

### Death Effect
- Explosion of particles on enemy death (via bevy_hanabi)
- Configurable particle count and color
- Entities cleaned up after lifetime expires
