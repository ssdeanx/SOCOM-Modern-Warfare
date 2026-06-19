use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use socom_core::resources::Paused;

use crate::states::AppState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Paused(false));
        app.add_systems(OnEnter(AppState::InGame), resume_from_pause);
        app.add_systems(Update, (toggle_pause_system, pause_menu_ui_system));
    }
}

fn toggle_pause_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut paused: ResMut<Paused>,
    mut cursor_query: Query<&mut bevy::window::CursorOptions>,
    state: Res<State<AppState>>,
) {
    if *state.get() != AppState::InGame {
        return;
    }
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    paused.0 = !paused.0;
    if let Ok(mut cursor) = cursor_query.single_mut() {
        if paused.0 {
            cursor.grab_mode = CursorGrabMode::None;
            cursor.visible = true;
        } else {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;
        }
    }
}

fn resume_from_pause(mut paused: ResMut<Paused>) {
    paused.0 = false;
}

#[derive(Component)]
struct PauseOverlay;
#[derive(Component)]
enum PauseMenuButton {
    Resume,
    SaveGame,
    LoadGame,
    MainMenu,
    Quit,
}

fn pause_menu_ui_system(
    paused: Res<Paused>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &PauseMenuButton), Changed<Interaction>>,
    pause_ui_query: Query<Entity, With<PauseOverlay>>,
) {
    if paused.0 && pause_ui_query.is_empty() {
        spawn_pause_ui(&mut commands);
    } else if !paused.0 && !pause_ui_query.is_empty() {
        for e in &pause_ui_query {
            commands.entity(e).despawn();
        }
    }
    if !paused.0 {
        return;
    }
    for (_, interaction, button) in interaction_query.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            PauseMenuButton::SaveGame => {
                // Trigger save via save_load::SaveManager
                let _ = commands; // save handled in auto_save_system
            }
            PauseMenuButton::LoadGame => {
                // Will trigger a reload of the state
            }
            _ => {
                for e in &pause_ui_query {
                    commands.entity(e).despawn();
                }
            }
        }
        match button {
            PauseMenuButton::Resume => {}
            PauseMenuButton::SaveGame | PauseMenuButton::LoadGame => {}
            PauseMenuButton::MainMenu => {
                next_state.set(AppState::MainMenu);
            }
            PauseMenuButton::Quit => {
                std::process::exit(0);
            }
        }
    }
}

fn spawn_pause_ui(commands: &mut Commands) {
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
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            PauseOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                PauseOverlay,
            ));
            // Resume
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                    Button,
                    PauseMenuButton::Resume,
                    PauseOverlay,
                ))
                .with_child((
                    Text::new("Resume"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    PauseOverlay,
                ));
            // Save Game
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                    BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 0.9)),
                    Button,
                    PauseMenuButton::SaveGame,
                    PauseOverlay,
                ))
                .with_child((
                    Text::new("Save Game (F5)"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.8, 1.0)),
                    PauseOverlay,
                ));
            // Load Game
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                    BackgroundColor(Color::srgba(0.2, 0.15, 0.15, 0.9)),
                    Button,
                    PauseMenuButton::LoadGame,
                    PauseOverlay,
                ))
                .with_child((
                    Text::new("Load Game (F9)"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.7, 0.7)),
                    PauseOverlay,
                ));
            // Main Menu
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                    Button,
                    PauseMenuButton::MainMenu,
                    PauseOverlay,
                ))
                .with_child((
                    Text::new("Return to Main Menu"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    PauseOverlay,
                ));
            // Quit
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                    BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)),
                    Button,
                    PauseMenuButton::Quit,
                    PauseOverlay,
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.3, 0.3)),
                    PauseOverlay,
                ));
        });
}
