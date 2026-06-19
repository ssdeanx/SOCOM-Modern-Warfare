use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use socom_core::components::{MovementState, Player};
use socom_input::actions::PlayerAction;

pub fn player_stance_system(
    mut query: Query<(&ActionState<PlayerAction>, &mut MovementState), With<Player>>,
) {
    for (action_state, mut stance) in query.iter_mut() {
        if action_state.just_pressed(&PlayerAction::Sprint) {
            *stance = match *stance {
                MovementState::Standing => MovementState::Sprinting,
                MovementState::Sprinting => MovementState::Standing,
                other => other,
            };
        }
        if action_state.just_pressed(&PlayerAction::Crouch) {
            *stance = match *stance {
                MovementState::Standing | MovementState::Sprinting => MovementState::Crouching,
                MovementState::Crouching => MovementState::Standing,
                other => other,
            };
        }
        if action_state.just_pressed(&PlayerAction::Prone) {
            *stance = match *stance {
                MovementState::Standing | MovementState::Sprinting | MovementState::Crouching => {
                    MovementState::Prone
                }
                MovementState::Prone => MovementState::Standing,
                other => other,
            };
        }
    }
}
