use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    World,
    Player,
    Enemy,
    Bullet,
    Camera,
}

#[derive(Component, Clone, Debug)]
pub struct CharacterController {
    /// Current velocity of the character (m/s).
    pub velocity: Vec3,
    /// Whether the character is on the ground this frame (set by movement system).
    pub on_ground: bool,
    /// Y position when the character left the ground (for fall-damage calculation).
    pub fall_start_y: f32,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            on_ground: false,
            fall_start_y: 0.0,
        }
    }
}
