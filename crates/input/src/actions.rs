use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

/// All player actions for Phase 0–1
#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
pub enum PlayerAction {
    /// WASD / Left Stick
    Move,
    /// Mouse / Right Stick
    Look,
    /// Shift / L3
    Sprint,
    /// C / B
    Crouch,
    /// Z / Down D-Pad
    Prone,
    /// Space / A
    Jump,
    /// E / Y
    Interact,
    /// Escape / Start
    Pause,
    /// Left Mouse / Right Trigger
    Fire,
    /// Right Mouse / Left Trigger
    Aim,
    /// R / X
    Reload,
    /// 1 / DPadUp — swap to primary
    SwapPrimary,
    /// 2 / DPadRight — swap to sidearm
    SwapSidearm,
    /// Q / LeftShoulder — swap camera shoulder
    ShoulderSwap,
}
