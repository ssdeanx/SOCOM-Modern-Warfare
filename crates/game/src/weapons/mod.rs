pub mod barrel;
pub mod caliber;
pub mod chassis;
pub mod magazine;
pub mod sight;
pub mod stock;
pub mod underbarrel;

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use socom_core::components::Weapon;

use self::barrel::BarrelType;
use self::caliber::Caliber;
use self::chassis::WeaponChassis;
use self::magazine::MagazineType;
use self::sight::SightType;
use self::stock::StockType;
use self::underbarrel::UnderbarrelType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteWeapon {
    pub chassis: WeaponChassis,
    pub caliber: Caliber,
    pub barrel: BarrelType,
    pub sight: SightType,
    pub underbarrel: UnderbarrelType,
    pub magazine: MagazineType,
    pub stock: StockType,
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

impl CompleteWeapon {
    pub fn assemble(
        chassis: WeaponChassis,
        barrel: BarrelType,
        sight: SightType,
        underbarrel: UnderbarrelType,
        magazine: MagazineType,
        stock: StockType,
    ) -> Self {
        let cal = chassis.caliber;
        let final_damage = chassis.base_damage * cal.damage_mult() * barrel.damage_mult();
        let final_fire_rate = chassis.base_fire_rate;
        let base_mag = chassis.base_magazine_size as f32 * magazine.capacity_mult();
        let final_magazine_size = base_mag.round().max(1.0) as u32;
        let final_reserve_ammo = chassis.base_reserve_ammo;
        let final_reload_time = chassis.base_reload_time * magazine.reload_mult();
        let final_spread_hip = chassis.base_spread_hip
            * sight.hip_spread_mult()
            * underbarrel.hip_spread_mult()
            * stock.hip_spread_mult();
        let final_spread_ads =
            chassis.base_spread_ads * cal.recoil_mult() * sight.ads_spread_mult();
        let final_recoil_vertical = chassis.base_recoil_vertical
            * cal.recoil_mult()
            * barrel.recoil_mult()
            * underbarrel.vertical_recoil_mult()
            * stock.recoil_mult();
        let final_recoil_horizontal = chassis.base_recoil_horizontal
            * cal.recoil_mult()
            * barrel.recoil_mult()
            * underbarrel.horizontal_recoil_mult()
            * stock.recoil_mult();
        let final_weight = (chassis.base_weight
            + barrel.weight_add()
            + sight.weight_add()
            + underbarrel.weight_add()
            + magazine.weight_add()
            + stock.weight_add())
        .max(0.5);
        let final_ads_speed = chassis.base_ads_speed
            * sight.ads_time_mult()
            * underbarrel.ads_speed_mult()
            * stock.ads_speed_mult();
        let final_sway = chassis.base_sway * stock.sway_mult();
        let final_max_range = chassis.max_range * cal.range_mult() * barrel.range_mult();
        let final_is_automatic = chassis.is_automatic;
        let pellets_per_shot = chassis.pellets_per_shot;
        CompleteWeapon {
            chassis,
            caliber: cal,
            barrel,
            sight,
            underbarrel,
            magazine,
            stock,
            final_damage,
            final_fire_rate,
            final_magazine_size,
            final_reserve_ammo,
            final_reload_time,
            final_spread_hip,
            final_spread_ads,
            final_recoil_vertical,
            final_recoil_horizontal,
            final_weight,
            final_ads_speed,
            final_sway,
            final_max_range,
            final_is_automatic,
            pellets_per_shot,
        }
    }

    pub fn default_m4a1() -> Self {
        Self::assemble(
            WeaponChassis::m4a1(),
            BarrelType::Standard,
            SightType::Iron,
            UnderbarrelType::None,
            MagazineType::Standard,
            StockType::Standard,
        )
    }
    pub fn default_mp5sd() -> Self {
        Self::assemble(
            WeaponChassis::mp5sd(),
            BarrelType::Standard,
            SightType::Iron,
            UnderbarrelType::None,
            MagazineType::Standard,
            StockType::Standard,
        )
    }
    pub fn default_m1911() -> Self {
        Self::assemble(
            WeaponChassis::m1911(),
            BarrelType::Standard,
            SightType::Iron,
            UnderbarrelType::None,
            MagazineType::Standard,
            StockType::Standard,
        )
    }
    pub fn default_ak47() -> Self {
        Self::assemble(
            WeaponChassis::ak47(),
            BarrelType::Standard,
            SightType::Iron,
            UnderbarrelType::None,
            MagazineType::Standard,
            StockType::Standard,
        )
    }

    /// Factory: default M24 SWS with SniperScope and bipod.
    pub fn default_m24() -> Self {
        Self::assemble(
            WeaponChassis::m24(),
            BarrelType::Standard,
            SightType::SniperScope,
            UnderbarrelType::Bipod,
            MagazineType::Standard, // Internal mag, unchanged
            StockType::Precision,
        )
    }

    /// Factory: default L96A1 with SniperScope and bipod.
    pub fn default_l96a1() -> Self {
        Self::assemble(
            WeaponChassis::l96a1(),
            BarrelType::Standard,
            SightType::SniperScope,
            UnderbarrelType::Bipod,
            MagazineType::Standard,
            StockType::Precision,
        )
    }
}

#[derive(Component, Debug, Clone)]
pub struct EquippedWeapon {
    pub weapon: CompleteWeapon,
}

impl EquippedWeapon {
    /// Convert to the legacy `Weapon` interface for systems that still consume it.
    pub fn to_weapon(&self) -> Weapon {
        Weapon {
            name: self.weapon.chassis.name.clone(),
            fire_rate: self.weapon.final_fire_rate,
            damage: self.weapon.final_damage,
            magazine_size: self.weapon.final_magazine_size,
            reserve_ammo: self.weapon.final_reserve_ammo,
            reload_time: self.weapon.final_reload_time,
            is_automatic: self.weapon.final_is_automatic,
            spread_degrees: self.weapon.final_spread_hip,
            max_range: self.weapon.final_max_range,
            ads_fov_target: 55.0,
        }
    }
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        Self {
            weapon: CompleteWeapon::default_m4a1(),
        }
    }
}
