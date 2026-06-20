use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use socom_core::components::{Player, Team};

use crate::combat::damage::DamageMessage;
use crate::combat::death::DeathMessage;

/// Tracks comprehensive player statistics.
#[derive(Resource)]
pub struct PlayerStats {
    pub kills: u32,
    pub deaths: u32,
    pub shots_fired: u32,
    pub shots_hit: u32,
    #[expect(dead_code, reason = "awaiting stat tracking")]
    pub headshots: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
    #[expect(dead_code, reason = "awaiting stat tracking")]
    pub missions_completed: u32,
    pub accuracy: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            kills: 0,
            deaths: 0,
            shots_fired: 0,
            shots_hit: 0,
            headshots: 0,
            damage_dealt: 0.0,
            damage_taken: 0.0,
            missions_completed: 0,
            accuracy: 0.0,
        }
    }
}

/// Tracks shots fired and hits from damage events.
pub fn damage_event_listener(
    mut damage_reader: MessageReader<DamageMessage>,
    mut stats: ResMut<PlayerStats>,
    player_query: Query<Entity, With<Player>>,
    team_query: Query<&Team>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    for msg in damage_reader.read() {
        if msg.source == player_entity {
            stats.shots_fired += 1;
            // If we hit an enemy, count as hit
            if let Ok(team) = team_query.get(msg.target) {
                if *team == Team::Enemy || *team == Team::Teammate {
                    stats.shots_hit += 1;
                    stats.damage_dealt += msg.amount;
                }
            }
        }
        if msg.target == player_entity {
            stats.damage_taken += msg.amount;
        }
    }
    stats.accuracy = if stats.shots_fired > 0 {
        stats.shots_hit as f32 / stats.shots_fired as f32
    } else {
        0.0
    };
}

/// Tracks kills and deaths.
pub fn death_event_listener(
    mut death_reader: MessageReader<DeathMessage>,
    mut stats: ResMut<PlayerStats>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    for msg in death_reader.read() {
        if msg.source == Some(player_entity) {
            stats.kills += 1;
        }
        if msg.entity == player_entity {
            stats.deaths += 1;
        }
    }
}
