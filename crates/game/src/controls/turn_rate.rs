// REALISTIC TURN RATE LIMITING
// Stance and weapon weight affect how fast the player can turn.
// Prone = slow turn, standing = full turn, heavy weapon = slower turn.

use bevy::prelude::*;

use socom_core::components::{MovementState, Player};

use crate::stamina::Stamina;
use crate::weapon_handling::WeaponWeight;

/// Returns the turn rate multiplier based on stance and weight.
/// Applied to mouse sensitivity.
pub fn turn_rate_mult(
    stance: &MovementState,
    weight_mult: f32,
    stamina: &Stamina,
) -> f32 {
    let stance_mult = match stance {
        MovementState::Prone => 0.3,
        MovementState::Crouching => 0.6,
        MovementState::InCover => 0.5,
        MovementState::Sprinting => 0.7,
        MovementState::Standing => 1.0,
    };
    // Weight mult: Light=1.0, Medium=0.85, Heavy=0.75 -> invert for turn rate
    let weight_turn = 2.0 - weight_mult; // Heavy weapon = slower turn
    let stamina_factor = if stamina.is_exhausted() { 0.6 } else { 1.0 };
    stance_mult * weight_turn * stamina_factor
}
