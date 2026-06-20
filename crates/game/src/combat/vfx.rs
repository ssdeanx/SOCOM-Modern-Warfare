/// Enterprise-grade VFX system using bevy_hanabi 0.18.0.
///
/// Provides 5 distinct particle effects:
/// 1. MuzzleFlash — yellow/orange burst at weapon muzzle (0.05s)
/// 2. BulletImpact — orange/white sparks along surface normal
/// 3. HitMarker — red flash on damage target (0.08s)
/// 4. DeathExplosion — 6-particle burst outward with gravity (1s)
/// 5. TracerRound — glowing sphere trail (0.3s)
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::combat::damage::DamageMessage;
use crate::combat::death::DeathMessage;
use crate::messages::HitConfirmedMessage;

// ═══════════════════════════════════════════════════════════════════════════════
// VFX COMPONENTS — Timed cleanup markers
// ═══════════════════════════════════════════════════════════════════════════════

/// Marker for a one-shot VFX effect that auto-despawns after its timer expires.
#[derive(Component)]
pub struct VfxEffect {
    pub timer: Timer,
}

/// Marker for a tracer projectile.
#[derive(Component)]
pub struct TracerProjectile {
    pub lifetime: Timer,
}

// ═══════════════════════════════════════════════════════════════════════════════
// EFFECT ASSETS — Initialised once in the plugin
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct VfxAssets {
    pub muzzle_flash: Handle<EffectAsset>,
    pub bullet_impact: Handle<EffectAsset>,
    pub hit_marker: Handle<EffectAsset>,
    pub death_explosion: Handle<EffectAsset>,
    pub tracer: Handle<EffectAsset>,
}

/// Build all VFX effect assets. Called once during plugin setup.
pub fn build_vfx_assets(assets: &mut Assets<EffectAsset>) -> VfxAssets {
    VfxAssets {
        muzzle_flash: assets.add(build_muzzle_flash_effect()),
        bullet_impact: assets.add(build_bullet_impact_effect()),
        hit_marker: assets.add(build_hit_marker_effect()),
        death_explosion: assets.add(build_death_explosion_effect()),
        tracer: assets.add(build_tracer_effect()),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EFFECT BUILDER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Muzzle flash: a fast burst of yellow-orange particles that expand outward.
fn build_muzzle_flash_effect() -> EffectAsset {
    let mut module = Module::default();
    let lifetime = module.lit(0.1);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut colour_gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colour_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0)); // Bright yellow
    colour_gradient.add_key(0.3, Vec4::new(1.0, 0.6, 0.1, 1.0)); // Orange
    colour_gradient.add_key(1.0, Vec4::new(0.8, 0.2, 0.0, 0.0)); // Transparent red

    let mut size_gradient: bevy_hanabi::Gradient<Vec3> = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.3));
    size_gradient.add_key(0.5, Vec3::splat(0.6));
    size_gradient.add_key(1.0, Vec3::splat(0.05));

    EffectAsset::new(64, SpawnerSettings::once((64.0).into()), module)
        .with_name("muzzle_flash")
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient: colour_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

/// Bullet impact sparks: white/orange particles along hit normal direction.
fn build_bullet_impact_effect() -> EffectAsset {
    let mut module = Module::default();
    let lifetime = module.lit(0.25);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut colour_gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colour_gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.3, 1.0)); // White-orange
    colour_gradient.add_key(0.5, Vec4::new(1.0, 0.4, 0.1, 0.8));
    colour_gradient.add_key(1.0, Vec4::new(0.5, 0.2, 0.0, 0.0));

    let mut size_gradient: bevy_hanabi::Gradient<Vec3> = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.08));
    size_gradient.add_key(0.5, Vec3::splat(0.12));
    size_gradient.add_key(1.0, Vec3::splat(0.02));

    EffectAsset::new(32, SpawnerSettings::once((32.0).into()), module)
        .with_name("bullet_impact")
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient: colour_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

/// Hit marker: fast red flash effect on the target.
fn build_hit_marker_effect() -> EffectAsset {
    let mut module = Module::default();
    let lifetime = module.lit(0.12);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut colour_gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colour_gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0)); // Full red
    colour_gradient.add_key(0.3, Vec4::new(1.0, 0.2, 0.0, 0.6));
    colour_gradient.add_key(1.0, Vec4::new(0.5, 0.0, 0.0, 0.0));

    let mut size_gradient: bevy_hanabi::Gradient<Vec3> = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.15));
    size_gradient.add_key(1.0, Vec3::splat(0.02));

    EffectAsset::new(8, SpawnerSettings::once((8.0).into()), module)
        .with_name("hit_marker")
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient: colour_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

/// Death explosion: 6 particles bursting outward with gravity.
fn build_death_explosion_effect() -> EffectAsset {
    let mut module = Module::default();
    let lifetime = module.lit(1.0);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let gravity = module.lit(Vec3::new(0.0, -5.0, 0.0));
    let update_accel = AccelModifier::new(gravity);

    let mut colour_gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colour_gradient.add_key(0.0, Vec4::new(1.0, 0.4, 0.0, 1.0)); // Orange
    colour_gradient.add_key(0.3, Vec4::new(1.0, 0.1, 0.0, 0.8)); // Red
    colour_gradient.add_key(0.7, Vec4::new(0.3, 0.0, 0.0, 0.3));
    colour_gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0));

    let mut size_gradient: bevy_hanabi::Gradient<Vec3> = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.5));
    size_gradient.add_key(0.5, Vec3::splat(1.0));
    size_gradient.add_key(1.0, Vec3::splat(0.1));

    EffectAsset::new(6, SpawnerSettings::once((6.0).into()), module)
        .with_name("death_explosion")
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: colour_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

/// Tracer round: a glowing sphere with a short trail.
fn build_tracer_effect() -> EffectAsset {
    let mut module = Module::default();
    let lifetime = module.lit(0.3);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut colour_gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colour_gradient.add_key(0.0, Vec4::new(0.8, 1.0, 0.3, 1.0)); // Yellow-green glow
    colour_gradient.add_key(0.5, Vec4::new(1.0, 0.6, 0.0, 0.6));
    colour_gradient.add_key(1.0, Vec4::new(0.5, 0.2, 0.0, 0.0));

    let mut size_gradient: bevy_hanabi::Gradient<Vec3> = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.1));
    size_gradient.add_key(1.0, Vec3::splat(0.02));

    EffectAsset::new(16, SpawnerSettings::once((16.0).into()), module)
        .with_name("tracer")
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier {
            gradient: colour_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYSTEMS — Each reads messages and spawns particle effects
// ═══════════════════════════════════════════════════════════════════════════════

/// Spawns muzzle flash at the shooter's position when a weapon fires.
pub fn muzzle_flash_system(
    mut commands: Commands,
    vfx: Res<VfxAssets>,
    mut fired_reader: bevy::ecs::message::MessageReader<crate::messages::WeaponFiredMessage>,
) {
    for msg in fired_reader.read() {
        commands.spawn((
            ParticleEffect::new(vfx.muzzle_flash.clone()),
            Transform::from_translation(msg.position + msg.direction * 0.5),
            VfxEffect {
                timer: Timer::from_seconds(0.08, TimerMode::Once),
            },
        ));
    }
}

/// Spawns bullet impact sparks at the hit point.
pub fn bullet_impact_system(
    mut commands: Commands,
    vfx: Res<VfxAssets>,
    mut hit_reader: bevy::ecs::message::MessageReader<HitConfirmedMessage>,
) {
    for msg in hit_reader.read() {
        if msg.hit {
            commands.spawn((
                ParticleEffect::new(vfx.bullet_impact.clone()),
                Transform::from_translation(msg.hit_point),
                VfxEffect {
                    timer: Timer::from_seconds(0.2, TimerMode::Once),
                },
            ));
        }
    }
}

/// Spawns hit marker effect on damaged entity.
pub fn hit_marker_system(
    mut commands: Commands,
    vfx: Res<VfxAssets>,
    mut damage_reader: bevy::ecs::message::MessageReader<DamageMessage>,
) {
    for msg in damage_reader.read() {
        commands.spawn((
            ParticleEffect::new(vfx.hit_marker.clone()),
            Transform::from_translation(msg.hit_point),
            VfxEffect {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ));
    }
}

/// Spawns death explosion at the death location.
pub fn death_explosion_system(
    mut commands: Commands,
    vfx: Res<VfxAssets>,
    mut death_reader: bevy::ecs::message::MessageReader<DeathMessage>,
    transform_query: Query<&Transform>,
) {
    for msg in death_reader.read() {
        if let Ok(transform) = transform_query.get(msg.entity) {
            commands.spawn((
                ParticleEffect::new(vfx.death_explosion.clone()),
                Transform::from_translation(transform.translation),
                VfxEffect {
                    timer: Timer::from_seconds(1.2, TimerMode::Once),
                },
            ));
        }
    }
}

/// Spawns tracer projectile when a weapon fires.
pub fn tracer_system(
    mut commands: Commands,
    vfx: Res<VfxAssets>,
    mut fired_reader: bevy::ecs::message::MessageReader<crate::messages::WeaponFiredMessage>,
) {
    for msg in fired_reader.read() {
        if !msg.hit_something {
            commands.spawn((
                ParticleEffect::new(vfx.tracer.clone()),
                Transform::from_translation(msg.position),
                TracerProjectile {
                    lifetime: Timer::from_seconds(0.3, TimerMode::Once),
                },
            ));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLEANUP SYSTEMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Despawns one-shot VFX effects after their timer expires.
pub fn cleanup_vfx_system(
    mut commands: Commands,
    time: Res<Time>,
    mut vfx_query: Query<(Entity, &mut VfxEffect)>,
) {
    for (entity, mut effect) in vfx_query.iter_mut() {
        effect.timer.tick(time.delta());
        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Despawns tracer projectiles after their lifetime expires.
pub fn cleanup_tracer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tracer_query: Query<(Entity, &mut TracerProjectile)>,
) {
    for (entity, mut tracer) in tracer_query.iter_mut() {
        tracer.lifetime.tick(time.delta());
        if tracer.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════════════

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        let vfx_assets = {
            let mut assets = app.world_mut().resource_mut::<Assets<EffectAsset>>();
            build_vfx_assets(&mut assets)
        };
        app.insert_resource(vfx_assets);

        app.add_systems(
            Update,
            (
                muzzle_flash_system,
                bullet_impact_system,
                hit_marker_system,
                death_explosion_system,
                tracer_system,
                cleanup_vfx_system,
                cleanup_tracer_system,
            ),
        );
    }
}
