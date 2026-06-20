use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AmmoType {
    Fmj,
    HollowPoint,
    ArmourPiercing,
    Tracer,
}
#[expect(dead_code, reason = "awaiting ammo selection UI")]
impl AmmoType {
    pub fn name(&self) -> &'static str {
        match self {
            AmmoType::Fmj => "FMJ",
            AmmoType::HollowPoint => "HP",
            AmmoType::ArmourPiercing => "AP",
            AmmoType::Tracer => "TRACER",
        }
    }
    pub fn damage_mult(&self) -> f32 {
        match self {
            AmmoType::Fmj => 1.0,
            AmmoType::HollowPoint => 1.25,
            AmmoType::ArmourPiercing => 0.85,
            AmmoType::Tracer => 0.95,
        }
    }
    pub fn penetration_mult(&self) -> f32 {
        match self {
            AmmoType::Fmj => 1.0,
            AmmoType::HollowPoint => 0.5,
            AmmoType::ArmourPiercing => 1.8,
            AmmoType::Tracer => 1.0,
        }
    }
    pub fn spread_mult(&self) -> f32 {
        match self {
            AmmoType::Fmj => 1.0,
            AmmoType::HollowPoint => 1.15,
            AmmoType::ArmourPiercing => 1.05,
            AmmoType::Tracer => 1.0,
        }
    }
}
impl Default for AmmoType {
    fn default() -> Self {
        AmmoType::Fmj
    }
}

#[expect(dead_code, reason = "awaiting ammo selection UI")]
#[derive(Component, Debug, Clone)]
pub struct LoadedAmmo {
    pub ammo_type: AmmoType,
    pub count: u32,
}
