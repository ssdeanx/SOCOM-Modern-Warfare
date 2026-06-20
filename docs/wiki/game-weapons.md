# Game Module: `weapons/` — Weapon Assembly

**Path:** `crates/game/src/weapons/`  
**Files:** 8 — `mod.rs`, `chassis.rs`, `caliber.rs`, `barrel.rs`, `sight.rs`, `underbarrel.rs`, `magazine.rs`, `stock.rs`  
**Purpose:** Modular weapon assembly system with attachment modifiers

## Module Map

```
weapons/
├── mod.rs          — CompleteWeapon struct + EquippedWeapon component
├── chassis.rs      — WeaponChassis (platform base stats) + WeaponClass
├── caliber.rs      — Caliber enum (damage, penetration, recoil mult)
├── barrel.rs       — BarrelType variants with stat modifiers
├── sight.rs        — SightType variants with stat modifiers
├── underbarrel.rs  — UnderbarrelType variants with stat modifiers
├── magazine.rs     — MagazineType variants with stat modifiers
└── stock.rs        — StockType variants with stat modifiers
```

## CompleteWeapon

The central type assembled from 6 components. Each component contributes modifiers to produce final weapon stats.

```rust
pub struct CompleteWeapon {
    pub chassis: WeaponChassis,
    pub caliber: Caliber,
    pub barrel: BarrelType,
    pub sight: SightType,
    pub underbarrel: UnderbarrelType,
    pub magazine: MagazineType,
    pub stock: StockType,
    // Final computed stats:
    pub final_damage: f32,
    pub final_fire_rate: f32,
    pub final_magazine_size: u32,
    pub final_reserve_ammo: u32,
    pub final_reload_time: f32,
    pub final_spread_hip: f32,
    pub final_spread_ads: f32,
    pub final_recoil_vertical: f32,
    pub final_recoil_horizontal: f32,
    pub final_weight: f32,
    pub final_ads_speed: f32,
    pub final_sway: f32,
    pub final_max_range: f32,
    pub final_is_automatic: bool,
    pub pellets_per_shot: u32,
}
```

### Assembly Formula
- **Damage** = chassis.base_damage × caliber.damage_mult × barrel.damage_mult
- **Fire Rate** = chassis.base_fire_rate (unchanged)
- **Magazine** = chassis.base_magazine_size × magazine.capacity_mult
- **Reload** = chassis.base_reload_time × magazine.reload_mult
- **Hip Spread** = chassis.base_spread_hip × sight.hip_spread_mult × underbarrel.hip_spread_mult × stock.hip_spread_mult
- **ADS Spread** = chassis.base_spread_ads × caliber.recoil_mult × sight.ads_spread_mult
- **Vertical Recoil** = chassis × caliber × barrel × underbarrel × stock
- **Weight** = chassis.base_weight + barrel + sight + underbarrel + magazine + stock
- **ADS Speed** = chassis.base_ads_speed × sight × underbarrel × stock
- **Max Range** = chassis.max_range × caliber.range_mult × barrel.range_mult
- **Sway** = chassis.base_sway × stock.sway_mult

### Factory Methods
- `default_m4a1()`, `default_mp5sd()`, `default_m1911()`, `default_ak47()`
- `default_m24()` — Sniper with SniperScope + Bipod
- `default_l96a1()` — Heavy sniper with SniperScope + Bipod

### EquippedWeapon Component
```rust
pub struct EquippedWeapon { pub weapon: CompleteWeapon }
```

## WeaponChassis

Defines base stats for a weapon platform:

```rust
pub struct WeaponChassis {
    pub name: String,
    pub class: WeaponClass,    // Pistol, SMG, AssaultRifle, BattleRifle, SniperRifle, Shotgun
    pub caliber: Caliber,
    pub base_damage: f32,
    pub base_fire_rate: f32,
    pub base_magazine_size: u32,
    pub base_reserve_ammo: u32,
    pub base_reload_time: f32,
    pub base_spread_hip: f32,
    pub base_spread_ads: f32,
    pub base_recoil_vertical: f32,
    pub base_recoil_horizontal: f32,
    pub base_weight: f32,
    pub base_ads_speed: f32,
    pub base_sway: f32,
    pub max_range: f32,
    pub is_automatic: bool,
    pub pellets_per_shot: u32,
}
```

Available chassis: M4A1, MP5SD, M1911, AK-47, M24, L96A1

## Caliber

```rust
pub enum Caliber { NineMm, FortyFiveACP, FiveFiveSixNato, SevenSixTwoX39, SevenSixTwoNato, TwelveGauge, FiftyBMG }
```

Each caliber provides: `damage_mult()`, `penetration_mult()`, `velocity_mult()`, `recoil_mult()`, `range_mult()`

## Attachments

Each attachment type is an enum with variants providing stat modifiers:

| Type | Variants |
|------|----------|
| `BarrelType` | Standard, Extended, Suppressed, Heavy, Short |
| `SightType` | Iron, RedDot, Holo, ACOG, SniperScope, CantedIron, CantedRedDot |
| `UnderbarrelType` | None, VerticalGrip, AngledGrip, Bipod, Flashlight, Laser |
| `MagazineType` | Standard, Extended, Fast, Drum |
| `StockType` | Standard, Light, Heavy, Precision, Folding, NoStock |
