/// Equipment inventory — holds 5 equipment slots for throwable/deployable/melee items.
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_input::actions::PlayerAction;

use crate::gear::equipment_types::{EquipmentItem, EquipmentType};

/// Equipment inventory slot index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Slot1 = 0,
    Slot2 = 1,
    Slot3 = 2,
    Slot4 = 3,
    Slot5 = 4,
}

impl EquipmentSlot {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

/// Player's carried equipment — up to 5 stacks of throwable/deployable/melee items.
#[derive(Resource)]
pub struct EquipmentInventory {
    pub slots: [Option<EquipmentItem>; 5],
    pub selected_slot: EquipmentSlot,
}

impl Default for EquipmentInventory {
    fn default() -> Self {
        Self {
            slots: [
                Some(EquipmentItem {
                    equip_type: EquipmentType::Knife,
                    quantity: 1,
                }),
                Some(EquipmentItem {
                    equip_type: EquipmentType::FragGrenade,
                    quantity: 4,
                }),
                Some(EquipmentItem {
                    equip_type: EquipmentType::Flashbang,
                    quantity: 2,
                }),
                Some(EquipmentItem {
                    equip_type: EquipmentType::SmokeGrenade,
                    quantity: 2,
                }),
                None,
            ],
            selected_slot: EquipmentSlot::Slot1,
        }
    }
}

impl EquipmentInventory {
    /// Returns the currently selected equipment item, if any.
    pub fn selected_item(&self) -> Option<&EquipmentItem> {
        self.slots[self.selected_slot.index()].as_ref()
    }

    /// Decrements quantity of the selected slot, removing if empty.
    pub fn consume_selected(&mut self) {
        if let Some(ref mut item) = self.slots[self.selected_slot.index()] {
            item.quantity = item.quantity.saturating_sub(1);
            if item.quantity == 0 {
                self.slots[self.selected_slot.index()] = None;
            }
        }
    }

    /// Cycle to the next occupied slot (wraps around).
    pub fn cycle_next(&mut self) {
        let start = self.selected_slot.index();
        for i in 1..=5 {
            let idx = (start + i) % 5;
            if self.slots[idx].is_some() {
                self.selected_slot = match idx {
                    0 => EquipmentSlot::Slot1,
                    1 => EquipmentSlot::Slot2,
                    2 => EquipmentSlot::Slot3,
                    3 => EquipmentSlot::Slot4,
                    _ => EquipmentSlot::Slot5,
                };
                return;
            }
        }
    }
}

/// Cycles equipment selection when CycleEquipment input is pressed.
pub fn select_equipment_system(
    mut inventory: ResMut<EquipmentInventory>,
    query: Query<&ActionState<PlayerAction>, With<socom_core::components::Player>>,
) {
    let Ok(action_state) = query.single() else {
        return;
    };
    if action_state.just_pressed(&PlayerAction::CycleEquipment) {
        inventory.cycle_next();
    }
}
