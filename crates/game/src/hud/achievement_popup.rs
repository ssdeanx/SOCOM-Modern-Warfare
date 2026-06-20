use crate::messages::AchievementUnlockMessage;
use bevy::prelude::*;

#[derive(Component)]
pub struct AchievementPopup;

pub fn achievement_popup_system(
    mut ach_reader: bevy::ecs::message::MessageReader<AchievementUnlockMessage>,
    mut commands: Commands,
    popup_query: Query<Entity, With<AchievementPopup>>,
) {
    for msg in ach_reader.read() {
        for e in &popup_query {
            commands.entity(e).despawn();
        }
        commands.spawn((
            Text::new(format!("Achievement: {}", msg.achievement)),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.6, 0.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(12.0),
                left: Val::Percent(50.0),
                ..default()
            },
            Transform::from_xyz(-100.0, 0.0, 0.0),
            GlobalTransform::default(),
            AchievementPopup,
        ));
    }
}

pub fn ach_popup_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<Entity, With<AchievementPopup>>,
    mut timer: Local<Option<Timer>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(4.0, TimerMode::Once));
    timer.tick(time.delta());
    if timer.just_finished() {
        for e in &query {
            commands.entity(e).despawn();
        }
    }
}
