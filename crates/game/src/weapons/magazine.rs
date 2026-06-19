use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MagazineType {
    Standard, Extended, QuickDraw, Drum,
}

impl MagazineType {
    pub fn name(&self) -> &'static str {
        match self { MagazineType::Standard => "Standard", MagazineType::Extended => "Extended", MagazineType::QuickDraw => "Quick-Draw", MagazineType::Drum => "Drum" }
    }
    pub fn capacity_mult(&self) -> f32 {
        match self { MagazineType::Standard => 1.0, MagazineType::Extended => 1.5, MagazineType::QuickDraw => 1.0, MagazineType::Drum => 3.0 }
    }
    pub fn reload_mult(&self) -> f32 {
        match self { MagazineType::Standard => 1.0, MagazineType::Extended => 1.2, MagazineType::QuickDraw => 0.7, MagazineType::Drum => 1.5 }
    }
    pub fn weight_add(&self) -> f32 {
        match self { MagazineType::Standard => 0.0, MagazineType::Extended => 0.3, MagazineType::QuickDraw => 0.1, MagazineType::Drum => 1.0 }
    }
    pub fn speed_mult(&self) -> f32 {
        match self { MagazineType::Standard => 1.0, MagazineType::Extended => 0.98, MagazineType::QuickDraw => 1.0, MagazineType::Drum => 0.92 }
    }
}

impl Default for MagazineType { fn default() -> Self { MagazineType::Standard } }
