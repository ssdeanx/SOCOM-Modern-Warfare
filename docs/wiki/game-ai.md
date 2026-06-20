# Game Module: `ai/` — Enemy & Teammate AI

**Path:** `crates/game/src/ai/`  
**Files:** 3 — `mod.rs`, `enemy.rs`, `teammate.rs`  
**Purpose:** Enemy AI finite state machine, teammate follow AI

## Module Map

```
ai/
├── mod.rs       — AiState, PatrolRoute, VisionCone, AiPlugin, AiSystems set
├── enemy.rs     — EnemyBundle, patrol_system, detection_system, engage_system, enemy_death_system
└── teammate.rs  — TeammateBundle + TeammatePlugin
```

## AiState (FSM)

```rust
pub enum AiState { Patrol, Alert, Engage }
```

- **Patrol** → Following waypoints, no contact. Suspicion decays.
- **Alert** → Investigating last-known position or sound (transitional).
- **Engage** → Actively engaging hostile target. Faces player, fires at intervals.

## Key Components

### PatrolRoute
```rust
pub struct PatrolRoute {
    pub waypoints: Vec<Vec3>,
    pub current_index: usize,
    pub wait_timer: Timer,
    pub is_waiting: bool,
}
```

### VisionCone
```rust
pub struct VisionCone {
    pub fov_h: f32,       // 120° horizontal
    pub fov_v: f32,       // 60° vertical
    pub range: f32,       // 40m detection range
    pub suspicion: f32,   // 0–100 meter
}
```

## AiSystems Set
A `#[derive(SystemSet)]` label applied to all AI systems. Used by `PhysicsPlugin` to ensure movement runs **after** AI tick.

## EnemyBundle
Spawns an enemy with:
- `Team::Enemy`, `Health` (80 HP), `MovementState::Standing`
- `Weapon::ak47()` with full `WeaponState`
- Vision cone, patrol route, kinematic rigidbody + capsule collider

## Systems

### patrol_system
- Moves enemy along waypoints at 1.5 m/s in Patrol state
- Waits at each waypoint for 2 seconds
- Applies gravity, smooth acceleration toward target

### detection_system
- For enemies in Patrol or Alert state:
  1. Range check (40m max)
  2. Horizontal cone-of-vision check (120° FOV)
  3. Line-of-sight raycast (checks for obstruction)
  4. If visible: build suspicion at 30/s
  5. At 100 suspicion: transition to Engage
  6. If not visible: decay suspicion at 10/s

### engage_system
- For enemies in Engage state:
  1. Face the player, decelerate to stop
  2. LOS check — if blocked, go to Alert
  3. Fire at player every 1.5s with 10 damage per hit
  4. Auto-reload when magazine empty (2.5s reload)

### enemy_death_system
- Despawns enemy entities when health reaches zero
