// REALISTIC STANCE TRANSITIONS
// Transitions between stances take realistic time:
//   Standing <-> Crouching: 0.3s
//   Standing <-> Prone: 1.0s (was 0.8s — ARMA-like)
//   Crouching <-> Prone: 0.6s (was 0.5s)
// Sprint cancels into any stance instantly (emergency drop).
// Supports hold-to-sprint and hold-to-crouch via GameSettings.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player};
use socom_core::resources::GameSettings;
use socom_input::actions::PlayerAction;

/// Tracks stance transition state.
#[derive(Component, Debug)]
pub struct StanceTransition {
    pub target_stance: MovementState,
    pub current_timer: Timer,
    pub transitioning: bool,
}

impl Default for StanceTransition {
    fn default() -> Self {
        Self {
            target_stance: MovementState::Standing,
            current_timer: Timer::from_seconds(0.0, TimerMode::Once),
            transitioning: false,
        }
    }
}

/// Returns the transition time in seconds between two stances.
fn transition_time(from: &MovementState, to: &MovementState) -> f32 {
    use MovementState::*;
    match (from, to) {
        (Standing, Crouching) | (Crouching, Standing) => 0.3,
        (Standing, Prone) | (Prone, Standing) => 1.0,   // Slower prone transition
        (Crouching, Prone) | (Prone, Crouching) => 0.6,
        (Sprinting, _) => 0.0, // Emergency drop — instant
        (_, Sprinting) => 0.3,
        (_, InCover) => 0.2,
        _ => 0.3,
    }
}

/// Handles stance input with realistic transition timers, support for
/// hold-to-sprint and hold-to-crouch modes.
pub fn stance_transition_system(
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut player_query: Query<
        (
            &ActionState<PlayerAction>,
            &mut MovementState,
            &mut StanceTransition,
        ),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok((action_state, mut current_stance, mut transition)) = player_query.single_mut() else {
        return;
    };

    let mut requested: Option<MovementState> = None;

    // ── Sprint (hold-to-sprint or toggle) ──────────────────────────────
    if settings.hold_to_sprint {
        let shift_held = action_state.pressed(&PlayerAction::Sprint);
        let can_enter_sprint = *current_stance == MovementState::Standing
            || *current_stance == MovementState::Sprinting;
        let wants_to_stay_sprint = *current_stance == MovementState::Sprinting;

        if shift_held && can_enter_sprint && *current_stance != MovementState::Sprinting {
            // Shift held while standing → sprint
            requested = Some(MovementState::Sprinting);
        } else if !shift_held && wants_to_stay_sprint {
            // Shift released while sprinting → walk
            requested = Some(MovementState::Standing);
        }
    } else {
        // Toggle mode
        if action_state.just_pressed(&PlayerAction::Sprint) {
            if *current_stance == MovementState::Sprinting {
                requested = Some(MovementState::Standing);
            } else if *current_stance == MovementState::Standing {
                requested = Some(MovementState::Sprinting);
            }
        }
    }

    // ── Crouch (hold-to-crouch or toggle) ──────────────────────────────
    if settings.hold_to_crouch {
        let ctrl_held = action_state.pressed(&PlayerAction::Crouch);
        let wants_to_stay_crouch = *current_stance == MovementState::Crouching;

        if ctrl_held && *current_stance != MovementState::Crouching {
            // Crouch key held while not crouching → crouch
            // But only from Standing or Sprinting
            if *current_stance == MovementState::Standing
                || *current_stance == MovementState::Sprinting
            {
                requested = Some(MovementState::Crouching);
            }
        } else if !ctrl_held && wants_to_stay_crouch {
            // Crouch key released → stand up
            requested = Some(MovementState::Standing);
        }
    } else {
        // Toggle mode
        if action_state.just_pressed(&PlayerAction::Crouch) {
            if *current_stance == MovementState::Crouching {
                requested = Some(MovementState::Standing);
            } else if *current_stance == MovementState::Standing
                || *current_stance == MovementState::Sprinting
            {
                requested = Some(MovementState::Crouching);
            }
        }
    }

    // ── Prone (always toggle, can drop from any stance) ────────────────
    if action_state.just_pressed(&PlayerAction::Prone) {
        if *current_stance == MovementState::Prone {
            requested = Some(MovementState::Standing);
        } else {
            requested = Some(MovementState::Prone);
        }
    }

    // ── Process requested stance transition ────────────────────────────
    if let Some(target) = requested {
        if target == *current_stance || transition.transitioning {
            return;
        }
        let trans_time = transition_time(&current_stance, &target);
        if trans_time <= 0.0 {
            // Instant transition (sprint emergency drop)
            *current_stance = target;
            transition.transitioning = false;
        } else {
            transition.target_stance = target;
            transition.current_timer = Timer::from_seconds(trans_time, TimerMode::Once);
            transition.transitioning = true;
        }
    }

    // ── Tick transition timer ──────────────────────────────────────────
    if transition.transitioning {
        transition.current_timer.tick(time.delta());
        if transition.current_timer.just_finished() {
            *current_stance = transition.target_stance;
            transition.transitioning = false;
        }
    }
}
