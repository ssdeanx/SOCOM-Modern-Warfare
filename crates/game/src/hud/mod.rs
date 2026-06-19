pub mod achievement_popup;
pub mod elements;
pub mod kill_feed;
pub mod squad_status;
pub mod stamina_bar;
pub mod systems;
pub mod xp_notification;

use crate::states::AppState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                elements::spawn_hud,
                stamina_bar::spawn_stamina_bar,
                squad_status::spawn_squad_status,
                squad_status::spawn_objective_text,
            ),
        );
        app.add_systems(OnExit(AppState::InGame), elements::cleanup_hud);
        app.add_systems(
            Update,
            (
                systems::update_health_bar,
                systems::update_stance_text,
                systems::update_ammo_text,
                systems::update_weapon_name,
                systems::crosshair_visibility_system,
                stamina_bar::update_stamina_bar,
            )
                .run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            (
                xp_notification::xp_notification_system,
                xp_notification::level_up_notification_system,
                xp_notification::popup_lifetime_system,
                achievement_popup::achievement_popup_system,
                achievement_popup::ach_popup_lifetime_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            (
                kill_feed::kill_feed_system,
                kill_feed::kill_feed_cleanup_system,
                squad_status::update_squad_status_system,
                squad_status::update_objective_text_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
