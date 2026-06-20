# Game Module: `weapon_handling/` — Weapon Handling

**Path:** `crates/game/src/weapon_handling/`  
**Files:** 1 — `mod.rs`  
**Purpose:** Weapon weight classes that affect movement speed, ADS time, and sway

## WeaponWeight

```rust
pub enum WeaponWeight { Light, Medium, Heavy, Sniper }
```

### Per-Class Modifiers

| Weight Class | Speed Mult | ADS Time | Sway Amp | Example |
|-------------|-----------|----------|----------|---------|
| Light | 1.0× | 0.12s | 0.001 | M1911 |
| Medium | 0.75× | 0.25s | 0.003 | MP5SD |
| Heavy | 0.55× | 0.40s | 0.008 | M4A1, AK-47 |
| Sniper | 0.45× | 0.50s | 0.012 | M24, L96A1 |

### Detection
```rust
pub fn from_weapon_name(name: &str) -> WeaponWeight
```

## WeaponHandling Component

```rust
pub struct WeaponHandling {
    pub current_ads_time: f32,
    pub current_weight_mult: f32,
    pub deploy_timer: Timer,
    pub is_deploying: bool,
}
```

## Weapon Handling System

Registered as a global system in `main.rs`. Updates handling stats based on the player's active weapon each frame:

1. Read `WeaponSlot` for active weapon name
2. Look up `WeaponWeight` from name
3. Apply `speed_mult` to `current_weight_mult`
4. Apply `ads_time` to `current_ads_time`

These values are consumed by:
- `player_movement_system` — speed modifier
- `turn_rate_update_system` — turn speed modifier
- Future weapon sway animation
