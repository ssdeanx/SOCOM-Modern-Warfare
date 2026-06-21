/// DEPRECATED: hitscan shooting system.
///
/// The bullet-spawn logic has been moved to `ballistics/spawn.rs` (the
/// `bullet_spawn_system` function).  The old hitscan approach is replaced
/// by the full projectile-ballistics pipeline.
///
/// The helper functions `bullet_ray` and `fast_random_seeded` are now
/// duplicated in `ballistics/spawn.rs` so that this file can be removed
/// entirely once all references are cleaned up.
///
/// Scheduled for removal: Next refactor pass.

use avian3d::prelude::*;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, Player};
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::damage::{DamageMessage, Dead};
use crate::combat::impacts::ImpactMarker;
use crate::combat::weapon_bob::AdsState;
use crate::combat::weapon_state::WeaponState;
use crate::messages::WeaponFiredMessage;
use crate::weapons::EquippedWeapon;

/// Stub – hitscan is replaced by ballistics. This function body is empty.
/// The real firing logic lives in `ballistics/spawn.rs::bullet_spawn_system`.
#[allow(dead_code, reason = "Kept for reference until full cleanup")]
pub fn shooting_system() {}
