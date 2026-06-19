use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BarrelType {
    Standard, Suppressor, Compensator, Extended, Short,
}

impl BarrelType {
    pub fn name(&self) -> &'static str {
        match self { BarrelType::Standard => "Standard", BarrelType::Suppressor => "Suppressor", BarrelType::Compensator => "Compensator", BarrelType::Extended => "Extended", BarrelType::Short => "Short" }
    }
    pub fn damage_mult(&self) -> f32 {
        match self { BarrelType::Standard => 1.0, BarrelType::Suppressor => 0.9, BarrelType::Compensator => 1.0, BarrelType::Extended => 1.08, BarrelType::Short => 0.92 }
    }
    pub fn recoil_mult(&self) -> f32 {
        match self { BarrelType::Standard => 1.0, BarrelType::Suppressor => 1.0, BarrelType::Compensator => 0.75, BarrelType::Extended => 0.9, BarrelType::Short => 1.15 }
    }
    pub fn spread_mult(&self) -> f32 {
        match self { BarrelType::Standard => 1.0, BarrelType::Suppressor => 0.95, BarrelType::Compensator => 0.85, BarrelType::Extended => 0.8, BarrelType::Short => 1.2 }
    }
    pub fn range_mult(&self) -> f32 {
        match self { BarrelType::Standard => 1.0, BarrelType::Suppressor => 1.0, BarrelType::Compensator => 1.0, BarrelType::Extended => 1.2, BarrelType::Short => 0.75 }
    }
    pub fn weight_add(&self) -> f32 {
        match self { BarrelType::Standard => 0.0, BarrelType::Suppressor => 0.4, BarrelType::Compensator => 0.3, BarrelType::Extended => 0.5, BarrelType::Short => -0.2 }
    }
}

impl Default for BarrelType { fn default() -> Self { BarrelType::Standard } }
