# Game Module: `physics/` — Movement & Physics

**Path:** `crates/game/src/physics/`  
**Files:** 5 — `mod.rs`, `player_movement.rs`, `enemy_movement.rs`, `stance.rs`, `layers.rs`  
**Purpose:** Player/enemy movement, stance transitions, collision layers

## Module Map

```
physics/
├── mod.rs              — PhysicsPlugin
├── player_movement.rs  — player_movement_system
├── enemy_movement.rs   — enemy_movement_system
├── stance.rs           — player_stance_system
└── layers.rs           — GameLayer enum + CharacterController component
```

## PhysicsPlugin

Registers:
- `player_movement_system` (after `AiSystems`, gated by `is_not_paused`)
- `player_stance_system` (gated by `is_not_paused`)
- `enemy_movement_system` (after `AiSystems`, gated by `is_not_paused`)

## CharacterController Component

```rust
pub struct CharacterController {
    pub velocity: Vec3,
    pub on_ground: bool,
    pub fall_start_y: f32,
}
```

## GameLayer
```rust
pub enum GameLayer { Default, World, Player, Enemy, Bullet, Camera }
```

## Player Movement System

### Movement Speeds
| Stance | Speed |
|--------|-------|
| Stand | 3.0 m/s |
| Sprint | 5.0 m/s |
| Crouch | 1.5 m/s |
| Prone / Cover | 0.8 m/s |

### System Flow
1. **Ground Detection** — Short raycast downward (0.15m)
2. **Fall Damage** — Calculated from peak fall height, 5 damage/m beyond 3m safe fall
3. **Jump** — 6.0 m/s upward velocity on Jump key
4. **Movement Input** — Camera-relative WASD → target velocity with 12 m/s² acceleration
5. **Speed Modifiers** — Stance × ADS × Stamina × WeaponWeight
6. **Gravity** — -19.6 m/s², capped at -30 m/s²
7. **MoveAndSlide** — avian3d character controller with collision response

## Enemy Movement System
Applies `CharacterController` velocity from AI systems to enemy transforms each frame.

## Stance System
- C → Crouch toggle
- Z → Prone toggle
- Shift (while moving) → Sprint
- Sprint toggles off when Shift released or stamina depleted
- Stance transition timers prevent rapid cycling
