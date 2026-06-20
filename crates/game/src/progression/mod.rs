pub mod achievements;
pub mod specializations;
pub mod stats;
pub mod xp;

use bevy::prelude::*;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<xp::PlayerProgression>();
        app.init_resource::<stats::PlayerStats>();
        app.init_resource::<achievements::AchievementTracker>();
        app.add_systems(
            Update,
            (
                xp::xp_event_listener,
                stats::damage_event_listener,
                stats::death_event_listener,
                achievements::achievement_checker,
            ),
        );
    }
}
