pub mod keybinds;
pub mod settings;

use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::states::AppState;
use crate::states::GameMode;

use self::settings::{SettingsContent, SettingsTab, TabButton, SettingActionId};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuPage {
    Main,
    Settings,
    Keybinds,
    #[expect(dead_code, reason = "awaiting quit confirmation dialog")]
    QuitConfirm,
}

#[derive(Resource)]
pub struct MenuState {
    pub page: MenuPage,
    pub settings_tab: settings::SettingsTab,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            page: MenuPage::Main,
            settings_tab: settings::SettingsTab::Display,
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>();
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (setup_main_menu, release_cursor),
        );
        app.add_systems(OnExit(AppState::MainMenu), cleanup_menu);
        app.add_systems(
            Update,
            (
                menu_navigation_system,
                main_menu_ui_system,
                settings::settings_interaction_system,
                settings::update_settings_ui_system,
            )
                .run_if(in_state(AppState::MainMenu)),
        );
    }
}

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
enum MenuButton {
    NewGame,
    Training,
    Settings,
    Controls,
    Quit,
    BackToMain,
}

fn setup_main_menu(mut state: ResMut<MenuState>) {
    state.page = MenuPage::Main;
}

fn release_cursor(mut cursor_query: Query<&mut bevy::window::CursorOptions>) {
    if let Ok(mut cursor) = cursor_query.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn menu_navigation_system(
    mut menu_state: ResMut<MenuState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut game_mode: ResMut<GameMode>,
    menu_ui_query: Query<Entity, With<MenuUI>>,
    interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
) {
    // ── Handle page navigation buttons ──
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        for e in &menu_ui_query {
            commands.entity(e).despawn();
        }
        match button {
            MenuButton::NewGame => {
                *game_mode = GameMode::Campaign;
                next_state.set(AppState::Loading);
            }
            MenuButton::Training => {
                *game_mode = GameMode::Training;
                next_state.set(AppState::Loading);
            }
            MenuButton::Settings => {
                menu_state.page = MenuPage::Settings;
            }
            MenuButton::Controls => {
                menu_state.page = MenuPage::Keybinds;
            }
            MenuButton::Quit => {
                std::process::exit(0);
            }
            MenuButton::BackToMain => {
                menu_state.page = MenuPage::Main;
            }
        }
    }
}

fn main_menu_ui_system(
    mut commands: Commands,
    menu_state: Res<MenuState>,
) {
    match menu_state.page {
        MenuPage::Main => {
            spawn_main_page(&mut commands);
        }
        MenuPage::Settings => {
            spawn_settings_page(&mut commands);
        }
        MenuPage::Keybinds => {
            spawn_keybinds_page(&mut commands);
        }
        MenuPage::QuitConfirm => std::process::exit(0),
    }
}

fn spawn_main_page(commands: &mut Commands) {
    // Spawn camera so the menu UI is visible
    commands.spawn((Camera2d, MenuUI));
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
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            MenuUI,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("SOCOM"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.7, 0.1)),
            ));
            p.spawn((
                Text::new("Tactical Shooter"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
            p.spawn((Node {
                height: Val::Px(40.0),
                ..default()
            },));
            // Buttons inlined
            for (label, btn) in [
                ("NEW GAME", MenuButton::NewGame),
                ("TRAINING", MenuButton::Training),
                ("SETTINGS", MenuButton::Settings),
                ("CONTROLS", MenuButton::Controls),
                ("QUIT", MenuButton::Quit),
            ] {
                p.spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(1.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.18, 0.9)),
                    Button,
                    btn,
                ))
                .with_child((
                    Text::new(label),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            }
            p.spawn((
                Text::new("v0.1.0 - Phase 1"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.3, 0.3)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    ..default()
                },
            ));
        });
}

fn spawn_settings_page(commands: &mut Commands) {
    // Use the new interactive settings page
    settings::spawn_settings_page(commands);
}

fn spawn_keybinds_page(commands: &mut Commands) {
    keybinds::spawn_keybinds_page(commands);
}
