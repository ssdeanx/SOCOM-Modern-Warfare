/// Sight attachments that affect ADS speed, zoom, and accuracy.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SightType {
    Iron,
    RedDot,
    Holo,
    Acog,
    SniperScope,
}

impl SightType {
    pub fn name(&self) -> &'static str {
        match self {
            SightType::Iron => "Iron Sights",
            SightType::RedDot => "Red Dot Sight",
            SightType::Holo => "Holo",
            SightType::Acog => "ACOG",
            SightType::SniperScope => "Sniper Scope",
        }
    }
    pub fn ads_spread_mult(&self) -> f32 {
        match self {
            SightType::Iron => 1.0,
            SightType::RedDot => 0.85,
            SightType::Holo => 0.8,
            SightType::Acog => 0.7,
            SightType::SniperScope => 0.5,
        }
    }
    pub fn ads_time_mult(&self) -> f32 {
        match self {
            SightType::Iron => 1.0,
            SightType::RedDot => 1.05,
            SightType::Holo => 1.1,
            SightType::Acog => 1.3,
            SightType::SniperScope => 1.6,
        }
    }
    pub fn hip_spread_mult(&self) -> f32 {
        match self {
            SightType::Iron => 1.0,
            SightType::RedDot => 1.0,
            SightType::Holo => 1.0,
            SightType::Acog => 1.1,
            SightType::SniperScope => 1.3,
        }
    }
    pub fn weight_add(&self) -> f32 {
        match self {
            SightType::Iron => 0.0,
            SightType::RedDot => 0.2,
            SightType::Holo => 0.3,
            SightType::Acog => 0.6,
            SightType::SniperScope => 1.0,
        }
    }
    pub fn zoom_factor(&self) -> f32 {
        match self {
            SightType::Iron => 1.0,
            SightType::RedDot => 1.0,
            SightType::Holo => 1.1,
            SightType::Acog => 2.5,
            SightType::SniperScope => 6.0,
        }
    }
}
impl Default for SightType {
    fn default() -> Self {
        SightType::Iron
    }
}
