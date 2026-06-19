use bevy::ecs::message::{Message, MessageReader};
use bevy::prelude::*;

use socom_core::components::{Health, Player, Weapon, WeaponSlot};

use crate::combat::weapon_state::{OffhandWeaponState, WeaponState};
use crate::level::{spawn_test_level, LevelEntity};

/// Fired when an entity's health reaches zero.
#[derive(Message, Debug, Clone)]
pub struct DeathMessage {
    pub entity: Entity,
    pub source: Option<Entity>,
}

/// Tracks the player's death-respawn cycle.
#[derive(Resource)]
pub struct RespawnState {
    pub timer: Timer,
    pub is_dead: bool,
}

impl Default for RespawnState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            is_dead: false,
        }
    }
}

/// Marker for the death-screen UI entity.
#[derive(Component)]
pub struct DeathScreenUI;

/// Listens for `DeathMessage`s targeting the player, starts the respawn
/// timer, and shows the death screen overlay.
pub fn handle_player_death(
    mut death_reader: MessageReader<DeathMessage>,
    player_query: Query<Entity, With<Player>>,
    mut respawn: ResMut<RespawnState>,
    mut commands: Commands,
) {
    for msg in death_reader.read() {
        if player_query.get(msg.entity).is_ok() {
            respawn.is_dead = true;
            respawn.timer.reset();
            commands.spawn((
                Text::new("YOU DIED"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.1, 0.1)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(40.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                Transform::from_xyz(-140.0, -32.0, 0.0),
                GlobalTransform::default(),
                DeathScreenUI,
            ));
        }
    }
}

/// After the respawn delay, resets the player to the spawn point with
/// full health and ammo for both weapon slots. Also respawns level entities.
pub fn respawn_system(
    time: Res<Time>,
    mut respawn: ResMut<RespawnState>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut Health,
            &mut WeaponSlot,
            Option<&mut WeaponState>,
            Option<&mut OffhandWeaponState>,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    death_ui_query: Query<Entity, With<DeathScreenUI>>,
    level_entity_query: Query<Entity, With<LevelEntity>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if !respawn.is_dead {
        return;
    }
    respawn.timer.tick(time.delta());
    if !respawn.timer.just_finished() {
        return;
    }

    // Despawn all level entities (enemies, teammate, geometry).
    for entity in &level_entity_query {
        commands.entity(entity).despawn();
    }

    // Re-spawn the level.
    spawn_test_level(commands.reborrow(), meshes, materials);

    // Reset player.
    if let Ok((mut transform, mut health, mut weapon_slot, weapon_state_opt, offhand_opt)) =
        player_query.single_mut()
    {
        *transform = Transform::from_xyz(0.0, 1.5, 0.0);
        health.current = health.max;
        *weapon_slot = WeaponSlot::default();

        if let Some(ref weapon) = weapon_slot.active_weapon().cloned() {
            if let Some(mut ws) = weapon_state_opt {
                *ws = WeaponState::from_weapon(weapon, weapon_slot.active_slot);
            }
        }

        if let Some(mut offhand) = offhand_opt {
            let sidearm = weapon_slot
                .sidearm
                .as_ref()
                .cloned()
                .unwrap_or_else(Weapon::m1911);
            offhand.0 = WeaponState::from_weapon(&sidearm, 1);
        }
    }

    respawn.is_dead = false;

    for entity in death_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
