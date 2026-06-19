use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Player, WeaponSlot};
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

/// Root entity for the visible weapon model (child of camera).
#[derive(Component)]
pub struct WeaponModelRoot;

/// Marker for the rifle model group.
#[derive(Component)]
pub struct RifleModel;

/// Marker for the pistol model group.
#[derive(Component)]
pub struct PistolModel;

/// Marker for the weapon muzzle flash light entity.
#[derive(Component)]
pub struct MuzzleFlashLight;

const MUZZLE_FLASH_DURATION: f32 = 0.05;

/// Spawns a procedural rifle model as children of `parent`.
fn spawn_rifle_parts(
    commands: &mut Commands,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let weapon_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.15),
        ..default()
    });
    let barrel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.22),
        ..default()
    });
    let grip_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.08, 0.06),
        ..default()
    });

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.06, 0.06, 0.25))),
            MeshMaterial3d(weapon_mat.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RifleModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.03, 0.03, 0.35))),
            MeshMaterial3d(barrel_mat.clone()),
            Transform::from_xyz(0.0, 0.0, -0.28),
            RifleModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.04, 0.04, 0.12))),
            MeshMaterial3d(weapon_mat.clone()),
            Transform::from_xyz(0.0, 0.0, 0.16),
            RifleModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.03, 0.08, 0.03))),
            MeshMaterial3d(grip_mat.clone()),
            Transform::from_xyz(0.0, -0.05, -0.02),
            RifleModel,
        ))
        .set_parent_in_place(parent);
}

/// Spawns a procedural pistol model as children of `parent`.
fn spawn_pistol_parts(
    commands: &mut Commands,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let body_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.12),
        ..default()
    });
    let barrel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        ..default()
    });
    let grip_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.06, 0.04),
        ..default()
    });

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.04, 0.04, 0.12))),
            MeshMaterial3d(body_mat.clone()),
            Transform::from_xyz(0.0, 0.01, 0.02),
            PistolModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.02, 0.02, 0.10))),
            MeshMaterial3d(barrel_mat.clone()),
            Transform::from_xyz(0.0, 0.0, -0.10),
            PistolModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.03, 0.07, 0.03))),
            MeshMaterial3d(grip_mat.clone()),
            Transform::from_xyz(0.0, -0.04, 0.02),
            PistolModel,
        ))
        .set_parent_in_place(parent);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.025, 0.01, 0.025))),
            MeshMaterial3d(body_mat.clone()),
            Transform::from_xyz(0.0, -0.02, -0.01),
            PistolModel,
        ))
        .set_parent_in_place(parent);
}

/// Spawns both rifle and pistol models as children of the camera.
pub fn spawn_weapon_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Query<Entity, (With<Camera3d>, With<ThirdPersonCamera>)>,
) {
    let Ok(_camera_entity) = camera_query.single() else {
        return;
    };

    let weapon_root = commands
        .spawn((
            Transform::from_xyz(0.35, -0.25, -0.6),
            GlobalTransform::default(),
            Visibility::Inherited,
            WeaponModelRoot,
            crate::states::ingame::IngameEntity,
        ))
        .id();

    let rifle_group = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
            RifleModel,
        ))
        .set_parent_in_place(weapon_root)
        .id();
    spawn_rifle_parts(&mut commands, rifle_group, &mut meshes, &mut materials);

    let pistol_group = commands
        .spawn((
            Transform::from_xyz(-0.05, 0.0, 0.08),
            GlobalTransform::default(),
            Visibility::Hidden,
            PistolModel,
        ))
        .set_parent_in_place(weapon_root)
        .id();
    spawn_pistol_parts(&mut commands, pistol_group, &mut meshes, &mut materials);

    commands
        .spawn((
            PointLight {
                intensity: 30_000.0,
                color: Color::srgb(1.0, 0.7, 0.2),
                range: 3.0,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -0.5),
            Visibility::Hidden,
            MuzzleFlashLight,
        ))
        .set_parent_in_place(weapon_root);
}

/// Toggles visible weapon model based on active slot.
pub fn weapon_model_swap_system(
    player_query: Query<&WeaponSlot, With<Player>>,
    mut rifle_query: Query<&mut Visibility, (With<RifleModel>, Without<PistolModel>)>,
    mut pistol_query: Query<&mut Visibility, (With<PistolModel>, Without<RifleModel>)>,
) {
    let Ok(weapon_slot) = player_query.single() else {
        return;
    };
    let rifle_visible = weapon_slot.active_slot == 0;
    for mut vis in rifle_query.iter_mut() {
        *vis = if rifle_visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for mut vis in pistol_query.iter_mut() {
        *vis = if rifle_visible {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

/// Mirrors the weapon model's X position based on camera shoulder.
/// When on the left shoulder, the weapon appears on the left side of the screen.
pub fn weapon_shoulder_mirror_system(
    camera_query: Query<&ThirdPersonCamera>,
    mut weapon_root_query: Query<&mut Transform, With<WeaponModelRoot>>,
) {
    let Ok(cam) = camera_query.single() else {
        return;
    };
    // shoulder_lerp: 1.0 = right, -1.0 = left
    let mirror_x = 0.35 * cam.shoulder_lerp;
    for mut transform in weapon_root_query.iter_mut() {
        transform.translation.x = mirror_x;
    }
}

/// Shows muzzle flash light briefly when player fires.
pub fn weapon_model_flash_system(
    time: Res<Time>,
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut flash_query: Query<
        &mut Visibility,
        (
            With<MuzzleFlashLight>,
            Without<RifleModel>,
            Without<PistolModel>,
        ),
    >,
    mut last_flash_time: Local<f32>,
) {
    let Ok(action_state) = player_query.single() else {
        return;
    };
    let now = time.elapsed_secs();
    if action_state.just_pressed(&PlayerAction::Fire) {
        *last_flash_time = now;
        for mut vis in flash_query.iter_mut() {
            *vis = Visibility::Inherited;
        }
    }
    if now - *last_flash_time > MUZZLE_FLASH_DURATION {
        for mut vis in flash_query.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }
}
