use crate::combat::damage::DamageMessage;
use crate::gear::items::{GearItem, GearSlot};
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerInventory {
    pub equipped: [Option<GearItem>; 5],
    pub stash: Vec<GearItem>,
    pub credits: u64,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            equipped: [None, None, None, None, None],
            stash: Vec::new(),
            credits: 0,
        }
    }
}

impl PlayerInventory {
    #[expect(dead_code, reason = "awaiting inventory management UI")]
    pub fn equip(&mut self, item: GearItem) -> Option<GearItem> {
        let idx = match item.slot {
            GearSlot::PrimaryWeapon => 0,
            GearSlot::SidearmWeapon => 1,
            GearSlot::Helmet => 2,
            GearSlot::BodyArmor => 3,
            GearSlot::TacticalGear => 4,
        };
        let previous = self.equipped[idx].take();
        self.equipped[idx] = Some(item);
        previous
    }
    #[expect(dead_code, reason = "awaiting inventory management UI")]
    pub fn add_item(&mut self, item: GearItem) {
        self.stash.push(item);
    }
    pub fn weapon_damage_bonus(&self) -> f32 {
        let mut bonus = 0.0;
        for i in 0..2 {
            if let Some(ref item) = self.equipped[i] {
                bonus += item.stats.get("damage").copied().unwrap_or(0.0);
            }
        }
        bonus
    }
}

pub fn track_damage_for_loot(mut damage_reader: MessageReader<DamageMessage>, _commands: Commands) {
    for _msg in damage_reader.read() {}
}
