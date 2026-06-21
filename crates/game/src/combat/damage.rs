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

/// Marker for entities in the bleed-out / downed state.
/// Used to reliably track downed entities even when `Changed<Health>`
/// does not fire (e.g. damage at already-zero health is clamped to 0).
#[derive(Component)]
pub struct Downed;

use crate::combat::death::DeathMessage;

/// When an entity's `Health` drops to zero (or below), enters bleed-out
/// on the first hit. If already downed and bleed-out expired, marks `Dead`.
///
/// Uses two queries:
/// - `fresh_query` detects newly-downed entities via `Changed<Health>`.
/// - `downed_query` checks all currently-downed entities every frame so that
///   finishing damage at 0 HP (which doesn't change `Health`) still leads to
///   death, and bleed-out expiry is properly detected.
pub fn death_check_system(
    mut commands: Commands,
    mut death_writer: bevy::ecs::message::MessageWriter<DeathMessage>,
    // Freshly dead — health just dropped to 0
    mut fresh_query: Query<
        (Entity, &mut Health),
        (Without<Dead>, Without<Downed>, Changed<Health>),
    >,
    // Already downed — check bleed-out expiry every frame
    downed_query: Query<(Entity, &Health), (With<Downed>, Without<Dead>)>,
) {
    // --- First pass: newly-dead entities enter bleed-out ---
    for (entity, mut health) in fresh_query.iter_mut() {
        if health.is_alive() {
            continue;
        }
        // First time hitting 0 HP — enter bleed-out instead of dying
        health.is_downed = true;
        health.bleed_out_remaining = 30.0;
        commands.entity(entity).insert(Downed);
    }

    // --- Second pass: downed entities — check for bleed-out expiry ---
    for (entity, health) in downed_query.iter() {
        if health.bleed_out_remaining > 0.0 {
            continue;
        }
        // Bleed-out expired — final death
        commands.entity(entity).insert(Dead);
        commands.entity(entity).remove::<Downed>();
        death_writer.write(DeathMessage {
            entity,
            source: None,
        });
    }
}
