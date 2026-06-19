use crate::combat::damage::DamageMessage;
use crate::combat::death::DeathMessage;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use socom_core::components::Team;

#[derive(Component)]
pub(crate) struct EnemyHurtFlash;
#[derive(Component)]
pub(crate) struct EnemyDeathParticle {
    velocity: Vec3,
    timer: Timer,
}

pub fn enemy_hurt_flash_system(
    mut damage_reader: MessageReader<DamageMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_query: Query<&Team>,
) {
    let flash_mesh = meshes.add(Sphere::new(0.1));
    let flash_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.1, 0.1),
        emissive: LinearRgba::new(1.0, 0.1, 0.0, 1.0),
        unlit: true,
        ..default()
    });
    for msg in damage_reader.read() {
        let is_enemy = enemy_query.get(msg.target).is_ok_and(|t| *t == Team::Enemy);
        if !is_enemy {
            continue;
        }
        commands.spawn((
            Mesh3d(flash_mesh.clone()),
            MeshMaterial3d(flash_mat.clone()),
            Transform::from_translation(msg.hit_point),
            EnemyHurtFlash,
        ));
    }
}

pub fn hurt_flash_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<(Entity, &EnemyHurtFlash)>,
    mut timers: Local<Vec<(Entity, Timer)>>,
) {
    timers.retain(|(entity, timer)| {
        if timer.just_finished() {
            commands.entity(*entity).despawn();
            false
        } else {
            true
        }
    });
    for (entity, _) in &query {
        if !timers.iter().any(|(e, _)| *e == entity) {
            timers.push((entity, Timer::from_seconds(0.08, TimerMode::Once)));
        }
    }
    for (_, timer) in timers.iter_mut() {
        timer.tick(time.delta());
    }
}

pub fn enemy_death_effect_system(
    mut death_reader: MessageReader<DeathMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transform_query: Query<&Transform>,
    enemy_query: Query<&Team>,
) {
    let mesh = meshes.add(Sphere::new(0.05));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0),
        emissive: LinearRgba::new(0.5, 0.2, 0.0, 1.0),
        unlit: true,
        ..default()
    });
    for msg in death_reader.read() {
        if !enemy_query.get(msg.entity).is_ok_and(|t| *t == Team::Enemy) {
            continue;
        }
        let Ok(transform) = transform_query.get(msg.entity) else {
            continue;
        };
        let pos = transform.translation;
        for i in 0..6 {
            let angle = i as f32 * std::f32::consts::TAU / 6.0;
            let dir = Vec3::new(angle.cos(), 0.3, angle.sin()).normalize();
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_translation(pos),
                EnemyDeathParticle {
                    velocity: dir * 3.0,
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                },
            ));
        }
    }
}

pub fn death_particle_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut EnemyDeathParticle)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (entity, mut transform, mut particle) in query.iter_mut() {
        transform.translation += particle.velocity * dt;
        particle.velocity.y -= 9.8 * dt;
        particle.timer.tick(time.delta());
        if particle.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
