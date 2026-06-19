// REALISTIC WEAPON HANDLING
// Weapon weight affects movement speed, deploy time, and sway.
// Heavier weapons = slower ADS, more sway when moving, slower movement.

use bevy::prelude::*;

use socom_core::components::WeaponSlot;

/// Weight classes for weapons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponWeight {
    Light,
    Medium,
    Heavy,
}

impl WeaponWeight {
    pub fn from_weapon_name(name: &str) -> Self {
        match name {
            "M1911" => WeaponWeight::Light,
            "MP5SD" => WeaponWeight::Medium,
            "M4A1" | "AK-47" => WeaponWeight::Heavy,
            _ => WeaponWeight::Medium,
        }
    }
    /// Movement speed multiplier while weapon is equipped.
    /// Light (pistol): full speed. Heavy (rifle): significant penalty.
    pub fn speed_mult(&self) -> f32 {
        match self {
            WeaponWeight::Light => 1.0,
            WeaponWeight::Medium => 0.75,
            WeaponWeight::Heavy => 0.55,
        }
    }
    /// Time in seconds to fully aim down sights.
    pub fn ads_time(&self) -> f32 {
        match self {
            WeaponWeight::Light => 0.12,
            WeaponWeight::Medium => 0.25,
            WeaponWeight::Heavy => 0.40,
        }
    }
    /// Base weapon sway amplitude.
    pub fn sway_amplitude(&self) -> f32 {
        match self {
            WeaponWeight::Light => 0.001,
            WeaponWeight::Medium => 0.003,
            WeaponWeight::Heavy => 0.008,
        }
    }
}

/// Tracks current weapon handling state.
#[derive(Component, Debug)]
pub struct WeaponHandling {
    pub current_ads_time: f32,
    pub current_weight_mult: f32,
    pub deploy_timer: Timer,
    pub is_deploying: bool,
}

impl Default for WeaponHandling {
    fn default() -> Self {
        Self {
            current_ads_time: 0.22,
            current_weight_mult: 0.85,
            deploy_timer: Timer::from_seconds(0.3, TimerMode::Once),
            is_deploying: false,
        }
    }
}

/// Updates weapon handling stats based on the active weapon.
pub fn weapon_handling_system(
    mut player_query: Query<
        (&WeaponSlot, &mut WeaponHandling),
        With<socom_core::components::Player>,
    >,
) {
    let Ok((weapon_slot, mut handling)) = player_query.single_mut() else {
        return;
    };
    if let Some(weapon) = weapon_slot.active_weapon() {
        let weight = WeaponWeight::from_weapon_name(&weapon.name);
        handling.current_weight_mult = weight.speed_mult();
        handling.current_ads_time = weight.ads_time();
    }
}
