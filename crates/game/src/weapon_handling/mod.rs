// REALISTIC WEAPON HANDLING
// Weapon weight affects movement speed, deploy time, and sway.
// Heavier weapons = slower ADS, more sway when moving, slower movement.
// More aggressive weight curves for SOCOM/SQUAD feel.

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

/// Maps weapon weight (kg) to a movement speed multiplier (0.25–1.0).
/// More aggressive: 0.5kg → 1.0, 8.0kg → 0.25 (was 0.35).
fn weight_to_speed_mult(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Linear map: 0.5kg → 1.0, 8.0kg → 0.25
    1.0 - (weight - 0.5) / 7.5 * 0.75
}

/// Maps weapon weight (kg) to ADS time in seconds (0.12–0.6).
/// Heavier weapons take longer to ADS: 0.5kg → 0.12s, 8.0kg → 0.6s (was 0.55).
fn weight_to_ads_time(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Linear map: 0.5kg → 0.12s, 8.0kg → 0.6s
    0.12 + (weight - 0.5) / 7.5 * 0.48
}

/// Maps weapon weight (kg) to sway amplitude (0.001–0.018).
/// Heavier weapons sway more aggressively.
fn weight_to_sway(weight_kg: f32) -> f32 {
    let weight = weight_kg.clamp(0.5, 8.0);
    // Heavier weapons sway more: 0.5kg → 0.001, 8.0kg → 0.018 (was 0.015)
    0.001 + (weight - 0.5) / 7.5 * 0.017
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
