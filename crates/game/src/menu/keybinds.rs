use crate::menu::MenuUI;
use bevy::prelude::*;
use socom_input::actions::PlayerAction;

pub fn spawn_keybinds_page(commands: &mut Commands, _asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.7, 0.1)),
                MenuUI,
            ));
            for (action, key) in binding_display_list() {
                let label = format!("{:?}: {:?}", action, key);
                parent.spawn((
                    Text::new(&label),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    MenuUI,
                ));
            }
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.18, 0.9)),
                    Button,
                    BackToControls,
                    MenuUI,
                ))
                .with_child((
                    Text::new("BACK"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    MenuUI,
                ));
        });
}

#[derive(Component)]
pub struct BackToControls;

fn binding_display_list() -> Vec<(PlayerAction, &'static str)> {
    vec![
        (PlayerAction::Move, "WASD"),
        (PlayerAction::Look, "Mouse"),
        (PlayerAction::Sprint, "Shift"),
        (PlayerAction::Crouch, "C"),
        (PlayerAction::Prone, "Z"),
        (PlayerAction::Jump, "Space"),
        (PlayerAction::Interact, "E"),
        (PlayerAction::Fire, "Left Click"),
        (PlayerAction::Aim, "Right Click"),
        (PlayerAction::Reload, "R"),
        (PlayerAction::SwapPrimary, "1"),
        (PlayerAction::SwapSidearm, "2"),
        (PlayerAction::ShoulderSwap, "Q"),
    ]
}
