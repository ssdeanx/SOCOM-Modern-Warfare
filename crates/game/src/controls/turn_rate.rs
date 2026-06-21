// REALISTIC TURN RATE LIMITING
// Stance, weapon weight, stamina, ADS, and sprint braking affect turn speed.
// Prone = very slow turn, standing = full turn, heavy weapon = slower turn.
// ADS reduces turn rate by ~30%. Sprint braking reduces by ~40%.

use bevy::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_core::resources::SensitivityMultiplier;

use crate::combat::weapon_bob::AdsState;
use crate::physics::movement_modifiers::SprintBrake;
use crate::stamina::{stamina_effects, Stamina};
use crate::weapon_handling::WeaponHandling;

/// Returns the turn rate multiplier based on stance, weight, stamina, ADS factor,
/// and sprint braking state.
/// Applied to mouse sensitivity.
pub fn turn_rate_mult(
    stance: &MovementState,
    weight_mult: f32,
    stamina: &Stamina,
    ads_factor: f32,
    sprint_braking: bool,
) -> f32 {
    // Stance-based turn rate multiplier
    let stance_mult = match stance {
        MovementState::Prone => 0.25,       // Very slow
        MovementState::Crouching => 0.55,   // Reduced
        MovementState::InCover => 0.5,      // Cover restricts movement
        MovementState::Sprinting => 0.65,   // Sprinting limits turning
        MovementState::Standing => 1.0,     // Full freedom
    };

    // Weight: heavier = slower turn (invert: 1.0→1.0, 0.25→1.75)
    let weight_turn = 2.0 - weight_mult; // 2.0 - 0.25 = 1.75 (57% of normal)

    // Stamina multi-tier effects
    let effects = stamina_effects(stamina, weight_mult);
    let stamina_turn = effects.turn_rate_mult;

    // ADS reduction: turning while aiming is slower
    let ads_turn = 1.0 - ads_factor * 0.4; // 1.0 at hip, 0.6 at full ADS

    // Sprint braking: even slower turning during brake
    let brake_turn = if sprint_braking { 0.6 } else { 1.0 };

    stance_mult * weight_turn * stamina_turn * ads_turn * brake_turn
}

/// System that writes the computed turn-rate multiplier to the
/// SensitivityMultiplier resource so the camera system can use it.
pub fn update_sensitivity_multiplier(
    player_query: Query<(&MovementState, &Stamina, &WeaponHandling, &SprintBrake), With<Player>>,
    ads_state: Res<AdsState>,
    mut sens: ResMut<SensitivityMultiplier>,
) {
    if let Ok((stance, stamina, handling, brake)) = player_query.single() {
        sens.0 = turn_rate_mult(
            stance,
            handling.current_weight_mult,
            stamina,
            ads_state.factor,
            brake.timer.fraction() < 1.0,
        );
    }
}
