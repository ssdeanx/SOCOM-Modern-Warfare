# Game Module: `stamina/` — Stamina System

**Path:** `crates/game/src/stamina/`  
**Files:** 1 — `mod.rs`  
**Purpose:** Realistic stamina affecting weapon sway, accuracy, movement speed, and breathing

## Stamina Component

```rust
pub struct Stamina {
    pub current: f32,    // 0–100
    pub max: f32,        // 100
    pub regen_timer: Timer,  // 1.5s delay before regen starts
    pub exhausted: bool,
}
```

## Constants
| Parameter | Value |
|-----------|-------|
| MAX_STAMINA | 100.0 |
| SPRINT_DRAIN_RATE | 25.0/s |
| REGEN_RATE | 15.0/s |
| REGEN_DELAY | 1.5s |
| EXHAUSTED_SPEED_MULT | 0.6 |

## Stamina System (`stamina_system`)

Registered as a global system in `main.rs`. Runs every frame during InGame.

1. **Sprinting:** Drain stamina at 25/s → exhausted at 0
2. **Not Sprinting:** After 1.5s delay, regen at 15/s
3. **Exhaustion:** Clears when stamina recovers above 10

## Helper Functions

### `stamina_speed_mult(stamina) → f32`
- Returns 0.6 when exhausted, 1.0 otherwise
- Applied to player movement speed

### `stamina_sway_mult(stamina) → f32`
| Stamina Ratio | Sway Multiplier |
|--------------|-----------------|
| > 60% | 1.0x (normal) |
| 30–60% | 1.3x |
| < 30% | 1.8x |
| Exhausted | 2.5x |

### `stamina_spread_mult(stamina) → f32`
| Condition | Spread Multiplier |
|-----------|------------------|
| Normal | 1.0x |
| < 30% | 1.5x |
| Exhausted | 2.0x |
