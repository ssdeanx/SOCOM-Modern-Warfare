/// Healing systems — medkits, bandages, bleed-out, and revive mechanics.
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, Player};
use socom_input::actions::PlayerAction;

use crate::combat::damage::Downed;
use crate::gear::equipment_inventory::EquipmentInventory;
use crate::gear::equipment_types::{EquipmentItem, EquipmentType};
use crate::messages::EquipmentUsedMessage;

/// Self-heal when UseEquipment is pressed with a healing item selected.
pub fn self_heal_system(
    mut commands: Commands,
    mut inventory: ResMut<EquipmentInventory>,
    mut equip_writer: MessageWriter<EquipmentUsedMessage>,
    mut player_query: Query<
        (Entity, &ActionState<PlayerAction>, &mut Health),
        With<Player>,
    >,
) {
    let Ok((entity, action_state, mut health)) = player_query.single_mut() else {
        return;
    };

    if !action_state.just_pressed(&PlayerAction::UseEquipment) {
        return;
    }

    // Can't heal while downed (needs revive)
    if health.is_downed {
        return;
    }

    let Some(item) = inventory.selected_item().cloned() else {
        return;
    };

    let heal_amount = match item.equip_type {
        EquipmentType::FieldBandage => 25.0,
        EquipmentType::MedicalKit => 75.0,
        EquipmentType::Splint => 40.0,
        EquipmentType::EnergyDrink => 15.0,
        _ => return,
    };

    if health.current >= health.max {
        return; // Full HP, don't waste item
    }

    inventory.consume_selected();
    health.current = (health.current + heal_amount).min(health.max);

    equip_writer.write(EquipmentUsedMessage {
        entity,
        equip_type: item.equip_type.name().to_string(),
        position: Vec3::ZERO,
        direction: Vec3::ZERO,
    });
}

/// Ticks bleed-out timer for downed entities; clears downed state when expired.
pub fn bleed_out_system(
    time: Res<Time>,
    mut query: Query<&mut Health>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for mut health in query.iter_mut() {
        if !health.is_downed {
            continue;
        }
        health.bleed_out_remaining -= dt;
        if health.bleed_out_remaining <= 0.0 {
            health.is_downed = false; // Triggers Changed<Health> → death_check
        }
    }
}

/// Revive a downed teammate when interacting (E) with a MedicalKit/FieldBandage selected.
pub fn revive_system(
    mut commands: Commands,
    mut inventory: ResMut<EquipmentInventory>,
    mut equip_writer: MessageWriter<EquipmentUsedMessage>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut downed_query: Query<(Entity, &mut Health, &Transform)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    let Some(item) = inventory.selected_item().cloned() else {
        return;
    };

    let revive_amount = match item.equip_type {
        EquipmentType::MedicalKit => 50.0,
        EquipmentType::FieldBandage => 25.0,
        _ => return,
    };

    let revive_name = item.equip_type.name().to_string();

    for (_entity, mut health, downed_transform) in downed_query.iter_mut() {
        if !health.is_downed {
            continue;
        }
        let dist = (downed_transform.translation - player_transform.translation).length();
        if dist > 2.0 {
            continue;
        }

        // Revive
        health.is_downed = false;
        health.current = revive_amount;
        health.bleed_out_remaining = 30.0;
        inventory.consume_selected();
        commands.entity(_entity).remove::<Downed>();

        equip_writer.write(EquipmentUsedMessage {
            entity: player_entity,
            equip_type: revive_name,
            position: downed_transform.translation,
            direction: Vec3::ZERO,
        });
        break;
    }
}
