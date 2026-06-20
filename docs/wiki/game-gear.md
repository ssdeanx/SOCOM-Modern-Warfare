# Game Module: `gear/` — Equipment & Inventory

**Path:** `crates/game/src/gear/`  
**Files:** 10 — `mod.rs`, `items.rs`, `inventory.rs`, `attachments.rs`, `workshop.rs`, `throwable.rs`, `deployable.rs`, `melee.rs`, `equipment_types.rs`, `equipment_inventory.rs`, `healing.rs`  
**Purpose:** Player inventory, weapon workshop, throwable/deployable equipment, melee, healing

## Module Map

```
gear/
├── mod.rs                 — GearPlugin + apply_workshop_to_weapon_system
├── items.rs               — GearItem + GearSlot definitions
├── inventory.rs           — PlayerInventory resource
├── attachments.rs         — Weapon attachment items
├── workshop.rs            — WeaponWorkshop resource + modification UI
├── throwable.rs           — Grenade throw logic + fuse timer
├── deployable.rs          — Deployable placement + arming + detonation
├── melee.rs               — Melee attack system (knife)
├── equipment_types.rs     — EquipmentType enum + GrenadeProjectile + Deployable component
├── equipment_inventory.rs — EquipmentInventory resource + selection
└── healing.rs             — Self-heal + bleed-out + revive systems
```

## GearPlugin

Registers: `PlayerInventory`, `WeaponWorkshop`, `EquipmentInventory` resources + 12 systems in Update.

### Systems
1. `track_damage_for_loot` — Monitors damage for loot drops
2. `weapon_modification_system` — Workshop attachment application
3. `select_equipment_system` — Cycle/select equipment slot
4. `throw_equipment_system` — Throw grenades
5. `fuse_timer_system` — Grenade fuse countdown
6. `c4_detonation_system` — Remote C4 detonation
7. `deploy_equipment_system` — Place deployables (Claymore, etc.)
8. `deployable_arm_system` — Arm deployables after placement delay
9. `claymore_detonation_system` — Proximity-based claymore trigger
10. `melee_attack_system` — Knife melee with damage + sound
11. `self_heal_system` — Use medical items to heal
12. `bleed_out_system` — Tick bleed-out timer
13. `revive_system` — Revive downed teammates
14. `apply_workshop_to_weapon_system` — Apply attachment modifiers to active weapon

## EquipmentType (14 variants)

| Type | Category | Damage | Blast Radius | Fuse Time |
|------|----------|--------|-------------|-----------|
| FragGrenade | Throwable | 200 | 6.0m | 3.5s |
| Flashbang | Throwable | 0 | — | — |
| SmokeGrenade | Throwable | 0 | — | — |
| TearGas | Throwable | 5 | — | — |
| C4 | Deployable | 400 | 10.0m | Remote |
| Claymore | Deployable | 300 | 8.0m | Proximity |
| BreachingCharge | Deployable | 500 | 4.0m | — |
| MedicalKit | Healing | — | — | — |
| FieldBandage | Healing | — | — | — |
| Splint | Healing | — | — | — |
| EnergyDrink | Consumable | — | — | — |
| Knife | Melee | 100 | — | — |
| Binoculars | Tool | — | — | — |
| Flare | Tool | — | — | — |

## Key Components

### GrenadeProjectile
```rust
pub struct GrenadeProjectile {
    pub fuse_timer: Timer,
    pub equip_type: EquipmentType,
    pub damage: f32,
    pub radius: f32,
    pub source: Entity,
}
```

### Deployable
```rust
pub struct Deployable {
    pub equip_type: EquipmentType,
    pub damage: f32,
    pub radius: f32,
    pub trigger_radius: f32,
    pub source: Entity,
    pub armed: bool,
    pub arm_timer: Option<Timer>,
}
```

### C4Charge
```rust
pub struct C4Charge {
    pub source: Entity,
    pub damage: f32,
    pub radius: f32,
}
```

## PlayerInventory Resource
```rust
pub struct PlayerInventory {
    pub equipped: [Option<GearItem>; 5],  // Primary, Sidearm, Helmet, BodyArmor, Tactical
    pub stash: Vec<GearItem>,
    pub credits: u64,
}
```

Slots: 0 = PrimaryWeapon, 1 = SidearmWeapon, 2 = Helmet, 3 = BodyArmor, 4 = TacticalGear
