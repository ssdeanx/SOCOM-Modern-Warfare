use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common, Uncommon, Rare, Epic, Legendary,
}

impl Rarity {
    pub fn name(&self) -> &'static str {
        match self { Rarity::Common => "Common", Rarity::Uncommon => "Uncommon", Rarity::Rare => "Rare", Rarity::Epic => "Epic", Rarity::Legendary => "Legendary" }
    }
    pub fn color(&self) -> (f32, f32, f32) {
        match self { Rarity::Common => (0.7,0.7,0.7), Rarity::Uncommon => (0.2,0.8,0.2), Rarity::Rare => (0.2,0.5,1.0), Rarity::Epic => (0.7,0.2,1.0), Rarity::Legendary => (1.0,0.7,0.1) }
    }
    pub fn stat_multiplier(&self) -> f32 {
        match self { Rarity::Common => 1.0, Rarity::Uncommon => 1.1, Rarity::Rare => 1.25, Rarity::Epic => 1.4, Rarity::Legendary => 1.6 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GearSlot {
    PrimaryWeapon, SidearmWeapon, Helmet, BodyArmor, TacticalGear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearItem {
    pub id: String,
    pub name: String,
    pub slot: GearSlot,
    pub rarity: Rarity,
    pub level_required: u32,
    pub stats: HashMap<String, f32>,
    pub description: String,
    pub icon_path: String,
}
