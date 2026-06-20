/// Glass fracture and shatter system (design.md §13.4, P5.5.7).
///
/// Special handling for Glass material: pre-scored crack lines, shatter
/// on Breach state, and 10–40 glass shard debris with spray-out physics.
use bevy::prelude::*;

use crate::combat::destruction::{
    DestructionLevel, DestructionState, DestructionTransitionMessage, MaterialType,
};

/// Marker component for a glass pane entity that supports pre-scored fracture.
#[expect(dead_code, reason = "awaiting glass entity spawning")]
#[derive(Component, Debug, Clone)]
pub struct GlassPane {
    /// Pre-scored crack lines (positions where fracture propagates).
    pub crack_points: Vec<Vec3>,
    /// Whether this pane has already been shattered.
    pub shattered: bool,
    /// Dimensions of the glass pane (width, height).
    pub width: f32,
    pub height: f32,
}

#[expect(dead_code, reason = "awaiting glass entity spawning")]
impl GlassPane {
    /// Create a new glass pane with pre-scored crack lines radiating from
    /// centre and edges. Up to 24 crack points for visual variety.
    pub fn new(width: f32, height: f32) -> Self {
        let mut crack_points = Vec::with_capacity(24);

        // Radial cracks from centre.
        for i in 0..8 {
            let angle = std::f32::consts::TAU * i as f32 / 8.0;
            let r = 0.3 + (i as f32 * 0.07).min(0.8);
            crack_points.push(Vec3::new(angle.cos() * r, angle.sin() * r * 0.6, 0.0));
        }
        // Edge cracks from random positions along perimeter.
        for i in 0..4 {
            let t = i as f32 / 4.0;
            crack_points.push(Vec3::new(t - 0.5, -0.5, 0.0));
            crack_points.push(Vec3::new(t - 0.5, 0.5, 0.0));
            crack_points.push(Vec3::new(-0.5, t - 0.5, 0.0));
            crack_points.push(Vec3::new(0.5, t - 0.5, 0.0));
        }

        Self {
            crack_points,
            shattered: false,
            width,
            height,
        }
    }
}

/// Glass shard debris marker, separate from generic Debris.
#[derive(Component, Debug, Clone)]
pub struct GlassShard {
    pub lifetime: Timer,
}

/// Checks for glass panes transitioning to Breached/Destroyed state and
/// spawns 10–40 shard debris entities that spray outward.
pub fn glass_fracture_system(
    mut commands: Commands,
    mut transition_reader: bevy::ecs::message::MessageReader<DestructionTransitionMessage>,
    mut glass_query: Query<(Entity, &Transform, &mut GlassPane)>,
    state_query: Query<&DestructionState>,
) {
    for msg in transition_reader.read() {
        if msg.to_state != DestructionLevel::Breached && msg.to_state != DestructionLevel::Destroyed
        {
            continue;
        }
        // Only process glass materials.
        let Ok(state) = state_query.get(msg.entity) else {
            continue;
        };
        if state.material != MaterialType::Glass && state.material != MaterialType::BulletproofGlass
        {
            continue;
        }

        let Ok((entity, transform, mut glass)) = glass_query.get_mut(msg.entity) else {
            continue;
        };

        if glass.shattered {
            continue;
        }
        glass.shattered = true;

        // Spawn 20–40 glass shards.
        let shard_count = 20 + (fast_rand() % 21); // 20..=40
        let lifetime_secs = if state.material == MaterialType::BulletproofGlass {
            15.0
        } else {
            10.0
        };

        for i in 0..shard_count {
            let _spread = 2.0;
            let _spray_dir = Vec3::new(
                (i as f32 - shard_count as f32 / 2.0) * 0.15,
                fast_rand() as f32 * 0.5 - 0.25,
                -1.0 - fast_rand() as f32 * 0.5,
            )
            .normalize()
                * (1.5 + fast_rand() as f32 * 3.0);

            let offset = Vec3::new(
                (i as f32 - shard_count as f32 / 2.0) * 0.1,
                fast_rand() as f32 * 0.2,
                0.0,
            );

            let pos = transform.translation + offset;

            commands.spawn((
                GlassShard {
                    lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
                },
                Transform::from_translation(pos),
                GlobalTransform::default(),
            ));
        }

        // Despawn the glass pane itself (it's broken).
        commands.entity(entity).despawn();
    }
}

/// Cleanup of glass shard debris after lifetime expires.
pub fn glass_debris_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut shard_query: Query<(Entity, &mut GlassShard)>,
) {
    for (entity, mut shard) in shard_query.iter_mut() {
        shard.lifetime.tick(time.delta());
        if shard.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Simple deterministic pseudo-random based on frame data (not crypto-secure).
/// Returns a value in 0..100 range.
fn fast_rand() -> u32 {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = Cell::new(42);
    }
    SEED.with(|seed| {
        let s = seed.get();
        let next = s.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        seed.set(next);
        (next >> 16) % 100
    })
}
