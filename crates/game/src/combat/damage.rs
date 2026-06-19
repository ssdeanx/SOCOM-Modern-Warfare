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

/// When an entity's `Health` drops to zero (or below), marks it with
/// `Dead` and emits a `DeathMessage`.
pub fn death_check_system(
    mut commands: Commands,
    mut death_writer: bevy::ecs::message::MessageWriter<DeathMessage>,
    query: Query<(Entity, &Health), (Without<Dead>, Changed<Health>)>,
) {
    for (entity, health) in query.iter() {
        if health.is_alive() {
            continue;
        }
        commands.entity(entity).insert(Dead);
        death_writer.write(DeathMessage {
            entity,
            source: None,
        });
    }
}
