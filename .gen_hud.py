
import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
base = "crates/game/src/hud"

with open(os.path.join(base, "xp_notification.rs"), "w", encoding="utf-8") as f:
    f.write("""use bevy::prelude::*;
use crate::messages::{XpGainedMessage, LevelUpMessage};

#[derive(Component)]
pub struct XpPopup;
#[derive(Component)]
pub struct LevelUpPopup;

pub fn xp_notification_system(
    mut xp_reader: bevy::ecs::message::MessageReader<XpGainedMessage>,
    mut commands: Commands,
    popup_query: Query<Entity, With<XpPopup>>,
) {
    for msg in xp_reader.read() {
        for e in &popup_query { commands.entity(e).despawn(); }
        commands.spawn((
            Text::new(format!("+{} XP", msg.amount)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(1.0, 0.85, 0.0)),
            Node { position_type: PositionType::Absolute, top: Val::Percent(15.0), left: Val::Percent(50.0), ..default() },
            Transform::from_xyz(-40.0, 0.0, 0.0), GlobalTransform::default(), XpPopup,
        ));
    }
}

pub fn level_up_notification_system(
    mut level_reader: bevy::ecs::message::MessageReader<LevelUpMessage>,
    mut commands: Commands, popup_query: Query<Entity, With<LevelUpPopup>>,
) {
    for msg in level_reader.read() {
        for e in &popup_query { commands.entity(e).despawn(); }
        commands.spawn((
            Text::new(format!("LEVEL UP! Level {}", msg.new_level)),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::srgb(0.2, 1.0, 0.2)),
            Node { position_type: PositionType::Absolute, top: Val::Percent(20.0), left: Val::Percent(50.0), ..default() },
            Transform::from_xyz(-100.0, 0.0, 0.0), GlobalTransform::default(), LevelUpPopup,
        ));
    }
}

pub fn popup_lifetime_system(
    time: Res<Time>, mut commands: Commands,
    xp_query: Query<Entity, With<XpPopup>>,
    level_query: Query<Entity, With<LevelUpPopup>>,
    mut timers: Local<Vec<(Entity, Timer)>>,
) {
    timers.retain(|(entity, timer)| {
        timer.tick(time.delta());
        if timer.just_finished() { commands.entity(*entity).despawn(); false } else { true }
    });
    for entity in &xp_query {
        if !timers.iter().any(|(e, _)| *e == entity) { timers.push((entity, Timer::from_seconds(2.0, TimerMode::Once))); }
    }
    for entity in &level_query {
        if !timers.iter().any(|(e, _)| *e == entity) { timers.push((entity, Timer::from_seconds(3.0, TimerMode::Once))); }
    }
}
""")
print("Created xp_notification.rs")
