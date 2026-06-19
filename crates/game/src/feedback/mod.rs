pub mod enemy_fx;
pub mod hit_marker;
pub mod vignette;

use bevy::prelude::*;

use socom_core::resources::is_not_paused;

pub struct FeedbackPlugin;

impl Plugin for FeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hit_marker::hit_marker_detect_system,
                hit_marker::hit_marker_lifetime_system,
                vignette::damage_vignette_system,
                enemy_fx::enemy_hurt_flash_system,
                enemy_fx::hurt_flash_lifetime_system,
                enemy_fx::enemy_death_effect_system,
                enemy_fx::death_particle_lifetime_system,
            )
                .run_if(is_not_paused),
        );
    }
}
