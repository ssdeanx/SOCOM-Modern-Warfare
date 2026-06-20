pub mod attachments;
pub mod deployable;
pub mod equipment_inventory;
pub mod equipment_types;
pub mod healing;
pub mod inventory;
pub mod items;
pub mod melee;
pub mod throwable;
pub mod workshop;

use bevy::prelude::*;

use socom_core::components::{Player, WeaponSlot};

use crate::combat::WeaponState;

pub struct GearPlugin;

impl Plugin for GearPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<inventory::PlayerInventory>();
        app.init_resource::<workshop::WeaponWorkshop>();
        app.init_resource::<equipment_inventory::EquipmentInventory>();
        app.add_systems(
            Update,
            (
                inventory::track_damage_for_loot,
                workshop::weapon_modification_system,
                equipment_inventory::select_equipment_system,
                throwable::throw_equipment_system,
                throwable::fuse_timer_system,
                throwable::c4_detonation_system,
                deployable::deploy_equipment_system,
                deployable::deployable_arm_system,
                deployable::claymore_detonation_system,
                melee::melee_attack_system,
                healing::self_heal_system,
                healing::bleed_out_system,
                healing::revive_system,
                apply_workshop_to_weapon_system,
            ),
        );
    }
}

/// System that reads WeaponWorkshop fitted attachments and applies their
/// modifiers to the player's active weapon stats.
pub fn apply_workshop_to_weapon_system(
    workshop: Res<workshop::WeaponWorkshop>,
    mut player_query: Query<(&mut WeaponSlot, Option<&mut WeaponState>), With<Player>>,
) {
    if !workshop.ui_open && !workshop.is_changed() {
        return;
    }
    let Ok((mut weapon_slot, _weapon_state_opt)) = player_query.single_mut() else {
        return;
    };
    if let Some(ref mut weapon) = weapon_slot.active_weapon_mut() {
        let (dmg_mod, spread_mod, mag_mod) = workshop.apply_modifiers(
            weapon.damage,
            weapon.spread_degrees,
            weapon.magazine_size as f32,
        );
        weapon.damage = dmg_mod;
        weapon.spread_degrees = spread_mod;
        weapon.magazine_size = (mag_mod.round() as u32).max(1);
    }
}
