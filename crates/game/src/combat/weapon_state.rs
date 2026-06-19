use bevy::prelude::*;

use socom_core::components::Weapon;

/// Per-entity weapon runtime state: ammo counts, fire cooldown, reload status.
#[derive(Component, Debug, Clone)]
pub struct WeaponState {
    /// Rounds remaining in the current magazine.
    pub magazine: u32,
    /// Total spare rounds not loaded into a magazine.
    pub reserve: u32,
    /// Timestamp (from `Time::elapsed_secs`) of the last round fired.
    pub last_fire_time: f32,
    /// Whether the weapon is currently being reloaded.
    pub is_reloading: bool,
    /// Remaining reload time in seconds.
    pub reload_timer: f32,
    /// Slot index this state belongs to (0 = primary, 1 = sidearm).
    pub slot_index: u8,
}

impl WeaponState {
    /// Initialise from a `Weapon` config and slot index.
    pub fn from_weapon(weapon: &Weapon, slot_index: u8) -> Self {
        Self {
            magazine: weapon.magazine_size,
            reserve: weapon.reserve_ammo,
            last_fire_time: -f32::MAX,
            is_reloading: false,
            reload_timer: 0.0,
            slot_index,
        }
    }
}

/// Persists the weapon state of the *inactive* weapon slot.
///
/// When the player swaps from primary to sidearm, the active `WeaponState`
/// is moved here (for the old primary) and the other slot's state is restored
/// back into `WeaponState`.  This prevents ammo loss when swapping back.
#[derive(Component, Debug, Clone)]
pub struct OffhandWeaponState(pub WeaponState);
