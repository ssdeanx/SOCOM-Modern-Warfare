use bevy::prelude::*;

/// Player specializations that modify gameplay stats.
#[expect(dead_code, reason = "awaiting specialization selection UI")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Specialization {
    Assault,  // +15% damage, +10% sprint speed
    Medic,    // +25% healing received, revive allies faster
    Engineer, // +20% ammo capacity, better gear effectiveness
    Recon,    // +20% accuracy, enhanced detection range
}

#[expect(dead_code, reason = "awaiting specialization selection UI")]
impl Specialization {
    pub fn name(&self) -> &'static str {
        match self {
            Specialization::Assault => "Assault",
            Specialization::Medic => "Medic",
            Specialization::Engineer => "Engineer",
            Specialization::Recon => "Recon",
        }
    }
    pub fn damage_mod(&self) -> f32 {
        match self {
            Specialization::Assault => 1.15,
            _ => 1.0,
        }
    }
    pub fn speed_mod(&self) -> f32 {
        match self {
            Specialization::Assault => 1.1,
            _ => 1.0,
        }
    }
    pub fn accuracy_mod(&self) -> f32 {
        match self {
            Specialization::Recon => 1.2,
            _ => 1.0,
        }
    }
    pub fn ammo_mod(&self) -> f32 {
        match self {
            Specialization::Engineer => 1.2,
            _ => 1.0,
        }
    }
}

/// Resource tracking the player's chosen specialization.
#[expect(dead_code, reason = "awaiting specialization selection UI")]
#[derive(Resource)]
pub struct PlayerSpecialization {
    pub current: Option<Specialization>,
}

impl Default for PlayerSpecialization {
    fn default() -> Self {
        Self { current: None }
    }
}
