use crate::hud::elements::HudElement;
use crate::stamina::Stamina;
use bevy::prelude::*;
use socom_core::components::Player;

#[derive(Component)]
pub struct StaminaBarFill;

pub fn spawn_stamina_bar(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(55.0),
            left: Val::Px(20.0),
            width: Val::Px(100.0),
            height: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        BackgroundColor(Color::srgb(0.05, 0.05, 0.15)),
        HudElement,
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(56.0),
            left: Val::Px(21.0),
            width: Val::Px(98.0),
            height: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.5, 1.0)),
        StaminaBarFill,
        HudElement,
    ));
}

pub fn update_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut fill_query: Query<&mut Node, With<StaminaBarFill>>,
) {
    let Ok(stamina) = player_query.single() else {
        return;
    };
    let ratio = stamina.ratio();
    for mut node in fill_query.iter_mut() {
        node.width = Val::Px(98.0 * ratio.clamp(0.0, 1.0));
    }
}
