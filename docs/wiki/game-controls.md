# Game Module: `controls/` — Controls & Stance

**Path:** `crates/game/src/controls/`  
**Files:** 3 — `mod.rs`, `stance.rs`, `turn_rate.rs`  
**Purpose:** Stance transition timers, turn rate sensitivity modulation

## ControlsPlugin

Registers `stance_transition_system` in Update.

## Stance System (`stance.rs`)

Handles stance transitions with timing constraints to prevent rapid cycling.

### Input Mapping
- **C** → Crouch toggle
- **Z** → Prone toggle
- **Shift** (while moving) → Sprint
- Sprint exits on Shift release or stamina depletion

## Turn Rate (`turn_rate.rs`)

```rust
pub fn turn_rate_mult(stance: &MovementState, weight_mult: f32, stamina: &Stamina) -> f32
```

Returns a multiplier applied to the camera's `SensitivityMultiplier` resource each frame. Accounts for:
- **Stance** — Prone restricts turning, standing allows full rotation
- **Weapon Weight** — Heavy weapons slow turn rate
- **Stamina** — Low stamina penalizes turn speed

This gives a realistic feel where:
- Sprinting with a heavy LMG = very slow turning
- Crouching with a pistol = relatively fast turning
- Prone with any weapon = restricted turning
- Exhausted = additional turn speed penalty
