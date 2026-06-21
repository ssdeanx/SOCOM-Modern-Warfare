use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use crate::menu::{MenuButton, MenuUI};
use socom_core::resources::{DisplayMode, GameSettings, GraphicsQuality};

// ═══════════════════════════════════════════════════════════════════════════════
// TAB SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SettingsTab {
    #[default]
    Display,
    Graphics,
    Camera,
}

impl SettingsTab {
    pub fn all() -> [Self; 3] {
        [Self::Display, Self::Graphics, Self::Camera]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Display => "Display",
            Self::Graphics => "Graphics",
            Self::Camera => "Camera",
        }
    }
}

/// Marker for tab header buttons.
#[derive(Component)]
pub struct TabButton(pub SettingsTab);

/// Marker for the settings content area — despawned on tab switch.
#[derive(Component)]
pub struct SettingsContent;

// ═══════════════════════════════════════════════════════════════════════════════
// SETTING CONTROLS COMPONENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Identifies a setting row so the interaction system knows what to update.
#[derive(Component)]
pub struct SettingActionId(pub String);

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE SHORTCUT
// ═══════════════════════════════════════════════════════════════════════════════

/// The type passed into `with_children()` closures on `EntityCommands` in Bevy 0.18.
type ChildSpawner<'a> = RelatedSpawnerCommands<'a, bevy::ecs::hierarchy::ChildOf>;

// ═══════════════════════════════════════════════════════════════════════════════
// SPAWN: FULL SETTINGS PAGE
// ═══════════════════════════════════════════════════════════════════════════════

pub fn spawn_settings_page(commands: &mut Commands) {
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
                justify_content: JustifyContent::Start,
                padding: UiRect::top(Val::Px(40.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            MenuUI,
        ))
        .with_children(|parent| {
            spawn_title(parent);
            spawn_tab_bar(parent);
            spawn_back_button(parent);
        });

    // Spawn default (Display) tab content
    spawn_tab_content(commands, SettingsTab::Display);
}

/// Title text
fn spawn_title(parent: &mut ChildSpawner) {
    parent.spawn((
        Text::new("SETTINGS"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.7, 0.1)),
    ));
}

/// Tab bar with clickable tab buttons
fn spawn_tab_bar(parent: &mut ChildSpawner) {
    parent
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
        ))
        .with_children(|bar| {
            for tab in SettingsTab::all() {
                bar.spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(36.0),
                        border: UiRect::all(Val::Px(1.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.25, 0.9)),
                    Button,
                    TabButton(tab),
                ))
                .with_child((
                    Text::new(tab.label()),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            }
        });
}

/// Back button at the bottom of the settings page.
fn spawn_back_button(parent: &mut ChildSpawner) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
            BackgroundColor(Color::srgba(0.15, 0.15, 0.18, 0.9)),
            Button,
            MenuButton::BackToMain,
        ))
        .with_child((
            Text::new("BACK"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAB CONTENT SPAWNERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Spawn the content for the currently active settings tab.
pub fn spawn_tab_content(commands: &mut Commands, tab: SettingsTab) {
    let root = commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(12.0)),
                width: Val::Percent(80.0),
                max_width: Val::Px(500.0),
                ..default()
            },
            SettingsContent,
        ))
        .id();

    match tab {
        SettingsTab::Display => spawn_display_content(commands, root),
        SettingsTab::Graphics => spawn_graphics_content(commands, root),
        SettingsTab::Camera => spawn_camera_content(commands, root),
    };
}

// ── Display Tab ──

fn spawn_display_content(commands: &mut Commands, root: Entity) {
    commands.entity(root).with_children(|parent| {
        spawn_section_header(parent, "Display Settings");

        spawn_cycle_row(parent, "Display Mode", "display_mode");
        spawn_cycle_row(parent, "Resolution", "resolution");
        spawn_toggle_row(parent, "V-Sync", "vsync");
    });
}

// ── Graphics Tab ──

fn spawn_graphics_content(commands: &mut Commands, root: Entity) {
    commands.entity(root).with_children(|parent| {
        spawn_section_header(parent, "Graphics Quality");

        spawn_cycle_row(parent, "Quality Preset", "quality");
        spawn_toggle_row(parent, "Bloom", "bloom");
    });
}

// ── Camera Tab ──

fn spawn_camera_content(commands: &mut Commands, root: Entity) {
    commands.entity(root).with_children(|parent| {
        spawn_section_header(parent, "Camera Settings");

        spawn_fov_row(parent, "FOV (3rd Person)", "fov_3rd");
        spawn_fov_row(parent, "FOV (1st Person)", "fov_1st");
        spawn_toggle_row(parent, "Camera Collision", "collision");
        spawn_toggle_row(parent, "Camera Shake", "shake");
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// UI ROW HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn spawn_section_header(parent: &mut ChildSpawner, label: &str) {
    parent.spawn((
        Text::new(label),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.7, 0.2)),
        Node {
            margin: UiRect::bottom(Val::Px(6.0)),
            ..default()
        },
    ));
}

/// A row with label on the left and a clickable value button on the right.
fn spawn_cycle_row(
    parent: &mut ChildSpawner,
    label: &str,
    action_id: &'static str,
) {
    parent
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.7)),
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            row.spawn((
                Node {
                    width: Val::Px(180.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.4, 0.6)),
                BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
                Button,
                SettingActionId(action_id.to_string()),
            ))
            .with_child((
                Text::new("..."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// A row with label on the left and an On/Off toggle button on the right.
fn spawn_toggle_row(
    parent: &mut ChildSpawner,
    label: &str,
    action_id: &'static str,
) {
    parent
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.7)),
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            row.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.4, 0.6)),
                BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
                Button,
                SettingActionId(action_id.to_string()),
            ))
            .with_child((
                Text::new("On"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// FOV adjustment row with − value +
fn spawn_fov_row(
    parent: &mut ChildSpawner,
    label: &str,
    action_id: &'static str,
) {
    parent
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.7)),
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // − button
            let dec_id = format!("{}_dec", action_id);
            row.spawn((
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.4, 0.6)),
                BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
                Button,
                SettingActionId(dec_id),
            ))
            .with_child((
                Text::new("−"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            // Value display
            let val_id = format!("{}_val", action_id);
            row.spawn((
                Text::new("70°"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    width: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SettingActionId(val_id),
            ));

            // + button
            let inc_id = format!("{}_inc", action_id);
            row.spawn((
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.4, 0.6)),
                BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
                Button,
                SettingActionId(inc_id),
            ))
            .with_child((
                Text::new("+"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTERACTION SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Processes button clicks in the settings page.
pub fn settings_interaction_system(
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    interaction_query: Query<(&Interaction, &SettingActionId), Changed<Interaction>>,
    tab_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    content_query: Query<Entity, With<SettingsContent>>,
) {
    // ── Handle tab clicks ──
    for (interaction, tab) in &tab_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        // Despawn old content
        for entity in content_query.iter() {
            commands.entity(entity).despawn();
        }
        // Spawn new content for the selected tab
        spawn_tab_content(&mut commands, tab.0);
    }

    // ── Handle setting button clicks ──
    for (interaction, id) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match id.0.as_str() {
            // Display tab
            "display_mode" => cycle_display_mode(&mut settings),
            "resolution" => cycle_resolution(&mut settings),
            "vsync" => toggle_vsync(&mut settings),
            // Graphics tab
            "quality" => cycle_graphics_quality(&mut settings),
            "bloom" => toggle_bloom(&mut settings),
            // Camera tab
            "fov_3rd_dec" => adjust_fov(&mut settings, "fov_3rd", -5.0),
            "fov_3rd_inc" => adjust_fov(&mut settings, "fov_3rd", 5.0),
            "fov_1st_dec" => adjust_fov(&mut settings, "fov_1st", -5.0),
            "fov_1st_inc" => adjust_fov(&mut settings, "fov_1st", 5.0),
            "collision" => toggle_collision(&mut settings),
            "shake" => toggle_shake(&mut settings),
            _ => {}
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTERACTION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn cycle_display_mode(settings: &mut GameSettings) {
    settings.display_mode = settings.display_mode.next();
}

fn cycle_resolution(settings: &mut GameSettings) {
    const PRESETS: &[(u32, u32)] = &[
        (1280, 720),
        (1920, 1080),
        (2560, 1440),
        (3840, 2160),
    ];
    let current = (settings.resolution_width, settings.resolution_height);
    let pos = PRESETS.iter().position(|&r| r == current).unwrap_or(0);
    let next = (pos + 1) % PRESETS.len();
    settings.resolution_width = PRESETS[next].0;
    settings.resolution_height = PRESETS[next].1;
}

fn toggle_vsync(settings: &mut GameSettings) {
    settings.vsync = !settings.vsync;
}

fn cycle_graphics_quality(settings: &mut GameSettings) {
    settings.graphics_quality = settings.graphics_quality.next();
    settings.bloom_enabled = settings.graphics_quality.bloom_enabled();
}

fn toggle_bloom(settings: &mut GameSettings) {
    settings.bloom_enabled = !settings.bloom_enabled;
}

fn adjust_fov(settings: &mut GameSettings, target: &str, delta: f32) {
    match target {
        "fov_3rd" => {
            settings.fov_third_person = (settings.fov_third_person + delta).clamp(50.0, 120.0);
        }
        "fov_1st" => {
            settings.fov_first_person = (settings.fov_first_person + delta).clamp(60.0, 120.0);
        }
        _ => {}
    }
}

fn toggle_collision(settings: &mut GameSettings) {
    settings.camera_collision = !settings.camera_collision;
}

fn toggle_shake(settings: &mut GameSettings) {
    settings.camera_shake = !settings.camera_shake;
}

// ═══════════════════════════════════════════════════════════════════════════════
// UI UPDATE SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Updates the text labels on all setting buttons/rows to reflect the current
/// `GameSettings` values.
pub fn update_settings_ui_system(
    settings: Res<GameSettings>,
    mut query: Query<(&SettingActionId, &mut Text)>,
) {
    for (id, mut text) in query.iter_mut() {
        let new_label = match id.0.as_str() {
            // Display
            "display_mode" => settings.display_mode.label().into(),
            "resolution" => format!("{}×{}", settings.resolution_width, settings.resolution_height),
            "vsync" if settings.vsync => "On".into(),
            "vsync" => "Off".into(),
            // Graphics
            "quality" => settings.graphics_quality.label().into(),
            "bloom" if settings.bloom_enabled => "On".into(),
            "bloom" => "Off".into(),
            // Camera toggles
            "collision" if settings.camera_collision => "On".into(),
            "collision" => "Off".into(),
            "shake" if settings.camera_shake => "On".into(),
            "shake" => "Off".into(),
            // FOV values
            "fov_3rd_val" => format!("{:.0}°", settings.fov_third_person),
            "fov_1st_val" => format!("{:.0}°", settings.fov_first_person),
            _ => return,
        };
        if text.0 != new_label {
            text.0 = new_label;
        }
    }
}
