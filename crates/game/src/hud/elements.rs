use crate::states::ingame::IngameEntity;
use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBarFill;
#[derive(Component)]
pub struct StanceText;
#[derive(Component)]
pub struct AmmoText;
#[derive(Component)]
pub struct Crosshair;
#[derive(Component)]
pub struct WeaponNameText;
#[derive(Component)]
pub struct HudElement;

pub fn spawn_hud(mut commands: Commands) {
    // Health bar background
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        BackgroundColor(Color::srgb(0.2, 0.05, 0.05)),
        HudElement,
        IngameEntity,
    ));
    // Health bar fill
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(21.0),
            width: Val::Px(198.0),
            height: Val::Px(18.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
        HealthBarFill,
        HudElement,
        IngameEntity,
    ));
    // HP label
    commands.spawn((
        Text::new("HP"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(31.0),
            left: Val::Px(24.0),
            ..default()
        },
        HudElement,
        IngameEntity,
    ));
    // Stance indicator
    commands.spawn((
        Text::new("STANDING"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(55.0),
            left: Val::Px(20.0),
            ..default()
        },
        StanceText,
        HudElement,
        IngameEntity,
    ));
    // Ammo counter
    commands.spawn((
        Text::new("30 / 120"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            right: Val::Px(30.0),
            ..default()
        },
        AmmoText,
        HudElement,
        IngameEntity,
    ));
    // Weapon name
    commands.spawn((
        Text::new("M4A1"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(55.0),
            right: Val::Px(30.0),
            ..default()
        },
        WeaponNameText,
        HudElement,
        IngameEntity,
    ));
    // Crosshair outer ring (centred with Percent + Transform offset)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            width: Val::Px(16.0),
            height: Val::Px(16.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        Transform::from_xyz(-8.0, -8.0, 0.0),
        GlobalTransform::default(),
        BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
        Crosshair,
        HudElement,
        IngameEntity,
    ));
    // Crosshair centre dot
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            width: Val::Px(8.0),
            height: Val::Px(8.0),
            ..default()
        },
        Transform::from_xyz(-4.0, -4.0, 0.0),
        GlobalTransform::default(),
        BackgroundColor(Color::srgb(0.8, 0.8, 0.4)),
        Crosshair,
        HudElement,
        IngameEntity,
    ));
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudElement>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
