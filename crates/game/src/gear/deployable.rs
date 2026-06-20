/// Deployable equipment systems — placing C4, Claymores, and Breaching Charges.
use avian3d::prelude::*;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, Player};
use socom_input::actions::PlayerAction;

use crate::gear::equipment_inventory::EquipmentInventory;
use crate::gear::equipment_types::{C4Charge, Deployable, EquipmentType};
use crate::messages::EquipmentUsedMessage;

/// Place deployable explosives (C4, Claymore, Breaching Charge).
pub fn deploy_equipment_system(
    mut commands: Commands,
    mut inventory: ResMut<EquipmentInventory>,
    mut equip_writer: MessageWriter<EquipmentUsedMessage>,
    player_query: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<Player>>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, action_state, transform)) = player_query.single() else {
        return;
    };

    if !action_state.just_pressed(&PlayerAction::UseEquipment) {
        return;
    }

    let Some(item) = inventory.selected_item().cloned() else {
        return;
    };

    if !item.equip_type.is_deployable() {
        return;
    }

    if item.quantity == 0 {
        return;
    }

    inventory.consume_selected();

    let forward = *transform.forward();
    let place_distance = 2.5;
    let place_pos = transform.translation + forward * place_distance;

    // Raycast down to find ground surface
    let ray_down = Ray3d::new(place_pos + Vec3::Y * 1.0, Dir3::NEG_Y);
    let ray_dir = ray_down.direction;
    let dir_vec = *ray_dir;
    let hit = spatial_query.cast_ray(
        ray_down.origin,
        ray_dir,
        2.0,
        true,
        &SpatialQueryFilter::default(),
    );
    let final_pos = hit
        .map(|h| ray_down.origin + dir_vec * h.distance)
        .unwrap_or(place_pos);

    equip_writer.write(EquipmentUsedMessage {
        entity: player_entity,
        equip_type: item.equip_type.name().to_string(),
        position: final_pos,
        direction: forward,
    });

    match item.equip_type {
        EquipmentType::C4 => {
            commands.spawn((
                C4Charge {
                    source: player_entity,
                    damage: item.equip_type.base_damage(),
                    radius: item.equip_type.blast_radius(),
                },
                Transform::from_translation(final_pos),
                RigidBody::Static,
                Collider::cuboid(0.15, 0.05, 0.1),
            ));
        }
        EquipmentType::Claymore => {
            commands.spawn((
                Deployable {
                    equip_type: EquipmentType::Claymore,
                    damage: item.equip_type.base_damage(),
                    radius: item.equip_type.blast_radius(),
                    trigger_radius: 3.0,
                    source: player_entity,
                    armed: false,
                    arm_timer: Some(Timer::from_seconds(2.0, TimerMode::Once)),
                },
                Transform::from_translation(final_pos).with_rotation(Quat::from_rotation_y(
                    transform.rotation.to_euler(bevy::math::EulerRot::YXZ).0,
                )),
                RigidBody::Static,
                Collider::cuboid(0.2, 0.1, 0.05),
            ));
        }
        EquipmentType::BreachingCharge => {
            commands.spawn((
                Deployable {
                    equip_type: EquipmentType::BreachingCharge,
                    damage: item.equip_type.base_damage(),
                    radius: item.equip_type.blast_radius(),
                    trigger_radius: 1.0,
                    source: player_entity,
                    armed: true,
                    arm_timer: None,
                },
                Transform::from_translation(final_pos),
                RigidBody::Static,
                Collider::cuboid(0.2, 0.05, 0.2),
            ));
        }
        _ => {}
    }
}

/// Arm timer system — counts down deployable arm timers.
pub fn deployable_arm_system(time: Res<Time>, mut deployable_query: Query<&mut Deployable>) {
    for mut deployable in deployable_query.iter_mut() {
        if let Some(ref mut timer) = deployable.arm_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                deployable.armed = true;
                deployable.arm_timer = None;
            }
        }
    }
}

/// Proximity detonation for claymores — checks living entities within trigger radius and 90° cone.
pub fn claymore_detonation_system(
    mut commands: Commands,
    mut grenade_writer: MessageWriter<crate::messages::GrenadeDetonatedMessage>,
    claymore_query: Query<(Entity, &Deployable, &Transform)>,
    target_query: Query<(&Transform, &Health)>,
) {
    for (entity, deployable, transform) in claymore_query.iter() {
        if !deployable.armed || deployable.equip_type != EquipmentType::Claymore {
            continue;
        }

        let forward = *transform.forward();
        let mut triggered = false;

        for (target_transform, health) in target_query.iter() {
            if !health.is_alive() {
                continue;
            }

            let to_target = target_transform.translation - transform.translation;
            let distance = to_target.length();
            if distance > deployable.trigger_radius {
                continue;
            }

            // Forward-facing 90° cone check
            let to_target_dir = to_target / distance;
            if forward.dot(to_target_dir) < 0.0 {
                continue;
            }

            triggered = true;
            break;
        }

        if triggered {
            let pos = transform.translation + forward * 0.3;
            grenade_writer.write(crate::messages::GrenadeDetonatedMessage {
                position: pos,
                damage: deployable.damage,
                radius: deployable.radius,
                source: deployable.source,
            });
            commands.entity(entity).despawn();
        }
    }
}
