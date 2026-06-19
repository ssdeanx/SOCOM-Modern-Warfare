use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ai::enemy::EnemyBundle;
use crate::ai::teammate::TeammateBundle;
use crate::states::ingame::IngameEntity;
use crate::states::AppState;

/// Marker for entities that are part of the level (despawned on respawn).
#[derive(Component)]
pub struct LevelEntity;

/// Plugin for the procedural test level.
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_test_level_system);
    }
}

/// System wrapper that calls the level spawn logic.
fn spawn_test_level_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_test_level(commands, meshes, materials);
}

/// Spawns the greybox test level: geometry, enemies, and teammate.
/// Public so the respawn system can call it on player death.
pub fn spawn_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mat = materials.add(StandardMaterial::from(Color::srgb(0.3, 0.3, 0.3)));
    let wall_mat = materials.add(StandardMaterial::from(Color::srgb(0.5, 0.4, 0.3)));
    let pillar_mat = materials.add(StandardMaterial::from(Color::srgb(0.4, 0.4, 0.5)));
    let ramp_mat = materials.add(StandardMaterial::from(Color::srgb(0.4, 0.5, 0.3)));
    let stair_mat = materials.add(StandardMaterial::from(Color::srgb(0.5, 0.3, 0.3)));

    let half_size = 10.0;
    let wall_height = 3.0;
    let wall_thick = 0.3;

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(ground_mat.clone()),
        Transform::from_xyz(0.0, -0.01, 0.0),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        IngameEntity,
        LevelEntity,
    ));
    // North wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(half_size * 2.0, wall_height, wall_thick))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, wall_height / 2.0, -half_size),
        RigidBody::Static,
        Collider::cuboid(half_size, wall_height / 2.0, wall_thick / 2.0),
        IngameEntity,
        LevelEntity,
    ));
    // South wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(half_size * 2.0, wall_height, wall_thick))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, wall_height / 2.0, half_size),
        RigidBody::Static,
        Collider::cuboid(half_size, wall_height / 2.0, wall_thick / 2.0),
        IngameEntity,
        LevelEntity,
    ));
    // West wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall_thick, wall_height, half_size * 2.0))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(-half_size, wall_height / 2.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(wall_thick / 2.0, wall_height / 2.0, half_size),
        IngameEntity,
        LevelEntity,
    ));
    // East wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall_thick, wall_height, half_size * 2.0))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(half_size, wall_height / 2.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(wall_thick / 2.0, wall_height / 2.0, half_size),
        IngameEntity,
        LevelEntity,
    ));

    // Pillars
    for pos in [
        Vec3::new(-4.0, 0.5, -4.0),
        Vec3::new(4.0, 0.5, -4.0),
        Vec3::new(-4.0, 0.5, 4.0),
        Vec3::new(4.0, 0.5, 4.0),
    ] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(pillar_mat.clone()),
            Transform::from_translation(pos),
            RigidBody::Static,
            Collider::cuboid(0.5, 0.5, 0.5),
            IngameEntity,
            LevelEntity,
        ));
    }

    // Ramp
    let ramp_angle = -0.52_f32;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.2, 4.0))),
        MeshMaterial3d(ramp_mat.clone()),
        Transform::from_xyz(-5.0, 1.0, 3.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            ramp_angle,
            0.0,
            0.0,
        )),
        RigidBody::Static,
        Collider::cuboid(1.0, 0.1, 2.0),
        IngameEntity,
        LevelEntity,
    ));

    // Stairs
    for step in 0..5 {
        let y = 0.25 + step as f32 * 0.5;
        let z = 2.0 + step as f32 * 0.5;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 0.5, 0.5))),
            MeshMaterial3d(stair_mat.clone()),
            Transform::from_xyz(5.0, y, z),
            RigidBody::Static,
            Collider::cuboid(1.0, 0.25, 0.25),
            IngameEntity,
            LevelEntity,
        ));
    }

    // Enemies
    commands.spawn((
        EnemyBundle::new(
            Vec3::new(-3.0, 0.0, -3.0),
            Vec3::new(-3.0, 0.0, -7.0),
            Vec3::new(-7.0, 0.0, -3.0),
        ),
        IngameEntity,
        LevelEntity,
    ));
    commands.spawn((
        EnemyBundle::new(
            Vec3::new(3.0, 0.0, 3.0),
            Vec3::new(7.0, 0.0, 3.0),
            Vec3::new(3.0, 0.0, 7.0),
        ),
        IngameEntity,
        LevelEntity,
    ));

    // Teammate
    commands.spawn((
        TeammateBundle::new(Vec3::new(-1.5, 0.0, 0.0)),
        IngameEntity,
        LevelEntity,
    ));
}
