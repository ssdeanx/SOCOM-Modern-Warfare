use avian3d::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

use socom_core::components::{Health, MovementState, Player, WeaponSlot};
use socom_rendering::camera::ThirdPersonCamera;
use socom_rendering::post_processing::PostProcessingProfile;

use crate::combat::weapon_bob::WeaponBobState;
use crate::combat::{OffhandWeaponState, WeaponState};
use crate::physics::CharacterController;
use crate::stamina::Stamina;
use crate::states::ingame::IngameEntity;
use crate::states::AppState;
use crate::weapon_handling::WeaponHandling;
use crate::weapons::{CompleteWeapon, EquippedWeapon};

/// Bundle to spawn a player entity with all required components.
///
/// The player uses a `Kinematic` rigid body with a capsule collider, driven
/// by the `MoveAndSlide` character controller in `PhysicsPlugin`.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub movement_state: MovementState,
    pub weapon_slot: WeaponSlot,
    pub weapon_state: WeaponState,
    pub offhand_state: OffhandWeaponState,
    pub equipped_weapon: EquippedWeapon,
    pub weapon_bob: WeaponBobState,
    pub stamina: Stamina,
    pub weapon_handling: WeaponHandling,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub character_controller: CharacterController,
}

impl PlayerBundle {
    pub fn new() -> Self {
        let weapon_slot = WeaponSlot::default();
        // Build CompleteWeapon from the WeaponSlot's defaults
        let equipped = EquippedWeapon::default();
        Self {
            player: Player,
            health: Health::new(100.0),
            movement_state: MovementState::Standing,
            weapon_state: WeaponState::from_complete_weapon(&equipped.weapon, 0),
            offhand_state: {
                // Initialise the offhand slot (sidearm) with full ammo.
                OffhandWeaponState(WeaponState::from_complete_weapon(&CompleteWeapon::default_m1911(), 1))
            },
            equipped_weapon: equipped,
            weapon_bob: WeaponBobState::default(),
            stamina: Stamina::default(),
            weapon_handling: WeaponHandling::default(),
            weapon_slot,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::capsule(0.3, 0.9),
            collision_layers: CollisionLayers::default(),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            global_transform: default(),
            character_controller: CharacterController::default(),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin for player entity management.
///
/// Spawns the player + camera when entering the `InGame` state, and cleans
/// up on exit so the player can respawn fresh on re-entry.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    let player_entity = commands.spawn((PlayerBundle::new(), IngameEntity)).id();

    // Spawn camera tracking the player with AAA post-processing stack.
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera::new(player_entity),
        IsDefaultUiCamera,
        IngameEntity,
        Tonemapping::AcesFitted,
        Bloom::default(),
        PostProcessingProfile::default(),
    ));
}
