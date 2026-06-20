# Game Module: `squad/` — Squad Command System

**Path:** `crates/game/src/squad/`  
**Files:** 3 — `mod.rs`, `orders.rs`, `formation.rs`  
**Purpose:** Teammate squad orders and formation movement

## Module Map

```
squad/
├── mod.rs       — SquadPlugin
├── orders.rs    — SquadOrder enum, SquadOrderMessage, ActiveOrders resource
└── formation.rs — squad_formation_system
```

## SquadOrder

```rust
pub enum SquadOrder {
    MoveToTarget(Entity),
    EngageTarget(Entity),
    SuppressPosition(Entity),
    RegroupOnPlayer,
    HoldPosition,
}
```

## SquadOrderMessage
```rust
pub struct SquadOrderMessage { pub order: SquadOrder, pub source: Entity }
```

## ActiveOrders Resource
```rust
pub struct ActiveOrders {
    pub orders: HashMap<Entity, SquadOrder>,
}
```

## Systems

### squad_order_dispatch_system
Reads `SquadOrderMessage`s and stores them in `ActiveOrders`, mapped to each teammate entity.

### squad_formation_system
Moves teammates to formation positions relative to the player:
- **Position 0** — Left rear (1.5m left, 1m back)
- **Position 1** — Right rear (1.5m right, 1m back)
- **Position 2** — Center rear (2.5m back)

Uses smooth acceleration at 3.5 m/s with 8.0 m/s² acceleration.
