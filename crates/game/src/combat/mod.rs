pub mod damage;
pub mod death;
pub mod impacts;
pub mod reload;
pub mod shooting;
pub mod weapon_bob;
pub mod weapon_model;
pub mod weapon_state;

// Re-exports for external consumers — some appear unused locally but are used across modules.
#[allow(unused_imports)]
pub use damage::*;
#[allow(unused_imports)]
pub use death::{DeathMessage, DeathScreenUI, RespawnState};
#[allow(unused_imports)]
pub use impacts::ImpactMarker;
#[allow(unused_imports)]
pub use weapon_model::{
    spawn_weapon_model, weapon_model_flash_system, weapon_model_swap_system,
    weapon_shoulder_mirror_system, MuzzleFlashLight, WeaponModelRoot,
};
#[allow(unused_imports)]
pub use weapon_state::{OffhandWeaponState, WeaponState};

use bevy::prelude::*;

use socom_core::resources::is_not_paused;

use crate::combat::weapon_bob::AdsState;

use crate::states::AppState;

/// Plugin for all combat-related systems.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AdsState::default());
        app.add_message::<damage::DamageMessage>();
        app.add_message::<DeathMessage>();

        app.add_systems(OnEnter(AppState::InGame), spawn_weapon_model);

        app.add_systems(
            Update,
            (
                reload::weapon_swap_system,
                weapon_model_swap_system,
                reload::reload_input_system,
                reload::reload_tick_system,
                shooting::shooting_system,
                damage::apply_damage_system,
                damage::death_check_system,
                death::handle_player_death,
                death::respawn_system,
                weapon_model_flash_system,
                weapon_model::weapon_shoulder_mirror_system,
                weapon_bob::weapon_bob_system,
                weapon_bob::ads_fov_system,
                impacts::impact_lifetime_system,
            )
                .chain()
                .run_if(is_not_paused),
        );
    }
}
