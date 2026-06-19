// REALISTIC STAMINA SYSTEM
// Affects weapon sway, accuracy, movement speed, and breathing.
// Sprinting drains stamina, resting regens it.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_input::actions::PlayerAction;

/// Maximum stamina value.
const MAX_STAMINA: f32 = 100.0;

/// Stamina drain per second while sprinting.
const SPRINT_DRAIN_RATE: f32 = 25.0;

/// Stamina regen per second when not sprinting + not moving fast.
const REGEN_RATE: f32 = 15.0;

/// Delay (seconds) after sprinting before regen starts.
const REGEN_DELAY: f32 = 1.5;

/// Speed multiplier when stamina is fully depleted.
const EXHAUSTED_SPEED_MULT: f32 = 0.6;

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
    pub fn ratio(&self) -> f32 { self.current / self.max }
    pub fn is_exhausted(&self) -> bool { self.current <= 0.0 || self.exhausted }
}

/// Updates stamina every frame based on movement state.
pub fn stamina_system(
    time: Res<Time>,
    mut player_query: Query<(&ActionState<PlayerAction>, &MovementState, &mut Stamina), With<Player>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 { return; }
    let Ok((action_state, movement_state, mut stamina)) = player_query.single_mut() else { return; };

    let sprinting = *movement_state == MovementState::Sprinting;

    if sprinting {
        // Drain stamina while sprinting
        stamina.current = (stamina.current - SPRINT_DRAIN_RATE * dt).max(0.0);
        stamina.regen_timer.reset(); // Reset regen delay
        stamina.exhausted = stamina.current <= 0.0;
    } else {
        // Regen stamina after delay
        stamina.regen_timer.tick(time.delta());
        if stamina.regen_timer.just_finished() {
            stamina.current = (stamina.current + REGEN_RATE * dt).min(stamina.max);
            if stamina.current > 10.0 { stamina.exhausted = false; }
        }
    }
}

/// Returns the current speed multiplier based on stamina.
pub fn stamina_speed_mult(stamina: &Stamina) -> f32 {
    if stamina.is_exhausted() { EXHAUSTED_SPEED_MULT } else { 1.0 }
}

/// Returns the current weapon sway multiplier (higher = more sway).
pub fn stamina_sway_mult(stamina: &Stamina) -> f32 {
    if stamina.is_exhausted() { 2.5 }
    else if stamina.ratio() < 0.3 { 1.8 }
    else if stamina.ratio() < 0.6 { 1.3 }
    else { 1.0 }
}

/// Returns the current accuracy/spread multiplier (higher = worse accuracy).
pub fn stamina_spread_mult(stamina: &Stamina) -> f32 {
    if stamina.is_exhausted() { 2.0 }
    else if stamina.ratio() < 0.3 { 1.5 }
    else { 1.0 }
}
