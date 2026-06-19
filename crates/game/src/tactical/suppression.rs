use bevy::prelude::*;

use socom_core::components::{Health, Player};

use crate::messages::SuppressionMessage;

/// Tracks suppression level per entity (0.0 = calm, 100.0 = fully suppressed).
#[derive(Component, Debug)]
pub struct Suppression {
    pub level: f32,
    pub decay_timer: Timer,
}

impl Default for Suppression {
    fn default() -> Self {
        Self {
            level: 0.0,
            decay_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

/// Applies suppression when the player takes damage. Emits SuppressionMessage.
pub fn suppression_system(
    time: Res<Time>,
    player_query: Query<(Entity, &Health), (With<Player>, Changed<Health>)>,
    mut suppression_query: Query<&mut Suppression, With<Player>>,
    mut sup_writer: bevy::ecs::message::MessageWriter<SuppressionMessage>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok((player_entity, _health)) = player_query.single() else {
        return;
    };
    if let Ok(mut suppression) = suppression_query.single_mut() {
        let old_level = suppression.level;
        suppression.level = (suppression.level + 20.0).min(100.0);
        suppression.decay_timer.reset();
        if (suppression.level - old_level).abs() > 5.0 {
            sup_writer.write(SuppressionMessage {
                target: player_entity,
                source: player_entity,
                amount: suppression.level,
            });
        }
    }
}

/// Decays suppression over time.
pub fn suppression_fx_system(
    time: Res<Time>,
    mut suppression_query: Query<&mut Suppression, With<Player>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    if let Ok(mut suppression) = suppression_query.single_mut() {
        suppression.decay_timer.tick(time.delta());
        if suppression.decay_timer.just_finished() {
            suppression.level = (suppression.level - 30.0 * dt).max(0.0);
        }
    }
}
