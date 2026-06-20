// REALISTIC WEAPON HANDLING
// Weapon weight affects movement speed, deploy time, and sway.
// Heavier weapons = slower ADS, more sway when moving, slower movement.

use bevy::prelude::*;

use socom_core::components::Player;

use crate::weapons::EquippedWeapon;

/// Tracks current weapon handling state, derived from the equipped weapon's stats.
#[derive(Component, Debug)]
pub struct WeaponHandling {
    /// Movement speed multiplier derived from weapon weight.
    pub current_weight_mult: f32,
    /// Time in seconds to fully aim down sights.
    pub current_ads_time: f32,
    /// Base weapon sway amplitude.
    pub current_sway_amplitude: f32,
    #[expect(dead_code, reason = "awaiting deploy animation")]
    pub deploy_timer: Timer,
    #[expect(dead_code, reason = "awaiting deploy animation")]
    pub is_deploying: bool,
}

impl Default for WeaponHandling {
    fn default() -> Self {
        Self {
            current_weight_mult: 1.0,
            current_ads_time: 0.22,
            current_sway_amplitude: 0.003,
            deploy_timer: Timer::from_seconds(0.3, TimerMode::Once),
            is_deploying: false,
        }
    }
}

/// Maps weapon weight (kg) to a movement speed multiplier (0.3–1.0).
/// Heavier weapons slow the player more.
fn weight_to_speed_mult(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Linear map: 0.5kg → 1.0, 8.0kg → 0.35
    1.0 - (weight - 0.5) / 7.5 * 0.65
}

/// Maps weapon weight (kg) to ADS time in seconds (0.1–0.6).
fn weight_to_ads_time(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Linear map: 0.5kg → 0.1s, 8.0kg → 0.55s
    0.1 + (weight - 0.5) / 7.5 * 0.45
}

/// Maps weapon weight (kg) to sway amplitude (0.001–0.015).
fn weight_to_sway(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Heavier weapons sway more
    0.001 + (weight - 0.5) / 7.5 * 0.014
}

/// Updates weapon handling stats based on the equipped weapon's complete stats.
/// Uses `EquippedWeapon.final_weight` to compute speed/ADS/sway multipliers.
pub fn weapon_handling_system(
    mut player_query: Query<
        (&EquippedWeapon, &mut WeaponHandling),
        With<Player>,
    >,
) {
    let Ok((equipped, mut handling)) = player_query.single_mut() else {
        return;
    };
    let weight = equipped.weapon.final_weight;
    handling.current_weight_mult = weight_to_speed_mult(weight);
    handling.current_ads_time = weight_to_ads_time(weight);
    handling.current_sway_amplitude = weight_to_sway(weight);
}
