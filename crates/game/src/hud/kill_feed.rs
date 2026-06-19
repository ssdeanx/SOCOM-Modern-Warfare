use crate::combat::death::DeathMessage;
use bevy::prelude::*;
use socom_core::components::{Player, Team};

#[derive(Component)]
pub struct KillFeedEntry;

pub fn kill_feed_system(
    mut death_reader: bevy::ecs::message::MessageReader<DeathMessage>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    team_query: Query<&Team>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    for msg in death_reader.read() {
        let text = if msg.source == Some(player_entity) {
            let name = team_query
                .get(msg.entity)
                .map(|t| match t {
                    Team::Enemy => "Enemy",
                    _ => "Target",
                })
                .unwrap_or("Unknown");
            format!("Killed {}", name)
        } else if msg.entity == player_entity {
            "You died".to_string()
        } else {
            continue;
        };
        commands.spawn((
            Text::new(text),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            KillFeedEntry,
        ));
    }
}

pub fn kill_feed_cleanup_system(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<Entity, With<KillFeedEntry>>,
    mut timers: Local<Vec<(Entity, Timer)>>,
) {
    let mut i = 0;
    while i < timers.len() {
        let (entity, ref mut timer) = timers[i];
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn();
            timers.swap_remove(i);
        } else {
            i += 1;
        }
    }
    for entity in &query {
        if !timers.iter().any(|(e, _)| *e == entity) {
            timers.push((entity, Timer::from_seconds(5.0, TimerMode::Once)));
        }
    }
}
