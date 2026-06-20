/// Material penetration table (design.md §13.2) — lookup for
/// (MaterialType × Caliber) → whether the bullet penetrates and the
/// resulting damage multiplier after passing through.
///
/// Uses the 7 calibers defined in weapons::caliber::Caliber.
use bevy::prelude::*;

use crate::combat::damage::DamageMessage;
use crate::combat::destruction::{DestructionState, MaterialType};
use crate::weapons::caliber::Caliber;

/// Result of a penetration lookup.
#[expect(dead_code, reason = "awaiting bullet penetration wiring")]
#[derive(Debug, Clone, Copy)]
pub struct PenetrationResult {
    /// Whether the bullet fully penetrates the material.
    pub penetrates: bool,
    /// Damage multiplier after penetrating (reduced by material resistance).
    pub damage_mult: f32,
}

/// Resource table mapping (MaterialType × Caliber) → PenetrationResult.
///
/// Pre-computed from design.md §13.2 — 11 materials × 7 calibers.
/// The table is deterministic; no runtime mutation needed.
#[derive(Resource, Debug, Clone)]
pub struct MaterialPenetrationTable {
    data: Vec<[PenetrationResult; 7]>, // indexed by material as usize × caliber as usize
}

impl Default for MaterialPenetrationTable {
    fn default() -> Self {
        let mut table = Self {
            data: Vec::with_capacity(12),
        };
        // Must match the order of MaterialType variants.
        let materials: &[(MaterialType, f32)] = &[
            (MaterialType::Drywall, 1.3),
            (MaterialType::Wood, 3.8),
            (MaterialType::Plywood, 1.9),
            (MaterialType::SheetMetal, 0.16),
            (MaterialType::Brick, 10.0),
            (MaterialType::Concrete, 20.0),
            (MaterialType::ReinforcedConcrete, 30.0),
            (MaterialType::Sandbag, 45.0),
            (MaterialType::Glass, 0.3),
            (MaterialType::BulletproofGlass, 3.8),
            (MaterialType::CarDoor, 0.1),
            (MaterialType::CarEngine, 20.0),
            (MaterialType::Flesh, 30.0),
        ];

        for &(mat, thickness_cm) in materials {
            let entry = Self::calc_row(mat, thickness_cm);
            table.data.push(entry);
        }
        table
    }
}

impl MaterialPenetrationTable {
    /// Look up whether the given caliber penetrates the given material.
    pub fn lookup(&self, material: MaterialType, caliber: Caliber) -> PenetrationResult {
        let mat_idx = material as usize;
        if mat_idx >= self.data.len() {
            return PenetrationResult {
                penetrates: false,
                damage_mult: 0.0,
            };
        }
        let cal_idx = caliber_to_index(caliber);
        self.data[mat_idx][cal_idx]
    }

    /// Build a single row of 7 calibers for one material + thickness.
    fn calc_row(material: MaterialType, thickness_cm: f32) -> [PenetrationResult; 7] {
        let calibers = [
            Caliber::NineMm,
            Caliber::FortyFiveACP,
            Caliber::FiveFiveSixNato,
            Caliber::SevenSixTwoX39,
            Caliber::SevenSixTwoNato,
            Caliber::TwelveGauge,
            Caliber::FiftyBMG,
        ];
        let mut row = [PenetrationResult {
            penetrates: false,
            damage_mult: 0.0,
        }; 7];
        for (i, cal) in calibers.iter().enumerate() {
            row[i] = Self::calc_penetration(material, thickness_cm, *cal);
        }
        row
    }

    /// Core penetration calculation based on design.md §13.2 table.
    #[allow(clippy::cyclomatic_complexity)]
    fn calc_penetration(
        material: MaterialType,
        thickness_cm: f32,
        caliber: Caliber,
    ) -> PenetrationResult {
        // ——— Base penetration capability per caliber (effective thickness in cm) ———
        let effective_cm = match caliber {
            Caliber::NineMm => 1.5,
            Caliber::FortyFiveACP => 2.0,
            Caliber::FiveFiveSixNato => 12.0,
            Caliber::SevenSixTwoX39 => 10.0,
            Caliber::SevenSixTwoNato => 15.0,
            Caliber::TwelveGauge => 2.5,
            Caliber::FiftyBMG => 50.0,
        };

        // ——— Material-specific hardness modifier ———
        let hardness = match material {
            MaterialType::Drywall => 0.2,
            MaterialType::Wood => 0.6,
            MaterialType::Plywood => 0.5,
            MaterialType::SheetMetal => 0.7,
            MaterialType::Brick => 1.5,
            MaterialType::Concrete => 2.0,
            MaterialType::ReinforcedConcrete => 3.0,
            MaterialType::Sandbag => 1.8,
            MaterialType::Glass => 0.3,
            MaterialType::BulletproofGlass => 3.5,
            MaterialType::CarDoor => 0.6,
            MaterialType::CarEngine => 5.0,
            MaterialType::Flesh => 0.3,
        };

        let effective_thickness = thickness_cm * hardness;
        let penetrates = effective_cm >= effective_thickness;

        // Damage multiplier after penetration: lost proportional to hardness.
        let damage_mult = if penetrates {
            (1.0 - hardness * 0.15).max(0.05)
        } else {
            // Bullets that stop still transfer a tiny fraction of energy as shock
            0.02
        };

        PenetrationResult {
            penetrates,
            damage_mult,
        }
    }
}

fn caliber_to_index(caliber: Caliber) -> usize {
    match caliber {
        Caliber::NineMm => 0,
        Caliber::FortyFiveACP => 1,
        Caliber::FiveFiveSixNato => 2,
        Caliber::SevenSixTwoX39 => 3,
        Caliber::SevenSixTwoNato => 4,
        Caliber::TwelveGauge => 5,
        Caliber::FiftyBMG => 6,
    }
}

/// System that checks `DamageMessage`s targeting destructible entities.
/// If the bullet caliber penetrates the material, the damage is passed
/// through with a multiplier. This is a stub that integrates with the
/// penetration table; a full implementation would do spatial queries
/// for entities behind the penetrated surface.
pub fn bullet_penetration_system(
    mut damage_reader: bevy::ecs::message::MessageReader<DamageMessage>,
    penetration_table: Res<MaterialPenetrationTable>,
    dest_query: Query<&DestructionState>,
) {
    // Note: In a full implementation, we would need the caliber from the
    // weapon/ammo data that produced the DamageMessage. Currently DamageMessage
    // does not carry caliber info, so this system reads the transition and
    // validates the penetration table is accessible.
    for msg in damage_reader.read() {
        if let Ok(state) = dest_query.get(msg.target) {
            // Look up penetration for reference (caliber info would come from
            // weapon system integration in Phase 3).
            let _ = penetration_table.lookup(state.material, Caliber::FiveFiveSixNato);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drywall_all_pens() {
        let table = MaterialPenetrationTable::default();
        for cal in &[
            Caliber::NineMm,
            Caliber::FortyFiveACP,
            Caliber::FiveFiveSixNato,
            Caliber::SevenSixTwoX39,
            Caliber::SevenSixTwoNato,
            Caliber::TwelveGauge,
            Caliber::FiftyBMG,
        ] {
            let result = table.lookup(MaterialType::Drywall, *cal);
            assert!(result.penetrates, "Drywall should be pen'd by {:?}", cal);
        }
    }

    #[test]
    fn fifty_bmg_pens_everything() {
        let table = MaterialPenetrationTable::default();
        let materials = [
            MaterialType::Drywall,
            MaterialType::Wood,
            MaterialType::Plywood,
            MaterialType::SheetMetal,
            MaterialType::Brick,
            MaterialType::Concrete,
            MaterialType::Sandbag,
            MaterialType::Glass,
            MaterialType::CarDoor,
            MaterialType::Flesh,
        ];
        for mat in &materials {
            let result = table.lookup(*mat, Caliber::FiftyBMG);
            assert!(result.penetrates, ".50 BMG should pen {:?}", mat);
        }
    }

    #[test]
    fn nine_mm_stops_on_brick() {
        let table = MaterialPenetrationTable::default();
        let result = table.lookup(MaterialType::Brick, Caliber::NineMm);
        assert!(!result.penetrates, "9mm should stop on brick");
    }

    #[test]
    fn wood_stops_pistols() {
        let table = MaterialPenetrationTable::default();
        assert!(!table.lookup(MaterialType::Wood, Caliber::NineMm).penetrates);
        assert!(
            !table
                .lookup(MaterialType::Wood, Caliber::FortyFiveACP)
                .penetrates
        );
        assert!(
            table
                .lookup(MaterialType::Wood, Caliber::FiveFiveSixNato)
                .penetrates
        );
    }

    #[test]
    fn reinforced_concrete_stops_all() {
        let table = MaterialPenetrationTable::default();
        for cal in &[
            Caliber::NineMm,
            Caliber::FortyFiveACP,
            Caliber::FiveFiveSixNato,
            Caliber::SevenSixTwoX39,
            Caliber::SevenSixTwoNato,
            Caliber::TwelveGauge,
        ] {
            let result = table.lookup(MaterialType::ReinforcedConcrete, *cal);
            assert!(
                !result.penetrates,
                "{:?} should stop on reinforced concrete",
                cal
            );
        }
    }
}
