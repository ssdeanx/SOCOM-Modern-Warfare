use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnderbarrelType {
    None,
    VerticalGrip,
    AngledGrip,
    Bipod,
}

impl UnderbarrelType {
    pub fn name(&self) -> &'static str {
        match self {
            UnderbarrelType::None => "None",
            UnderbarrelType::VerticalGrip => "Vertical Grip",
            UnderbarrelType::AngledGrip => "Angled Grip",
            UnderbarrelType::Bipod => "Bipod",
        }
    }
    pub fn vertical_recoil_mult(&self) -> f32 {
        match self {
            UnderbarrelType::None => 1.0,
            UnderbarrelType::VerticalGrip => 0.8,
            UnderbarrelType::AngledGrip => 0.9,
            UnderbarrelType::Bipod => 0.4,
        }
    }
    pub fn horizontal_recoil_mult(&self) -> f32 {
        match self {
            UnderbarrelType::None => 1.0,
            UnderbarrelType::VerticalGrip => 0.95,
            UnderbarrelType::AngledGrip => 0.75,
            UnderbarrelType::Bipod => 0.6,
        }
    }
    pub fn hip_spread_mult(&self) -> f32 {
        match self {
            UnderbarrelType::None => 1.0,
            UnderbarrelType::VerticalGrip => 0.9,
            UnderbarrelType::AngledGrip => 0.85,
            UnderbarrelType::Bipod => 0.7,
        }
    }
    pub fn ads_speed_mult(&self) -> f32 {
        match self {
            UnderbarrelType::None => 1.0,
            UnderbarrelType::VerticalGrip => 0.9,
            UnderbarrelType::AngledGrip => 0.95,
            UnderbarrelType::Bipod => 0.7,
        }
    }
    pub fn weight_add(&self) -> f32 {
        match self {
            UnderbarrelType::None => 0.0,
            UnderbarrelType::VerticalGrip => 0.3,
            UnderbarrelType::AngledGrip => 0.25,
            UnderbarrelType::Bipod => 0.8,
        }
    }
}

impl Default for UnderbarrelType {
    fn default() -> Self {
        UnderbarrelType::None
    }
}
