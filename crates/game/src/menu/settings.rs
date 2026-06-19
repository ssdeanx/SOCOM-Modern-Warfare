use crate::menu::MenuUI;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingsTab {
    Display,
    Audio,
    Controls,
}

pub fn spawn_settings_page(commands: &mut Commands) {
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
                row_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.7, 0.1)),
                MenuUI,
            ));
            parent.spawn((
                Text::new("Display - Audio - Controls"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                MenuUI,
            ));
            // Display content
            parent.spawn((
                Text::new("Resolution: 1280 x 720"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                MenuUI,
            ));
            parent.spawn((
                Text::new("Fullscreen: Off"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                MenuUI,
            ));
            parent.spawn((
                Text::new("Master Volume: 80%"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                MenuUI,
            ));
            parent.spawn((
                Text::new("Sensitivity: 1.0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                MenuUI,
            ));
            // Back button
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
                    BackToSettings,
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
pub struct BackToSettings;
