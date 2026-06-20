use bevy::ecs::message::{Message, MessageReader};
use bevy::prelude::*;

use socom_core::components::Health;

/// Fired when a bullet hits an entity (or geometry that has an entity).
#[derive(Message, Debug, Clone)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
    pub source: Entity,
    pub hit_point: Vec3,
    pub hit_normal: Vec3,
}

/// Reads `DamageMessage`s and subtracts health from the target entity.
pub fn apply_damage_system(
    mut damage_reader: MessageReader<DamageMessage>,
    mut health_query: Query<&mut Health>,
) {
    for msg in damage_reader.read() {
        if let Ok(mut health) = health_query.get_mut(msg.target) {
            health.current = (health.current - msg.amount.max(0.0)).max(0.0);
        }
    }
}

/// Marker for entities killed this frame (prevents double-processing).
#[derive(Component)]
pub struct Dead;

use crate::combat::death::DeathMessage;

/// When an entity's `Health` drops to zero (or below), enters bleed-out
/// on the first hit. If already downed and bleed-out expired, marks `Dead`.
pub fn death_check_system(
    mut commands: Commands,
    mut death_writer: bevy::ecs::message::MessageWriter<DeathMessage>,
    mut query: Query<(Entity, &mut Health), (Without<Dead>, Changed<Health>)>,
) {
    for (entity, mut health) in query.iter_mut() {
        if health.is_alive() {
            continue;
        }
        // First time hitting 0 HP — enter bleed-out instead of dying
        if !health.is_downed {
            health.is_downed = true;
            health.bleed_out_remaining = 30.0;
            continue;
        }
        // Already downed and bleed-out expired — final death
        commands.entity(entity).insert(Dead);
        death_writer.write(DeathMessage {
            entity,
            source: None,
        });
    }
}
