# Game Module: `breathing/` — Hold-Breath System

**Path:** `crates/game/src/breathing/`  
**Files:** 1 — `mod.rs`  
**Purpose:** Hold-breath mechanic for weapon steadiness while aiming

## Breathing Component

```rust
pub struct Breathing {
    pub holding: bool,       // Currently holding breath
    pub hold_timer: f32,     // Seconds held so far
    pub cooldown_timer: Timer, // 2s cooldown between holds
    pub steadiness: f32,     // 1.0 = steady, 0.5 = swaying
}
```

## Breathing System

### Activation
- Hold both **Aim (Right Mouse)** + **Sprint (Shift)** simultaneously
- Requires stamina > 5.0

### While Holding
- Stamina drains at 8/s
- Steadiness degrades: `1.0 - (hold_time × 2.0).min(0.5)`
- Caps at 50% steadiness after 0.25s

### Release
- 2s cooldown before re-holding is possible
- Steadiness instantly resets to 1.0

## BreathingPlugin
Simple plugin registering `breathing_system` in Update.
