use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Player, WeaponSlot};
use socom_input::actions::PlayerAction;

use crate::combat::weapon_state::{OffhandWeaponState, WeaponState};
use crate::weapons::EquippedWeapon;

/// Handles 1/2 key presses to swap between primary (slot 0) and sidearm (slot 1).
/// Preserves per-slot ammo state by exchanging WeaponState <-> OffhandWeaponState.
pub fn weapon_swap_system(
    mut player_query: Query<
        (
            &ActionState<PlayerAction>,
            &mut WeaponSlot,
            &mut WeaponState,
            &mut OffhandWeaponState,
        ),
        With<Player>,
    >,
) {
    for (action_state, mut weapon_slot, mut weapon_state, mut offhand) in player_query.iter_mut() {
        let should_swap = if action_state.just_pressed(&PlayerAction::SwapPrimary) {
            weapon_slot.active_slot != 0
        } else if action_state.just_pressed(&PlayerAction::SwapSidearm) {
            weapon_slot.active_slot != 1
        } else {
            false
        };

        if !should_swap {
            continue;
        }

        let active = &mut *weapon_state;
        std::mem::swap(active, &mut offhand.0);

        weapon_slot.active_slot = if weapon_slot.active_slot == 0 { 1 } else { 0 };

        weapon_state.slot_index = weapon_slot.active_slot;
        offhand.0.slot_index = if weapon_slot.active_slot == 0 { 1 } else { 0 };
    }
}

/// Manual reload triggered by pressing R.
pub fn reload_input_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<
        (&ActionState<PlayerAction>, &EquippedWeapon, &mut WeaponState),
        With<Player>,
    >,
) {
    for (action_state, equipped, mut weapon_state) in player_query.iter_mut() {
        if !action_state.just_pressed(&PlayerAction::Reload) {
            continue;
        }
        if weapon_state.is_reloading {
            continue;
        }
        let weapon = &equipped.weapon;
        if weapon_state.magazine >= weapon.final_magazine_size || weapon_state.reserve == 0 {
            continue;
        }
        weapon_state.is_reloading = true;
        weapon_state.reload_timer = weapon.final_reload_time;

        let handle: Handle<AudioSource> = asset_server.load("audio/ui_click.ogg");
        commands.spawn((
            AudioPlayer(handle),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.6)),
        ));
    }
}

/// Each frame, ticks down the reload timer. When complete, refills magazine.
pub fn reload_tick_system(
    time: Res<Time>,
    mut player_query: Query<(&mut WeaponState, &EquippedWeapon), With<Player>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (mut weapon_state, equipped) in player_query.iter_mut() {
        if !weapon_state.is_reloading {
            continue;
        }
        weapon_state.reload_timer -= dt;
        if weapon_state.reload_timer <= 0.0 {
            weapon_state.is_reloading = false;
            let weapon = &equipped.weapon;
            let needed = weapon.final_magazine_size - weapon_state.magazine;
            let available = weapon_state.reserve.min(needed);
            weapon_state.magazine += available;
            weapon_state.reserve -= available;
        }
    }
}
