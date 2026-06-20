/// Throwable equipment systems — grenade throwing, fuse timing, and detonation.
use avian3d::prelude::*;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::Player;
use socom_input::actions::PlayerAction;

use crate::gear::equipment_inventory::EquipmentInventory;
use crate::gear::equipment_types::{EquipmentType, GrenadeProjectile};
use crate::messages::{EquipmentUsedMessage, GrenadeDetonatedMessage};

/// Player throws the selected throwable equipment.
pub fn throw_equipment_system(
    mut commands: Commands,
    mut inventory: ResMut<EquipmentInventory>,
    mut equip_writer: MessageWriter<EquipmentUsedMessage>,
    player_query: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<Player>>,
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

    if !item.equip_type.is_throwable() {
        return;
    }

    // Consume one from inventory
    inventory.consume_selected();

    let forward = *transform.forward();
    let spawn_pos = transform.translation + forward * 2.0 + Vec3::Y * 1.5;

    equip_writer.write(EquipmentUsedMessage {
        entity: player_entity,
        equip_type: item.equip_type.name().to_string(),
        position: spawn_pos,
        direction: forward,
    });

    match item.equip_type {
        EquipmentType::FragGrenade => {
            commands.spawn((
                GrenadeProjectile {
                    fuse_timer: Timer::from_seconds(item.equip_type.fuse_time(), TimerMode::Once),
                    equip_type: EquipmentType::FragGrenade,
                    damage: item.equip_type.base_damage(),
                    radius: item.equip_type.blast_radius(),
                    source: player_entity,
                },
                Transform::from_translation(spawn_pos).with_rotation(Quat::from_rotation_x(0.3)),
                RigidBody::Dynamic,
                Collider::sphere(0.1),
                LinearVelocity(forward * 12.0 + Vec3::Y * 4.0),
                GravityScale(1.0),
                Sensor,
            ));
        }
        EquipmentType::Flashbang => {
            commands.spawn((
                GrenadeProjectile {
                    fuse_timer: Timer::from_seconds(4.0, TimerMode::Once),
                    equip_type: EquipmentType::Flashbang,
                    damage: 0.0,
                    radius: 12.0,
                    source: player_entity,
                },
                Transform::from_translation(spawn_pos),
                RigidBody::Dynamic,
                Collider::sphere(0.1),
                LinearVelocity(forward * 14.0 + Vec3::Y * 3.0),
                GravityScale(1.0),
                Sensor,
            ));
        }
        EquipmentType::SmokeGrenade => {
            commands.spawn((
                GrenadeProjectile {
                    fuse_timer: Timer::from_seconds(1.5, TimerMode::Once),
                    equip_type: EquipmentType::SmokeGrenade,
                    damage: 0.0,
                    radius: 8.0,
                    source: player_entity,
                },
                Transform::from_translation(spawn_pos),
                RigidBody::Dynamic,
                Collider::sphere(0.12),
                LinearVelocity(forward * 8.0 + Vec3::Y * 2.0),
                GravityScale(1.0),
                Sensor,
            ));
        }
        EquipmentType::TearGas => {
            commands.spawn((
                GrenadeProjectile {
                    fuse_timer: Timer::from_seconds(3.0, TimerMode::Once),
                    equip_type: EquipmentType::TearGas,
                    damage: 5.0,
                    radius: 8.0,
                    source: player_entity,
                },
                Transform::from_translation(spawn_pos),
                RigidBody::Dynamic,
                Collider::sphere(0.1),
                LinearVelocity(forward * 10.0 + Vec3::Y * 3.5),
                GravityScale(1.0),
                Sensor,
            ));
        }
        EquipmentType::Flare => {
            commands.spawn((
                GrenadeProjectile {
                    fuse_timer: Timer::from_seconds(0.5, TimerMode::Once),
                    equip_type: EquipmentType::Flare,
                    damage: 0.0,
                    radius: 5.0,
                    source: player_entity,
                },
                Transform::from_translation(spawn_pos),
                RigidBody::Dynamic,
                Collider::sphere(0.08),
                LinearVelocity(forward * 15.0 + Vec3::Y * 5.0),
                GravityScale(0.5),
                Sensor,
            ));
        }
        _ => {}
    }
}

/// Tick fuse timers on grenade projectiles; detonate when fuse expires.
pub fn fuse_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut grenade_query: Query<(Entity, &mut GrenadeProjectile, &Transform)>,
    mut grenade_writer: MessageWriter<GrenadeDetonatedMessage>,
) {
    for (entity, mut grenade, transform) in grenade_query.iter_mut() {
        grenade.fuse_timer.tick(time.delta());
        if grenade.fuse_timer.just_finished() {
            let pos = transform.translation;
            grenade_writer.write(GrenadeDetonatedMessage {
                position: pos,
                damage: grenade.damage,
                radius: grenade.radius,
                source: grenade.source,
            });

            // AOE damage to entities in radius requires spatial query — deferred to later pass
            commands.entity(entity).despawn();
        }
    }
}

/// Detonate C4 charges when the player presses UseEquipment while looking at one.
pub fn c4_detonation_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut c4_query: Query<(Entity, &crate::gear::equipment_types::C4Charge)>,
    mut grenade_writer: MessageWriter<GrenadeDetonatedMessage>,
    player_query: Query<&Transform, With<Player>>,
) {
    // Remote detonate C4 with Backspace (temporary key)
    if !input.just_pressed(KeyCode::Backspace) {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (entity, c4) in c4_query.iter_mut() {
        let pos = player_transform.translation;
        grenade_writer.write(GrenadeDetonatedMessage {
            position: pos,
            damage: c4.damage,
            radius: c4.radius,
            source: c4.source,
        });
        // Despawn the C4 charge
        commands.entity(entity).despawn();
    }
}
