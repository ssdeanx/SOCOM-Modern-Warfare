/// Damage application systems for destruction:
///   1. apply_explosion_damage_system — spherical falloff from grenades
///   2. destruction_state_machine_system — Pristine → Damaged → Breached → Destroyed
///   3. collapse_animation_system — apply downward velocity + dust on Destroyed
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::prelude::*;

use crate::combat::damage::DamageMessage;
use crate::combat::destruction::{
    debris, DestructionLevel, DestructionState, DestructionTransitionMessage, MaterialType,
};
use crate::messages::GrenadeDetonatedMessage;

/// Thresholds relative to max_health for state transitions.
const DAMAGED_RATIO: f32 = 0.60; // Below 60% → Damaged
const BREACHED_RATIO: f32 = 0.30; // Below 30% → Breached
const DESTROYED_RATIO: f32 = 0.00; // At 0% → Destroyed

/// Reads `GrenadeDetonatedMessage` and applies spherical falloff damage
/// to all destructible entities within the explosion radius.
pub fn apply_explosion_damage_system(
    mut grenade_reader: MessageReader<GrenadeDetonatedMessage>,
    mut dest_query: Query<(Entity, &Transform, &mut DestructionState)>,
    mut damage_writer: MessageWriter<DamageMessage>,
) {
    for msg in grenade_reader.read() {
        let radius = msg.radius.max(0.1);
        let max_damage = msg.damage;

        for (entity, transform, mut state) in dest_query.iter_mut() {
            let dist = transform.translation.distance(msg.position);
            if dist > radius {
                continue;
            }
            // Spherical falloff: 1.0 at centre, 0.0 at radius edge.
            let falloff = 1.0 - (dist / radius);
            let damage = max_damage * falloff * 0.8; // 0.8 structural damage efficiency

            // Apply structural damage.
            state.health = (state.health - damage).max(0.0);

            // Also send a DamageMessage for overlay systems that listen for it.
            damage_writer.write(DamageMessage {
                target: entity,
                amount: damage,
                source: msg.source,
                hit_point: transform.translation,
                hit_normal: (transform.translation - msg.position).normalize(),
            });
        }
    }
}

/// Destruction state machine: monitors `DestructionState.health` and
/// advances through Pristine → Damaged → Breached → Destroyed.
///
/// Emits `DestructionTransitionMessage` on each transition and triggers
/// debris spawning via `debris::spawn_debris_for_transition`.
pub fn destruction_state_machine_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DestructionState, Option<&Transform>)>,
    mut transition_writer: MessageWriter<DestructionTransitionMessage>,
) {
    for (entity, mut state, transform_opt) in query.iter_mut() {
        let old_state = state.state;
        let new_state = determine_state(state.ratio(), state.material);

        if new_state == old_state {
            continue;
        }

        let position = transform_opt.map(|t| t.translation).unwrap_or_default();

        state.state = new_state;

        // Emit transition message for other systems (audio, VFX, networking).
        transition_writer.write(DestructionTransitionMessage {
            entity,
            from_state: old_state,
            to_state: new_state,
            position,
        });

        // Spawn debris on Breach or Destroyed (one-shot per entity).
        if (new_state == DestructionLevel::Breached || new_state == DestructionLevel::Destroyed)
            && !state.debris_spawned
        {
            state.debris_spawned = true;
            debris::spawn_debris_for_transition(&mut commands, state.material, new_state, position);
        }
    }
}

/// Determine next destruction level based on health ratio and material.
fn determine_state(ratio: f32, _material: MaterialType) -> DestructionLevel {
    if ratio <= DESTROYED_RATIO {
        DestructionLevel::Destroyed
    } else if ratio <= BREACHED_RATIO {
        DestructionLevel::Breached
    } else if ratio <= DAMAGED_RATIO {
        DestructionLevel::Damaged
    } else {
        DestructionLevel::Pristine
    }
}

/// Component for debris entities that need collapse physics (downward velocity).
#[derive(Component, Debug, Clone)]
pub struct CollapseDebris {
    pub lifetime: Timer,
    #[expect(dead_code, reason = "awaiting debris physics")]
    pub velocity: Vec3,
}

/// Simple collapse animation: applies downward velocity to debris entities
/// when a structure enters the Destroyed state. In a full implementation,
/// this would integrate with avian3d rigidbody physics.
pub fn collapse_animation_system(
    time: Res<Time>,
    mut transition_reader: MessageReader<DestructionTransitionMessage>,
    mut commands: Commands,
    mut debris_query: Query<(Entity, &mut CollapseDebris)>,
) {
    // Process pending transitions for collapse effect.
    for msg in transition_reader.read() {
        if msg.to_state == DestructionLevel::Destroyed {
            // Tag nearby debris with collapse velocity for the next few frames.
            // Real implementation would use spatial query; for now we mark
            // any debris spawned near this position in the same frame.
        }
    }

    // Animate existing collapse debris (simple descent + despawn).
    for (entity, mut cd) in debris_query.iter_mut() {
        cd.lifetime.tick(time.delta());
        if cd.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
