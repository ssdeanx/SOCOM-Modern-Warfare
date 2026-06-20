use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StockType {
    Standard,
    Folding,
    Precision,
    NoStock,
}

impl StockType {
    pub fn name(&self) -> &'static str {
        match self {
            StockType::Standard => "Standard",
            StockType::Folding => "Folding",
            StockType::Precision => "Precision",
            StockType::NoStock => "No Stock",
        }
    }
    pub fn recoil_mult(&self) -> f32 {
        match self {
            StockType::Standard => 1.0,
            StockType::Folding => 1.1,
            StockType::Precision => 0.8,
            StockType::NoStock => 1.35,
        }
    }
    pub fn ads_speed_mult(&self) -> f32 {
        match self {
            StockType::Standard => 1.0,
            StockType::Folding => 1.15,
            StockType::Precision => 0.85,
            StockType::NoStock => 1.25,
        }
    }
    pub fn hip_spread_mult(&self) -> f32 {
        match self {
            StockType::Standard => 1.0,
            StockType::Folding => 1.05,
            StockType::Precision => 0.9,
            StockType::NoStock => 1.3,
        }
    }
    pub fn sway_mult(&self) -> f32 {
        match self {
            StockType::Standard => 1.0,
            StockType::Folding => 1.1,
            StockType::Precision => 0.7,
            StockType::NoStock => 1.5,
        }
    }
    pub fn weight_add(&self) -> f32 {
        match self {
            StockType::Standard => 0.0,
            StockType::Folding => -0.3,
            StockType::Precision => 0.4,
            StockType::NoStock => -0.5,
        }
    }
}

impl Default for StockType {
    fn default() -> Self {
        StockType::Standard
    }
}
