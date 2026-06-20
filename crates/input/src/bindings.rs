use bevy::prelude::{GamepadButton, KeyCode, MouseButton};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::gamepad::GamepadStick;
use leafwing_input_manager::user_input::mouse::MouseMove;
use leafwing_input_manager::user_input::virtual_axial::VirtualDPad;

use crate::actions::PlayerAction;

/// Returns the default keyboard + gamepad input map for PlayerAction.
pub fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    // Keyboard bindings
    input_map.insert_dual_axis(PlayerAction::Move, VirtualDPad::wasd());
    input_map.insert_dual_axis(PlayerAction::Look, MouseMove::default());
    input_map.insert(PlayerAction::Sprint, KeyCode::ShiftLeft);
    input_map.insert(PlayerAction::Crouch, KeyCode::KeyC);
    input_map.insert(PlayerAction::Prone, KeyCode::KeyZ);
    input_map.insert(PlayerAction::Jump, KeyCode::Space);
    input_map.insert(PlayerAction::Interact, KeyCode::KeyE);
    input_map.insert(PlayerAction::Pause, KeyCode::Escape);
    input_map.insert(PlayerAction::Fire, MouseButton::Left);
    input_map.insert(PlayerAction::Aim, MouseButton::Right);
    input_map.insert(PlayerAction::Reload, KeyCode::KeyR);
    input_map.insert(PlayerAction::SwapPrimary, KeyCode::Digit1);
    input_map.insert(PlayerAction::SwapSidearm, KeyCode::Digit2);
    input_map.insert(PlayerAction::ShoulderSwap, KeyCode::KeyQ);
    input_map.insert(PlayerAction::CycleEquipment, KeyCode::KeyX);
    input_map.insert(PlayerAction::UseEquipment, KeyCode::KeyG);
    input_map.insert(PlayerAction::Melee, KeyCode::KeyF);

    // Gamepad bindings
    input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);
    input_map.insert_dual_axis(PlayerAction::Look, GamepadStick::RIGHT);
    input_map.insert(PlayerAction::Sprint, GamepadButton::LeftThumb);
    input_map.insert(PlayerAction::Crouch, GamepadButton::South);
    input_map.insert(PlayerAction::Prone, GamepadButton::DPadDown);
    input_map.insert(PlayerAction::Jump, GamepadButton::East);
    input_map.insert(PlayerAction::Interact, GamepadButton::North);
    input_map.insert(PlayerAction::Pause, GamepadButton::Start);
    input_map.insert(PlayerAction::Fire, GamepadButton::RightTrigger);
    input_map.insert(PlayerAction::Aim, GamepadButton::LeftTrigger);
    input_map.insert(PlayerAction::Reload, GamepadButton::West);
    input_map.insert(PlayerAction::SwapPrimary, GamepadButton::DPadUp);
    input_map.insert(PlayerAction::SwapSidearm, GamepadButton::DPadRight);
    input_map.insert(PlayerAction::ShoulderSwap, GamepadButton::LeftTrigger2);
    input_map.insert(PlayerAction::CycleEquipment, GamepadButton::DPadLeft);
    input_map.insert(PlayerAction::UseEquipment, GamepadButton::RightTrigger2);
    input_map.insert(PlayerAction::Melee, GamepadButton::RightThumb);

    input_map
}
