use bevy::prelude::*;
use socom_core::components::{Health, Player};

#[derive(Component)]
pub struct DamageVignette;

const VIGNETTE_FADE_TIME: f32 = 0.5;

pub fn damage_vignette_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    vignette_query: Query<Entity, With<DamageVignette>>,
    mut bg_query: Query<&mut BackgroundColor, With<DamageVignette>>,
    mut state: Local<f32>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    if let Ok(health) = player_query.single() {
        *state = (1.0 - health.ratio()) * 0.6;
    }
    *state = (*state - dt / VIGNETTE_FADE_TIME).max(0.0);
    if *state > 0.01 && vignette_query.is_empty() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.6, 0.0, 0.0, 0.0)),
            DamageVignette,
        ));
    } else if *state <= 0.01 {
        for entity in &vignette_query {
            commands.entity(entity).despawn();
        }
    } else if let Ok(mut bg) = bg_query.single_mut() {
        bg.0.set_alpha(*state);
    }
}
