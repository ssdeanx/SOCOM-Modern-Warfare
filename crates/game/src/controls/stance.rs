// REALISTIC STANCE TRANSITIONS
// Transitions between stances take realistic time:
//   Standing <-> Crouching: 0.3s
//   Standing <-> Prone: 0.8s
//   Crouching <-> Prone: 0.5s
// Sprint cancels into any stance instantly (emergency drop).

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{MovementState, Player};
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
        (Standing, Prone) | (Prone, Standing) => 0.8,
        (Crouching, Prone) | (Prone, Crouching) => 0.5,
        (Sprinting, _) => 0.0, // Emergency drop
        (_, Sprinting) => 0.4,
        (_, InCover) => 0.2,
        _ => 0.3,
    }
}

/// Handles stance input with realistic transition timers.
pub fn stance_transition_system(
    time: Res<Time>,
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

    // Determine requested stance from input
    let requested = if action_state.just_pressed(&PlayerAction::Sprint) {
        if *current_stance == MovementState::Sprinting {
            Some(MovementState::Standing)
        } else {
            Some(MovementState::Sprinting)
        }
    } else if action_state.just_pressed(&PlayerAction::Crouch) {
        if *current_stance == MovementState::Crouching {
            Some(MovementState::Standing)
        } else {
            Some(MovementState::Crouching)
        }
    } else if action_state.just_pressed(&PlayerAction::Prone) {
        if *current_stance == MovementState::Prone {
            Some(MovementState::Standing)
        } else {
            Some(MovementState::Prone)
        }
    } else {
        None
    };

    if let Some(target) = requested {
        if target == *current_stance {
            return;
        }
        let trans_time = transition_time(&current_stance, &target);
        if trans_time <= 0.0 {
            // Instant transition (sprint cancel)
            *current_stance = target;
            transition.transitioning = false;
        } else {
            transition.target_stance = target;
            transition.current_timer = Timer::from_seconds(trans_time, TimerMode::Once);
            transition.transitioning = true;
        }
    }

    // Tick transition timer
    if transition.transitioning {
        transition.current_timer.tick(time.delta());
        if transition.current_timer.just_finished() {
            *current_stance = transition.target_stance;
            transition.transitioning = false;
        }
    }
}
