use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::Component;

/// Marker component for the player entity
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player;

/// Movement direction in world space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Velocity(pub glam::Vec3);

/// Current movement stance
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MovementState {
    #[default]
    Standing,
    Sprinting,
    Crouching,
    Prone,
    InCover,
}

/// Health pool with armor, bleed-out, and revive support.
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    /// Armor points that absorb a portion of incoming damage.
    pub armor: f32,
    /// Maximum armor capacity.
    pub max_armor: f32,
    /// Whether this entity is in the bleed-out / downed state.
    pub is_downed: bool,
    /// Seconds remaining in bleed-out before final death.
    pub bleed_out_remaining: f32,
    /// Progress towards being revived (0.0 = not revived).
    pub revive_progress: f32,
}

impl Health {
    pub const fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            armor: 0.0,
            max_armor: 100.0,
            is_downed: false,
            bleed_out_remaining: 30.0,
            revive_progress: 0.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn is_down(&self) -> bool {
        self.is_downed
    }

    pub fn ratio(&self) -> f32 {
        self.current / self.max
    }

    pub fn armor_ratio(&self) -> f32 {
        if self.max_armor > 0.0 {
            self.armor / self.max_armor
        } else {
            0.0
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100.0)
    }
}

/// Team affiliation
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Team {
    Player,
    Teammate,
    Enemy,
}

/// Full weapon configuration data.
///
/// Defines all static properties of a weapon: damage, fire rate, magazine
/// capacity, reload speed, automatic vs semi-auto, and accuracy spread.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    /// Human-readable weapon name (e.g. "M4A1", "1911").
    pub name: String,
    /// Rounds per second (semi-auto weapons cap via fire rate too).
    pub fire_rate: f32,
    /// Base damage per bullet; modified by distance and hit location in Phase 2.
    pub damage: f32,
    /// Rounds per magazine before reload is required.
    pub magazine_size: u32,
    /// Total spare rounds carried (not loaded into a magazine).
    pub reserve_ammo: u32,
    /// Time in seconds to complete a reload.
    pub reload_time: f32,
    /// `true` = hold to fire full-auto; `false` = one shot per press.
    pub is_automatic: bool,
    /// Base weapon spread in degrees (bullet deviation from crosshair centre).
    pub spread_degrees: f32,
    /// Maximum effective range in metres.
    pub max_range: f32,
}

impl Weapon {
    /// Create an M4A1 assault rifle (player primary).
    pub fn m4a1() -> Self {
        Self {
            name: "M4A1".into(),
            fire_rate: 10.0,
            damage: 25.0,
            magazine_size: 30,
            reserve_ammo: 120,
            reload_time: 2.1,
            is_automatic: true,
            spread_degrees: 0.5,
            max_range: 300.0,
        }
    }

    /// Create an MP5SD submachine gun.
    pub fn mp5sd() -> Self {
        Self {
            name: "MP5SD".into(),
            fire_rate: 12.0,
            damage: 18.0,
            magazine_size: 30,
            reserve_ammo: 90,
            reload_time: 2.5,
            is_automatic: true,
            spread_degrees: 1.0,
            max_range: 150.0,
        }
    }

    /// Create an M1911 pistol (player sidearm).
    pub fn m1911() -> Self {
        Self {
            name: "M1911".into(),
            fire_rate: 5.0,
            damage: 35.0,
            magazine_size: 7,
            reserve_ammo: 28,
            reload_time: 1.5,
            is_automatic: false,
            spread_degrees: 0.8,
            max_range: 50.0,
        }
    }

    /// Create an AK-47 assault rifle (enemy weapon).
    pub fn ak47() -> Self {
        Self {
            name: "AK-47".into(),
            fire_rate: 8.0,
            damage: 30.0,
            magazine_size: 30,
            reserve_ammo: 90,
            reload_time: 2.5,
            is_automatic: true,
            spread_degrees: 1.5,
            max_range: 350.0,
        }
    }
}

/// Weapon slots for a character.
///
/// Defaults to M4A1 (primary) + M1911 (sidearm) for the player.
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponSlot {
    pub primary: Option<Weapon>,
    pub sidearm: Option<Weapon>,
    /// Index of the currently active slot: 0 = primary, 1 = sidearm.
    pub active_slot: u8,
}

impl Default for WeaponSlot {
    fn default() -> Self {
        Self {
            primary: Some(Weapon::m4a1()),
            sidearm: Some(Weapon::m1911()),
            active_slot: 0,
        }
    }
}

impl WeaponSlot {
    /// Returns a reference to the currently equipped weapon, if any.
    pub fn active_weapon(&self) -> Option<&Weapon> {
        match self.active_slot {
            0 => self.primary.as_ref(),
            1 => self.sidearm.as_ref(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the currently equipped weapon, if any.
    pub fn active_weapon_mut(&mut self) -> Option<&mut Weapon> {
        match self.active_slot {
            0 => self.primary.as_mut(),
            1 => self.sidearm.as_mut(),
            _ => None,
        }
    }

    /// Swap between primary (0) and sidearm (1).
    pub fn swap_slot(&mut self) {
        self.active_slot = if self.active_slot == 0 { 1 } else { 0 };
    }
}

/// Camera shoulder preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Shoulder {
    #[default]
    Right,
    Left,
}
