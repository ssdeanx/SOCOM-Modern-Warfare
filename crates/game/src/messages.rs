// New game messages (complementing existing DamageMessage + DeathMessage)

use bevy::ecs::message::Message;
use bevy::prelude::*;

// COMBAT - new messages

#[derive(Message, Debug, Clone)]
pub struct WeaponFiredMessage {
    pub shooter: Entity,
    pub weapon_name: String,
    pub position: Vec3,
    pub direction: Vec3,
    pub hit_something: bool,
}

#[derive(Message, Debug, Clone)]
pub struct PlayerDamagedMessage {
    pub amount: f32,
    pub source: Entity,
    pub hit_point: Vec3,
}

#[derive(Message, Debug, Clone)]
pub struct HitConfirmedMessage {
    pub shooter: Entity,
    pub target: Entity,
    pub hit: bool,
    pub hit_point: Vec3,
}

// PROGRESSION

#[derive(Message, Debug, Clone)]
pub struct XpGainedMessage {
    pub amount: u64,
}

#[derive(Message, Debug, Clone)]
pub struct LevelUpMessage {
    pub entity: Entity,
    pub new_level: u32,
}

#[derive(Message, Debug, Clone)]
pub struct AchievementUnlockMessage {
    pub achievement: String,
}

// SQUAD

#[derive(Message, Debug, Clone)]
pub struct SquadStatusMessage {
    pub member: Entity,
    pub alive: bool,
}

// TACTICAL

#[derive(Message, Debug, Clone)]
pub struct CoverStateMessage {
    pub entity: Entity,
    pub in_cover: bool,
}

#[derive(Message, Debug, Clone)]
pub struct SuppressionMessage {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32,
}

// GEAR

#[derive(Message, Debug, Clone)]
pub struct ItemPickupMessage {
    pub entity: Entity,
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Message, Debug, Clone)]
pub struct ItemEquipMessage {
    pub entity: Entity,
    pub item_id: String,
    pub slot: u8,
    pub equip: bool,
}
