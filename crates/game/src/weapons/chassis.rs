use super::caliber::Caliber;
/// Weapon chassis definitions. Each is a complete weapon platform with base
/// stats that are then modified by caliber, barrel, sight, grip, magazine,
/// and stock attachments.
use serde::{Deserialize, Serialize};

/// Weapon class determines handling characteristics independent of attachments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponClass {
    Pistol,
    SubmachineGun,
    AssaultRifle,
    BattleRifle,
    SniperRifle,
    Shotgun,
}

impl WeaponClass {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponClass::Pistol => "Pistol",
            WeaponClass::SubmachineGun => "Submachine Gun",
            WeaponClass::AssaultRifle => "Assault Rifle",
            WeaponClass::BattleRifle => "Battle Rifle",
            WeaponClass::SniperRifle => "Sniper Rifle",
            WeaponClass::Shotgun => "Shotgun",
        }
    }
}

/// A weapon chassis defines the base stats of a weapon platform before any
/// attachments are applied. Final stats = chassis base * caliber * attachments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponChassis {
    pub name: String,
    pub class: WeaponClass,
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
    /// Number of pellets fired per shot (1 for most, >1 for shotguns).
    pub pellets_per_shot: u32,
}

impl WeaponChassis {
    pub fn m4a1() -> Self {
        WeaponChassis {
            name: "M4A1".into(),
            class: WeaponClass::AssaultRifle,
            caliber: Caliber::FiveFiveSixNato,
            base_damage: 25.0,
            base_fire_rate: 10.0,
            base_magazine_size: 30,
            base_reserve_ammo: 120,
            base_reload_time: 2.1,
            base_spread_hip: 2.5,
            base_spread_ads: 0.5,
            base_recoil_vertical: 0.8,
            base_recoil_horizontal: 0.4,
            base_weight: 3.5,
            base_ads_speed: 0.25,
            base_sway: 0.003,
            max_range: 300.0,
            is_automatic: true,
            pellets_per_shot: 1,
        }
    }

    pub fn mp5sd() -> Self {
        WeaponChassis {
            name: "MP5SD".into(),
            class: WeaponClass::SubmachineGun,
            caliber: Caliber::NineMm,
            base_damage: 18.0,
            base_fire_rate: 12.0,
            base_magazine_size: 30,
            base_reserve_ammo: 90,
            base_reload_time: 2.5,
            base_spread_hip: 3.0,
            base_spread_ads: 1.0,
            base_recoil_vertical: 0.5,
            base_recoil_horizontal: 0.3,
            base_weight: 2.8,
            base_ads_speed: 0.22,
            base_sway: 0.002,
            max_range: 150.0,
            is_automatic: true,
            pellets_per_shot: 1,
        }
    }

    pub fn m1911() -> Self {
        WeaponChassis {
            name: "M1911".into(),
            class: WeaponClass::Pistol,
            caliber: Caliber::FortyFiveACP,
            base_damage: 35.0,
            base_fire_rate: 5.0,
            base_magazine_size: 7,
            base_reserve_ammo: 28,
            base_reload_time: 1.5,
            base_spread_hip: 4.0,
            base_spread_ads: 0.8,
            base_recoil_vertical: 1.2,
            base_recoil_horizontal: 0.3,
            base_weight: 1.1,
            base_ads_speed: 0.12,
            base_sway: 0.001,
            max_range: 50.0,
            is_automatic: false,
            pellets_per_shot: 1,
        }
    }

    pub fn ak47() -> Self {
        WeaponChassis {
            name: "AK-47".into(),
            class: WeaponClass::AssaultRifle,
            caliber: Caliber::SevenSixTwoX39,
            base_damage: 30.0,
            base_fire_rate: 8.0,
            base_magazine_size: 30,
            base_reserve_ammo: 90,
            base_reload_time: 2.5,
            base_spread_hip: 3.5,
            base_spread_ads: 1.5,
            base_recoil_vertical: 1.2,
            base_recoil_horizontal: 0.6,
            base_weight: 3.8,
            base_ads_speed: 0.28,
            base_sway: 0.004,
            max_range: 350.0,
            is_automatic: true,
            pellets_per_shot: 1,
        }
    }
}
