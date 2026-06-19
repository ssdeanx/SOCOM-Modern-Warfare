use avian3d::prelude::*;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use socom_core::components::{Health, Player, Weapon, WeaponSlot};
use socom_input::actions::PlayerAction;
use socom_rendering::camera::ThirdPersonCamera;

use crate::combat::damage::{DamageMessage, Dead};
use crate::combat::impacts::ImpactMarker;
use crate::combat::weapon_bob::AdsState;
use crate::combat::weapon_state::WeaponState;

const MAX_SHOOT_DISTANCE: f32 = 300.0;
const IMPACT_LIFETIME: f32 = 2.0;
const RAYCAST_NEAR_CLIP: f32 = 0.5;

/// Computes the ray origin and direction for a shot, applying weapon spread.
/// `ads_mult` is the ADS spread multiplier (1.0 = hip, 0.5 = fully aimed).
fn bullet_ray(camera_transform: &Transform, weapon: &Weapon, ads_mult: f32) -> (Vec3, Vec3) {
    let origin = camera_transform.translation;
    let forward = *camera_transform.forward();
    let spread_rad = weapon.spread_degrees.to_radians() * ads_mult;
    if spread_rad <= 0.0 {
        return (origin, forward);
    }

    let angle = std::f32::consts::TAU * fast_random();
    let radius = fast_random().sqrt() * spread_rad.tan() * RAYCAST_NEAR_CLIP;

    let up = Vec3::Y;
    let right = forward.cross(up).normalize_or_zero();
    let up = if right == Vec3::ZERO {
        Vec3::Z
    } else {
        right.cross(forward).normalize()
    };
    let right = forward.cross(up).normalize();

    let spread_offset = right * (radius * angle.cos()) + up * (radius * angle.sin());
    let direction = (forward + spread_offset).normalize();
    (origin, direction)
}

/// Simple LCG hash for pseudo-random spread.
fn fast_random() -> f32 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let frac = (t
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)) as f32;
    (frac.abs() % 1000.0) / 1000.0
}

/// Main hitscan shooting system.
#[allow(clippy::too_many_arguments)]
pub fn shooting_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut damage_writer: MessageWriter<DamageMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            &WeaponSlot,
            Option<&mut WeaponState>,
            &Health,
        ),
        With<Player>,
    >,
    inventory: Option<Res<crate::gear::inventory::PlayerInventory>>,
    camera_query: Query<&Transform, With<ThirdPersonCamera>>,
    ads_state: Res<AdsState>,
    dead_query: Query<(), With<Dead>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let impact_mesh: Handle<Mesh> = meshes.add(Sphere::new(0.04));
    let impact_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.15),
        unlit: true,
        ..default()
    });
    let tracer_mesh: Handle<Mesh> = meshes.add(Sphere::new(0.03));
    let tracer_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.3),
        emissive: LinearRgba::new(0.5, 0.4, 0.0, 1.0),
        unlit: true,
        ..default()
    });

    for (player_entity, action_state, weapon_slot, weapon_state_opt, health) in
        player_query.iter_mut()
    {
        // Don't allow firing when dead
        if !health.is_alive() {
            continue;
        }
        let fire_pressed = action_state.pressed(&PlayerAction::Fire);
        if !fire_pressed {
            continue;
        }

        let Some(weapon) = weapon_slot.active_weapon() else {
            continue;
        };

        let should_fire = if weapon.is_automatic {
            fire_pressed
        } else {
            action_state.just_pressed(&PlayerAction::Fire)
        };
        if !should_fire {
            continue;
        }

        let fire_interval = 1.0 / weapon.fire_rate;
        let can_fire = weapon_state_opt.as_ref().is_some_and(|ws| {
            !ws.is_reloading
                && ws.magazine > 0
                && time.elapsed_secs() - ws.last_fire_time >= fire_interval
        });
        if !can_fire {
            continue;
        }

        // Fire!
        let (ray_origin, ray_dir) = bullet_ray(camera_transform, weapon, ads_state.spread_mult);
        let ray_dir3 = Dir3::new(ray_dir).unwrap_or(Dir3::Z);

        let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);
        let max_range = weapon.max_range.min(MAX_SHOOT_DISTANCE);
        // Exclude already-dead entities from being shot further
        let hit = spatial_query.cast_ray(ray_origin, ray_dir3, max_range, true, &filter);

        if let Some(hit_data) = hit {
            let hit_entity = hit_data.entity;
            if dead_query.contains(hit_entity) {
                continue;
            }
            let hit_point = ray_origin + ray_dir * hit_data.distance;
            // Apply inventory damage bonus from equipped gear
            let damage_bonus = inventory
                .as_ref()
                .map(|inv| inv.weapon_damage_bonus())
                .unwrap_or(0.0);
            damage_writer.write(DamageMessage {
                target: hit_entity,
                amount: weapon.damage + damage_bonus,
                source: player_entity,
                hit_point,
                hit_normal: hit_data.normal,
            });

            commands.spawn((
                Mesh3d(impact_mesh.clone()),
                MeshMaterial3d(impact_mat.clone()),
                Transform {
                    translation: hit_point + hit_data.normal * 0.01,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(1.0),
                },
                ImpactMarker {
                    timer: Timer::from_seconds(IMPACT_LIFETIME, TimerMode::Once),
                },
            ));
        } else {
            let end = ray_origin + ray_dir * max_range;
            commands.spawn((
                Mesh3d(tracer_mesh.clone()),
                MeshMaterial3d(tracer_mat.clone()),
                Transform::from_translation(end),
                ImpactMarker {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                },
            ));
        }

        // Update weapon state
        if let Some(mut ws) = weapon_state_opt {
            ws.last_fire_time = time.elapsed_secs();
            ws.magazine = ws.magazine.saturating_sub(1);
            if ws.magazine == 0 && ws.reserve > 0 {
                ws.is_reloading = true;
                ws.reload_timer = weapon.reload_time;
                let click: Handle<AudioSource> = asset_server.load("audio/ui_click.ogg");
                commands.spawn((
                    AudioPlayer(click),
                    PlaybackSettings::ONCE.with_volume(Volume::Linear(0.6)),
                ));
            }
        }

        // Weapon fire sound
        let sound_path = match weapon_slot.active_slot {
            1 => "audio/weapon_1911.ogg",
            _ if weapon.name == "MP5SD" => "audio/weapon_mp5.ogg",
            _ => "audio/weapon_m4.ogg",
        };
        let handle: Handle<AudioSource> = asset_server.load(sound_path);
        commands.spawn((
            AudioPlayer(handle),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.5)),
        ));

        // Muzzle flash
        commands.spawn((
            PointLight {
                intensity: 50_000.0,
                color: Color::srgb(1.0, 0.7, 0.2),
                range: 5.0,
                ..default()
            },
            Transform::from_translation(ray_origin + ray_dir * 0.5),
            ImpactMarker {
                timer: Timer::from_seconds(MUZZLE_FLASH_DURATION, TimerMode::Once),
            },
        ));
    }
}

const MUZZLE_FLASH_DURATION: f32 = 0.05;
