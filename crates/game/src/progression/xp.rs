use bevy::prelude::*;

use socom_core::components::{Player, Team};

use crate::combat::death::DeathMessage;
use crate::messages::{LevelUpMessage, XpGainedMessage};

/// Source of XP gain.
#[expect(dead_code, reason = "awaiting XP source differentiation")]
#[derive(Clone, Copy, Debug)]
pub enum XpSource {
    Kill,
    Headshot,
    MissionComplete,
    Objective,
}

/// Tracks the player's level and experience.
#[derive(Resource)]
pub struct PlayerProgression {
    pub xp: u64,
    pub level: u32,
    pub total_xp_earned: u64,
}

impl Default for PlayerProgression {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 1,
            total_xp_earned: 0,
        }
    }
}

impl PlayerProgression {
    /// Add XP. Returns true if leveled up.
    pub fn add_xp(&mut self, amount: u64) -> bool {
        self.xp += amount;
        self.total_xp_earned += amount;
        let xp_needed = self.level as u64 * 100;
        if self.xp >= xp_needed {
            self.xp -= xp_needed;
            self.level += 1;
            true
        } else {
            false
        }
    }
}

/// Listens for kills and awards XP. Fires XpGainedMessage and LevelUpMessage.
pub fn xp_event_listener(
    mut death_reader: bevy::ecs::message::MessageReader<DeathMessage>,
    mut xp_writer: bevy::ecs::message::MessageWriter<XpGainedMessage>,
    mut level_writer: bevy::ecs::message::MessageWriter<LevelUpMessage>,
    mut progression: ResMut<PlayerProgression>,
    _team_query: Query<&Team>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };
    for msg in death_reader.read() {
        // Only award XP if source is the player (player killed something)
        if msg.source != Some(player_entity) {
            continue;
        }
        let xp_amount = 50u64;
        xp_writer.write(XpGainedMessage { amount: xp_amount });
        if progression.add_xp(xp_amount) {
            level_writer.write(LevelUpMessage {
                entity: player_entity,
                new_level: progression.level,
            });
        }
    }
}
