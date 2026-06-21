use crate::combat::WeaponState;
use crate::controls::stance::StanceTransition;
use crate::hud::elements::*;
use bevy::prelude::*;
use socom_core::components::{Health, MovementState, Player, WeaponSlot};

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut fill_query: Query<&mut Node, With<HealthBarFill>>,
) {
    let Ok(health) = player_query.single() else {
        return;
    };
    let ratio = health.ratio();
    for mut node in fill_query.iter_mut() {
        node.width = Val::Px(198.0 * ratio.clamp(0.0, 1.0));
    }
}

pub fn update_stance_text(
    player_query: Query<(&MovementState, &StanceTransition), With<Player>>,
    mut text_query: Query<&mut Text, With<StanceText>>,
) {
    let Ok((stance, transition)) = player_query.single() else {
        return;
    };
    let label = if transition.transitioning {
        format!("{:?} → {:?}", stance, transition.target_stance)
    } else {
        let name = match stance {
            MovementState::Standing => "STANDING",
            MovementState::Sprinting => "SPRINTING",
            MovementState::Crouching => "CROUCHING",
            MovementState::Prone => "PRONE",
            MovementState::InCover => "IN COVER",
        };
        name.to_string()
    };
    for mut text in text_query.iter_mut() {
        text.0 = label.clone();
    }
}

pub fn update_ammo_text(
    player_query: Query<&WeaponState, With<Player>>,
    mut ammo_query: Query<&mut Text, (With<AmmoText>, Without<StanceText>)>,
) {
    let Ok(weapon_state) = player_query.single() else {
        return;
    };
    let label = if weapon_state.is_reloading {
        "RELOADING...".into()
    } else {
        format!("{} / {}", weapon_state.magazine, weapon_state.reserve)
    };
    for mut text in ammo_query.iter_mut() {
        text.0 = label.clone();
    }
}

pub fn update_weapon_name(
    player_query: Query<&WeaponSlot, With<Player>>,
    mut name_query: Query<&mut Text, With<WeaponNameText>>,
) {
    let Ok(weapon_slot) = player_query.single() else {
        return;
    };
    let name = weapon_slot
        .active_weapon()
        .map(|w| w.name.as_str())
        .unwrap_or("NONE");
    for mut text in name_query.iter_mut() {
        text.0 = name.into();
    }
}

pub fn crosshair_visibility_system(
    player_query: Query<&Health, With<Player>>,
    mut crosshair_query: Query<&mut Visibility, With<Crosshair>>,
) {
    let hide = player_query
        .single()
        .map_or(true, |health| !health.is_alive());
    for mut visibility in crosshair_query.iter_mut() {
        *visibility = if hide {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}
