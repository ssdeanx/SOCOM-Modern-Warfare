/// Caliber definitions with damage, penetration, and ballistic characteristics.
/// Each caliber modifies a weapon's base stats multiplicatively.
use serde::{Deserialize, Serialize};

/// Available calibers in the game, from light pistol to heavy anti-materiel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Caliber {
    NineMm,
    FortyFiveACP,
    FiveFiveSixNato,
    SevenSixTwoX39,
    SevenSixTwoNato,
    TwelveGauge,
    FiftyBMG,
}

impl Caliber {
    pub fn name(&self) -> &'static str {
        match self {
            Caliber::NineMm => "9x19mm",
            Caliber::FortyFiveACP => ".45 ACP",
            Caliber::FiveFiveSixNato => "5.56x45mm",
            Caliber::SevenSixTwoX39 => "7.62x39mm",
            Caliber::SevenSixTwoNato => "7.62x51mm",
            Caliber::TwelveGauge => "12 Gauge",
            Caliber::FiftyBMG => ".50 BMG",
        }
    }

    pub fn damage_mult(&self) -> f32 {
        match self {
            Caliber::NineMm => 0.8,
            Caliber::FortyFiveACP => 1.1,
            Caliber::FiveFiveSixNato => 1.0,
            Caliber::SevenSixTwoX39 => 1.15,
            Caliber::SevenSixTwoNato => 1.3,
            Caliber::TwelveGauge => 1.4,
            Caliber::FiftyBMG => 2.0,
        }
    }

    pub fn penetration_mult(&self) -> f32 {
        match self {
            Caliber::NineMm => 0.6,
            Caliber::FortyFiveACP => 0.8,
            Caliber::FiveFiveSixNato => 1.0,
            Caliber::SevenSixTwoX39 => 1.1,
            Caliber::SevenSixTwoNato => 1.3,
            Caliber::TwelveGauge => 0.5,
            Caliber::FiftyBMG => 2.0,
        }
    }

    pub fn velocity_mult(&self) -> f32 {
        match self {
            Caliber::NineMm => 0.7,
            Caliber::FortyFiveACP => 0.65,
            Caliber::FiveFiveSixNato => 1.0,
            Caliber::SevenSixTwoX39 => 0.9,
            Caliber::SevenSixTwoNato => 1.1,
            Caliber::TwelveGauge => 0.5,
            Caliber::FiftyBMG => 1.3,
        }
    }

    pub fn recoil_mult(&self) -> f32 {
        match self {
            Caliber::NineMm => 0.7,
            Caliber::FortyFiveACP => 1.0,
            Caliber::FiveFiveSixNato => 1.0,
            Caliber::SevenSixTwoX39 => 1.2,
            Caliber::SevenSixTwoNato => 1.4,
            Caliber::TwelveGauge => 1.5,
            Caliber::FiftyBMG => 2.5,
        }
    }

    pub fn range_mult(&self) -> f32 {
        match self {
            Caliber::NineMm => 0.5,
            Caliber::FortyFiveACP => 0.4,
            Caliber::FiveFiveSixNato => 1.0,
            Caliber::SevenSixTwoX39 => 0.9,
            Caliber::SevenSixTwoNato => 1.2,
            Caliber::TwelveGauge => 0.3,
            Caliber::FiftyBMG => 2.0,
        }
    }
}

impl Default for Caliber {
    fn default() -> Self {
        Caliber::FiveFiveSixNato
    }
}
