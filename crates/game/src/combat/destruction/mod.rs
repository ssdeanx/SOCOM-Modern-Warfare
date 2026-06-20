/// Destruction System (P5.5) — Enterprise-grade material penetration, debris,
/// glass fracture, building collapse, and vehicle damage states.
///
/// Architecture:
///   - Message-driven: DestructionTransitionMessage emitted on state changes
///   - Systems in Update schedule, chained via `.chain()`
///   - Per-material penetration lookup via MaterialPenetrationTable resource
///   - Debris entities with Timed lifetime cleanup
///   - Vehicle damage handled as a specialized state machine overlay
pub mod damage;
pub mod debris;
pub mod glass;
pub mod penetration;
pub mod vehicles;

use bevy::prelude::*;

/// Re-export key types for ergonomic access from other modules.
#[allow(unused_imports)]
pub use penetration::{MaterialPenetrationTable, PenetrationResult};

/// Fired whenever a destructible entity transitions between destruction levels.
#[derive(Message, Debug, Clone)]
pub struct DestructionTransitionMessage {
    pub entity: Entity,
    pub from_state: DestructionLevel,
    pub to_state: DestructionLevel,
    pub position: Vec3,
}

/// Top-level destruction level for any destructible entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DestructionLevel {
    #[default]
    Pristine,
    Damaged,
    Breached,
    Destroyed,
}

/// Material classification used for destruction, penetration, and debris spawning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Drywall,
    Wood,
    Plywood,
    SheetMetal,
    Brick,
    Concrete,
    ReinforcedConcrete,
    Sandbag,
    Glass,
    BulletproofGlass,
    CarDoor,
    CarEngine,
    Flesh,
}

/// Per-entity destruction state machine component.
///
/// Tracks structural health, material type, accumulated bullet holes,
/// and whether debris was already spawned to avoid duplication.
#[derive(Component, Debug, Clone)]
pub struct DestructionState {
    pub state: DestructionLevel,
    pub health: f32,
    pub max_health: f32,
    pub material: MaterialType,
    pub bullet_holes: Vec<Vec3>,
    pub debris_spawned: bool,
}

impl DestructionState {
    /// Create a new DestructionState for the given material with appropriate defaults.
    pub fn for_material(material: MaterialType) -> Self {
        let max_health = Self::default_max_health(material);
        Self {
            state: DestructionLevel::Pristine,
            health: max_health,
            max_health,
            material,
            bullet_holes: Vec::with_capacity(8),
            debris_spawned: false,
        }
    }

    /// Default max structural HP per material type (from design.md §13.3).
    fn default_max_health(material: MaterialType) -> f32 {
        match material {
            MaterialType::Drywall => 200.0,
            MaterialType::Wood => 100.0,
            MaterialType::Plywood => 80.0,
            MaterialType::SheetMetal => 300.0,
            MaterialType::Brick => 1500.0,
            MaterialType::Concrete => 2000.0,
            MaterialType::ReinforcedConcrete => 5000.0,
            MaterialType::Sandbag => 500.0,
            MaterialType::Glass => 30.0,
            MaterialType::BulletproofGlass => 200.0,
            MaterialType::CarDoor => 200.0,
            MaterialType::CarEngine => 500.0,
            MaterialType::Flesh => 100.0,
        }
    }

    /// Health ratio (0.0 – 1.0).
    pub fn ratio(&self) -> f32 {
        if self.max_health > 0.0 {
            (self.health / self.max_health).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Check whether this entity is fully destroyed.
    #[expect(dead_code, reason = "awaiting gameplay loop integration")]
    pub fn is_destroyed(&self) -> bool {
        self.state == DestructionLevel::Destroyed
    }
}

/// Default gives a basic drywall wall.
impl Default for DestructionState {
    fn default() -> Self {
        Self::for_material(MaterialType::Drywall)
    }
}

/// Plugin that registers all destruction-related types and systems.
pub struct DestructionPlugin;

impl Plugin for DestructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DestructionTransitionMessage>()
            .init_resource::<MaterialPenetrationTable>()
            .add_systems(
                Update,
                (
                    penetration::bullet_penetration_system,
                    damage::apply_explosion_damage_system,
                    damage::destruction_state_machine_system,
                    debris::debris_lifetime_system,
                    glass::glass_fracture_system,
                    glass::glass_debris_lifetime_system,
                    vehicles::vehicle_damage_state_system,
                    damage::collapse_animation_system,
                )
                    .chain(),
            );
    }
}
