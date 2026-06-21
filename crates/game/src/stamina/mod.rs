// REALISTIC STAMINA SYSTEM
// Affects weapon sway, accuracy, movement speed, and breathing.
// Sprinting drains stamina, resting regens it.
// Weight-dependent drain: heavier weapons drain faster.
// 4 stamina tiers: Full (≥75%), Moderate (≥50%), Low (≥25%), Exhausted (<25%).

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_input::actions::PlayerAction;

use crate::weapon_handling::WeaponHandling;

/// Maximum stamina value.
const MAX_STAMINA: f32 = 100.0;

/// Base sprint drain rate per second (for a light weapon).
const BASE_SPRINT_DRAIN_RATE: f32 = 18.0;

/// Additional drain per inverted weight unit.
const WEIGHT_DRAIN_FACTOR: f32 = 4.0;

/// Stamina regen per second when resting.
const REGEN_RATE: f32 = 12.0;

/// Delay (seconds) after sprinting before regen starts.
const REGEN_DELAY: f32 = 2.0;

/// Delay when exhausted before regen starts (even longer).
const EXHAUSTED_REGEN_DELAY: f32 = 3.0;

/// Threshold above which exhausted status is cleared.
const EXHAUSTED_CLEAR_THRESHOLD: f32 = 20.0;

/// Multi-level stamina effects returned by stamina_effects().
#[derive(Debug, Clone, Copy)]
pub struct StaminaEffects {
    /// Movement speed multiplier (1.0 = full speed).
    pub speed_mult: f32,
    /// Weapon sway multiplier (1.0 = normal sway).
    pub sway_mult: f32,
    /// Accuracy/spread multiplier (1.0 = normal spread).
    pub spread_mult: f32,
    /// Turn rate multiplier (1.0 = normal turn speed).
    pub turn_rate_mult: f32,
    /// Whether the player is breathing heavily (for audio cues).
    pub breathing_heavy: bool,
}

/// Tracks the player's stamina and regen cooldown.
#[derive(Component, Debug)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_timer: Timer,
    pub exhausted: bool,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: MAX_STAMINA,
            max: MAX_STAMINA,
            regen_timer: Timer::from_seconds(REGEN_DELAY, TimerMode::Once),
            exhausted: false,
        }
    }
}

impl Stamina {
    pub fn ratio(&self) -> f32 {
        self.current / self.max
    }

    pub fn is_exhausted(&self) -> bool {
        self.current <= 0.0 || self.exhausted
    }

    /// Returns true if the player can sprint based on stamina state.
    pub fn can_sprint(&self) -> bool {
        !self.exhausted || self.ratio() > 0.2
    }
}

/// Compute stamina effects based on current stamina and weapon weight.
pub fn stamina_effects(stamina: &Stamina, _weight_mult: f32) -> StaminaEffects {
    let ratio = stamina.ratio();

    // Speed mult based on stamina tier
    let speed_mult = if ratio >= 0.5 {
        1.0
    } else if ratio >= 0.25 {
        0.85
    } else if ratio > 0.0 {
        0.70
    } else {
        0.55 // exhausted
    };

    // Sway mult based on stamina tier
    let sway_mult = if ratio >= 0.6 {
        1.0
    } else if ratio >= 0.3 {
        1.5
    } else if ratio > 0.0 {
        2.0
    } else {
        2.5 // exhausted
    };

    // Spread mult based on stamina tier
    let spread_mult = if ratio >= 0.5 {
        1.0
    } else if ratio >= 0.25 {
        1.3
    } else {
        1.8
    };

    // Turn rate mult based on stamina tier
    let turn_rate_mult = if ratio >= 0.5 {
        1.0
    } else if ratio >= 0.25 {
        0.8
    } else {
        0.6
    };

    StaminaEffects {
        speed_mult,
        sway_mult,
        spread_mult,
        turn_rate_mult,
        breathing_heavy: ratio < 0.3,
    }
}

/// Updates stamina every frame based on movement state and weapon weight.
pub fn stamina_system(
    time: Res<Time>,
    mut player_query: Query<
        (&ActionState<PlayerAction>, &MovementState, &mut Stamina, &WeaponHandling),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok((_action_state, movement_state, mut stamina, handling)) = player_query.single_mut() else {
        return;
    };

    let sprinting = *movement_state == MovementState::Sprinting;

    if sprinting {
        // Drain stamina while sprinting
        // Base rate + weight penalty (heavier = faster drain)
        // weight_mult: 1.0 (light) to ~0.25 (heavy)
        // Light weapon (1.0): penalty = 0.0, drain = BASE
        // Heavy weapon (0.25): penalty = (1.0-0.25)*2.0 = 1.5, drain = BASE + 1.5*4.0 = BASE + 6.0
        let weight_penalty = (1.0 - handling.current_weight_mult) * 2.0;
        let drain_rate = BASE_SPRINT_DRAIN_RATE + weight_penalty * WEIGHT_DRAIN_FACTOR;

        stamina.current = (stamina.current - drain_rate * dt).max(0.0);
        stamina.regen_timer.reset(); // Reset regen delay
        stamina.exhausted = stamina.current <= 0.0;
    } else {
        // Regen stamina after delay
        stamina.regen_timer.tick(time.delta());
        if stamina.regen_timer.just_finished() {
            stamina.current = (stamina.current + REGEN_RATE * dt).min(stamina.max);
            if stamina.current > EXHAUSTED_CLEAR_THRESHOLD {
                stamina.exhausted = false;
            }
        }
    }
}

/// Returns the current speed multiplier based on stamina.
pub fn stamina_speed_mult(stamina: &Stamina) -> f32 {
    let eff = stamina_effects(stamina, 1.0);
    eff.speed_mult
}

/// Returns the current weapon sway multiplier (higher = more sway).
pub fn stamina_sway_mult(stamina: &Stamina) -> f32 {
    let eff = stamina_effects(stamina, 1.0);
    eff.sway_mult
}

/// Returns the current accuracy/spread multiplier (higher = worse accuracy).
pub fn stamina_spread_mult(stamina: &Stamina) -> f32 {
    let eff = stamina_effects(stamina, 1.0);
    eff.spread_mult
}
