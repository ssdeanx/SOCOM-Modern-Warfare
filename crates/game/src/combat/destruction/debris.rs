/// Debris spawning and lifecycle management (design.md §13.4).
///
/// Each material type produces different debris types with varying
/// count, lifetime, and physics behaviour.
use bevy::prelude::*;

use crate::combat::destruction::{DestructionLevel, MaterialType};

/// Marker component for a spawned debris entity.
#[derive(Component, Debug, Clone)]
pub struct Debris {
    #[expect(dead_code, reason = "awaiting material-specific debris")]
    pub material: MaterialType,
    pub lifetime: Timer,
}

/// Spawn debris entities when a structure transitions to Breached or Destroyed.
///
/// Called from `destruction_state_machine_system` in damage.rs. One-shot
/// per entity to avoid debris duplication.
pub fn spawn_debris_for_transition(
    commands: &mut Commands,
    material: MaterialType,
    state: DestructionLevel,
    position: Vec3,
) {
    let (debris_count, lifetime_secs) = debris_params(material, state);

    for i in 0..debris_count {
        let offset = Vec3::new(
            (i as f32 - debris_count as f32 / 2.0) * 0.3,
            0.5,
            (i as f32 % 3.0 - 1.0) * 0.3,
        );
        let pos = position + offset;

        commands.spawn((
            Debris {
                material,
                lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
            },
            Transform::from_translation(pos),
            GlobalTransform::default(),
        ));
    }
}

/// Returns (count, lifetime_seconds) for the given material and state.
fn debris_params(material: MaterialType, state: DestructionLevel) -> (u32, f32) {
    let is_destroyed = state == DestructionLevel::Destroyed;
    match material {
        MaterialType::Concrete | MaterialType::ReinforcedConcrete => {
            if is_destroyed {
                (20, 30.0)
            } else {
                (5, 30.0)
            }
        }
        MaterialType::Brick => {
            if is_destroyed {
                (30, 30.0)
            } else {
                (10, 30.0)
            }
        }
        MaterialType::Wood | MaterialType::Plywood => {
            if is_destroyed {
                (15, 20.0)
            } else {
                (5, 20.0)
            }
        }
        MaterialType::SheetMetal | MaterialType::CarDoor => {
            if is_destroyed {
                (10, 60.0)
            } else {
                (3, 60.0)
            }
        }
        MaterialType::Glass => {
            // Glass handled by glass_fracture_system, but fallback:
            if is_destroyed {
                (20, 10.0)
            } else {
                (5, 10.0)
            }
        }
        MaterialType::Drywall => {
            if is_destroyed {
                (15, 15.0)
            } else {
                (5, 15.0)
            }
        }
        MaterialType::Sandbag => {
            if is_destroyed {
                (10, 20.0)
            } else {
                (3, 20.0)
            }
        }
        MaterialType::BulletproofGlass => {
            if is_destroyed {
                (10, 15.0)
            } else {
                (3, 15.0)
            }
        }
        MaterialType::CarEngine => {
            if is_destroyed {
                (5, 60.0)
            } else {
                (2, 60.0)
            }
        }
        MaterialType::Flesh => (0, 0.0), // No debris from flesh
    }
}

/// Timer-based cleanup for debris entities. Runs every frame; debris
/// is despawned after its lifetime expires.
pub fn debris_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut debris_query: Query<(Entity, &mut Debris)>,
) {
    for (entity, mut debris) in debris_query.iter_mut() {
        debris.lifetime.tick(time.delta());
        if debris.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
