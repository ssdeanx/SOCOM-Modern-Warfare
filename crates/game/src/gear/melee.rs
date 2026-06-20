/// Melee attack system — knife lunges and close-quarters strikes.
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, Player};
use socom_input::actions::PlayerAction;

use crate::combat::damage::DamageMessage;
use crate::messages::MeleeHitMessage;

const MELEE_RANGE: f32 = 2.0;
const KNIFE_DAMAGE: f32 = 100.0;

/// Performs a melee attack (knife lunge) when the Melee action is pressed.
/// Hits the closest entity with a Health component within range in front of the player.
pub fn melee_attack_system(
    mut damage_writer: MessageWriter<DamageMessage>,
    mut melee_writer: MessageWriter<MeleeHitMessage>,
    player_query: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<Player>>,
    target_query: Query<(Entity, &Transform, &Health), Without<Player>>,
) {
    let Ok((player_entity, action_state, transform)) = player_query.single() else {
        return;
    };

    if !action_state.just_pressed(&PlayerAction::Melee) {
        return;
    }

    let player_pos = transform.translation;
    let forward = *transform.forward();

    let mut closest_entity: Option<Entity> = None;
    let mut closest_distance = MELEE_RANGE;

    for (target_entity, target_transform, health) in target_query.iter() {
        if !health.is_alive() {
            continue;
        }

        let to_target = target_transform.translation - player_pos;
        let distance = to_target.length();

        if distance > MELEE_RANGE {
            continue;
        }

        // Check if enemy is in front of the player (within ~60° cone)
        let to_target_dir = to_target / distance;
        let dot = forward.dot(to_target_dir);
        if dot < 0.5 {
            continue;
        }

        if distance < closest_distance {
            closest_distance = distance;
            closest_entity = Some(target_entity);
        }
    }

    if let Some(target) = closest_entity {
        damage_writer.write(DamageMessage {
            target,
            amount: KNIFE_DAMAGE,
            source: player_entity,
            hit_point: player_pos + forward * closest_distance,
            hit_normal: forward,
        });

        melee_writer.write(MeleeHitMessage {
            attacker: player_entity,
            target,
            damage: KNIFE_DAMAGE,
        });
    }
}
