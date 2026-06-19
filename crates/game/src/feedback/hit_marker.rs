use crate::combat::damage::DamageMessage;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use socom_core::components::{Player, Team};

#[derive(Component)]
pub struct HitMarker;
const HIT_MARKER_DURATION: f32 = 0.15;

#[derive(Resource, Default)]
pub(crate) struct HitMarkerState {
    timer: Timer,
}

pub fn hit_marker_detect_system(
    mut damage_reader: MessageReader<DamageMessage>,
    mut commands: Commands,
    hit_marker_query: Query<Entity, With<HitMarker>>,
    player_query: Query<Entity, (With<Player>, Without<Team>)>,
    enemy_query: Query<&Team>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    for msg in damage_reader.read() {
        if msg.source != player_entity {
            continue;
        }
        let is_enemy = enemy_query.get(msg.target).is_ok_and(|t| *t == Team::Enemy);
        if !is_enemy {
            continue;
        }
        if hit_marker_query.is_empty() {
            commands.spawn((
                Text::new("X"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                Transform::from_xyz(-10.0, -14.0, 0.0),
                GlobalTransform::default(),
                HitMarker,
            ));
        }
    }
}

pub fn hit_marker_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    hit_marker_query: Query<Entity, With<HitMarker>>,
    mut state: Local<Option<HitMarkerState>>,
) {
    let state = state.get_or_insert_with(HitMarkerState::default);
    if hit_marker_query.is_empty() {
        state.timer = Timer::from_seconds(HIT_MARKER_DURATION, TimerMode::Once);
        return;
    }
    state.timer.tick(time.delta());
    if state.timer.just_finished() {
        for entity in &hit_marker_query {
            commands.entity(entity).despawn();
        }
    }
}
