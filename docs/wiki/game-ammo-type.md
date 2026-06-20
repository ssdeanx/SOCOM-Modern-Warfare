# Game Module: `ammo_type/` — Ammo Types

**Path:** `crates/game/src/ammo_type/`  
**Files:** 1 — `mod.rs`  
**Purpose:** Ammunition type definitions with damage, penetration, and spread modifiers

## AmmoType

```rust
pub enum AmmoType { Fmj, HollowPoint, ArmourPiercing, Tracer }
```

| Type | Damage Mult | Penetration Mult | Spread Mult |
|------|-------------|-----------------|-------------|
| FMJ | 1.0× | 1.0× | 1.0× |
| Hollow Point | 1.25× | 0.5× | 1.15× |
| Armour Piercing | 0.85× | 1.8× | 1.05× |
| Tracer | 0.95× | 1.0× | 1.0× |

## LoadedAmmo Component
```rust
pub struct LoadedAmmo { pub ammo_type: AmmoType, pub count: u32 }
```
