/// Equipment type definitions for throwable, deployable, and melee items.
use bevy::prelude::*;

/// All equipment items a player can carry and use.
#[expect(dead_code, reason = "awaiting loadout/workshop UI")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentType {
    FragGrenade,
    Flashbang,
    SmokeGrenade,
    TearGas,
    C4,
    Claymore,
    BreachingCharge,
    MedicalKit,
    FieldBandage,
    Splint,
    EnergyDrink,
    Knife,
    Binoculars,
    Flare,
}

impl EquipmentType {
    pub fn name(&self) -> &'static str {
        match self {
            EquipmentType::FragGrenade => "Frag Grenade",
            EquipmentType::Flashbang => "Flashbang",
            EquipmentType::SmokeGrenade => "Smoke Grenade",
            EquipmentType::TearGas => "Tear Gas",
            EquipmentType::C4 => "C4 Explosive",
            EquipmentType::Claymore => "Claymore",
            EquipmentType::BreachingCharge => "Breaching Charge",
            EquipmentType::MedicalKit => "Medical Kit",
            EquipmentType::FieldBandage => "Field Bandage",
            EquipmentType::Splint => "Splint",
            EquipmentType::EnergyDrink => "Energy Drink",
            EquipmentType::Knife => "Knife",
            EquipmentType::Binoculars => "Binoculars",
            EquipmentType::Flare => "Flare",
        }
    }

    /// Weight in kg for this equipment item.
    #[expect(dead_code, reason = "awaiting loadout/workshop UI")]
    pub fn weight(&self) -> f32 {
        match self {
            EquipmentType::FragGrenade => 0.4,
            EquipmentType::Flashbang => 0.3,
            EquipmentType::SmokeGrenade => 0.5,
            EquipmentType::TearGas => 0.4,
            EquipmentType::C4 => 1.0,
            EquipmentType::Claymore => 1.5,
            EquipmentType::BreachingCharge => 2.0,
            EquipmentType::MedicalKit => 0.5,
            EquipmentType::FieldBandage => 0.15,
            EquipmentType::Splint => 0.3,
            EquipmentType::EnergyDrink => 0.3,
            EquipmentType::Knife => 0.2,
            EquipmentType::Binoculars => 0.5,
            EquipmentType::Flare => 0.2,
        }
    }

    /// Damage dealt when this equipment hits/explodes (0 = no damage).
    pub fn base_damage(&self) -> f32 {
        match self {
            EquipmentType::FragGrenade => 200.0,
            EquipmentType::Flashbang => 0.0,
            EquipmentType::SmokeGrenade => 0.0,
            EquipmentType::TearGas => 5.0,
            EquipmentType::C4 => 400.0,
            EquipmentType::Claymore => 300.0,
            EquipmentType::BreachingCharge => 500.0,
            EquipmentType::MedicalKit => 0.0,
            EquipmentType::FieldBandage => 0.0,
            EquipmentType::Splint => 0.0,
            EquipmentType::EnergyDrink => 0.0,
            EquipmentType::Knife => 100.0,
            EquipmentType::Binoculars => 0.0,
            EquipmentType::Flare => 0.0,
        }
    }

    /// Explosion/blast radius in meters (0 = no area damage).
    pub fn blast_radius(&self) -> f32 {
        match self {
            EquipmentType::FragGrenade => 6.0,
            EquipmentType::C4 => 10.0,
            EquipmentType::Claymore => 8.0,
            EquipmentType::BreachingCharge => 4.0,
            _ => 0.0,
        }
    }

    /// Fuse time in seconds (0 = instant/on-demand).
    pub fn fuse_time(&self) -> f32 {
        match self {
            EquipmentType::FragGrenade => 3.5,
            EquipmentType::C4 => 0.0, // Remote detonated
            _ => 0.0,
        }
    }

    /// Max stack size per inventory slot.
    #[expect(dead_code, reason = "awaiting loadout/workshop UI")]
    pub fn max_stack(&self) -> u32 {
        match self {
            EquipmentType::FragGrenade => 4,
            EquipmentType::Flashbang => 4,
            EquipmentType::SmokeGrenade => 3,
            EquipmentType::TearGas => 3,
            EquipmentType::C4 => 2,
            EquipmentType::Claymore => 2,
            EquipmentType::BreachingCharge => 2,
            EquipmentType::MedicalKit => 3,
            EquipmentType::FieldBandage => 5,
            EquipmentType::Splint => 3,
            EquipmentType::EnergyDrink => 3,
            EquipmentType::Knife => 1,
            EquipmentType::Binoculars => 1,
            EquipmentType::Flare => 5,
        }
    }

    /// Whether this item is a throwable explosive.
    pub fn is_throwable(&self) -> bool {
        matches!(
            self,
            EquipmentType::FragGrenade
                | EquipmentType::Flashbang
                | EquipmentType::SmokeGrenade
                | EquipmentType::TearGas
                | EquipmentType::Flare
        )
    }

    /// Whether this item is a deployable explosive.
    pub fn is_deployable(&self) -> bool {
        matches!(
            self,
            EquipmentType::C4 | EquipmentType::Claymore | EquipmentType::BreachingCharge
        )
    }
}

/// One equipment stack carried in the player's equipment inventory.
#[derive(Debug, Clone, Component)]
pub struct EquipmentItem {
    pub equip_type: EquipmentType,
    pub quantity: u32,
}

/// Spawned when a throwable is thrown — applies physics and fuse logic.
#[derive(Debug, Clone, Component)]
pub struct GrenadeProjectile {
    pub fuse_timer: Timer,
    #[expect(dead_code, reason = "awaiting loadout/workshop UI")]
    pub equip_type: EquipmentType,
    pub damage: f32,
    pub radius: f32,
    pub source: Entity,
}

/// Spawned when a deployable is placed.
#[derive(Debug, Clone, Component)]
pub struct Deployable {
    pub equip_type: EquipmentType,
    pub damage: f32,
    pub radius: f32,
    pub trigger_radius: f32,
    pub source: Entity,
    pub armed: bool,
    pub arm_timer: Option<Timer>,
}

/// Spawned projectiles that are C4 bricks (adhesive, remote detonated).
#[derive(Debug, Clone, Component)]
pub struct C4Charge {
    pub source: Entity,
    pub damage: f32,
    pub radius: f32,
}
