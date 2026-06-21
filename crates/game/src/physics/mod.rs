pub mod enemy_movement;
pub mod layers;
pub mod movement_modifiers;
pub mod player_movement;
pub mod stance;

pub use layers::*;

use bevy::prelude::*;

use crate::ai::AiSystems;
use socom_core::resources::is_not_paused;
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(movement_modifiers::MovementModifiersPlugin);
        app.add_systems(
            Update,
            player_movement::player_movement_system
                .after(AiSystems)
                .run_if(is_not_paused),
        );
        app.add_systems(Update, stance::player_stance_system.run_if(is_not_paused));
        app.add_systems(
            Update,
            enemy_movement::enemy_movement_system
                .after(AiSystems)
                .run_if(is_not_paused),
        );
    }
}
