use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

use socom_core::components::{MovementState, Player, Shoulder};
use socom_core::resources::{is_not_paused, GameSettings, SensitivityMultiplier};

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Default 3rd-person orbit radius in meters (Arma-style: tactical awareness @ 6m).
const DEFAULT_DISTANCE: f32 = 6.0;

/// 1st-person camera is placed exactly at eye level (no offset).
const FIRST_PERSON_EYE_OFFSET: f32 = 0.0;

/// Shoulder offset magnitude — wide enough for visible shoulder swap (Q key).
const SHOULDER_OFFSET: f32 = 0.7;

/// Camera collision minimum margin from obstruction.
const CAMERA_COLLISION_MARGIN: f32 = 0.5;

/// Eye heights per stance (metres above ground).
const EYE_HEIGHT_STANDING: f32 = 1.65; // True eye level
const EYE_HEIGHT_SPRINTING: f32 = 1.65;
const EYE_HEIGHT_CROUCHING: f32 = 1.0;
const EYE_HEIGHT_PRONE: f32 = 0.35;

/// Eye-height interpolation speed per frame.
const EYE_HEIGHT_LERP_FACTOR: f32 = 0.08;

/// Base mouse sensitivity fallback.
const DEFAULT_SENSITIVITY: f32 = 0.005;

/// FOV values in degrees.
const DEFAULT_FOV_3RD: f32 = 70.0; // 3rd-person base
const DEFAULT_FOV_1ST: f32 = 80.0; // 1st-person wider for immersion
const ADS_FOV_OFFSET: f32 = -15.0; // FOV reduction when aiming down sights

/// Perspective transition speed (0..1 per frame).
const PERSPECTIVE_LERP_SPEED: f32 = 0.12;

// ═══════════════════════════════════════════════════════════════════════════════
// PERSPECTIVE ENUM
// ═══════════════════════════════════════════════════════════════════════════════

/// The camera perspective mode, matching ARMA/SQUAD toggle behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraPerspective {
    /// Over-the-shoulder third person with shoulder swap.
    #[default]
    ThirdPerson,
    /// First person from eye level (no character visible, weapon-only).
    FirstPerson,
}

// ═══════════════════════════════════════════════════════════════════════════════
// CAMERA COMPONENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Enterprise-grade camera controller supporting 1st/3rd person toggle,
/// ADS zoom, shoulder swap, freelook, and collision.
///
/// Designed to mirror ARMA/SQUAD behaviour:
/// - Scroll wheel or keybind toggles between 1st and 3rd person
/// - Right-click aims (FOV zoom) in both perspectives
/// - Freelook (middle-mouse) separates look direction from movement
/// - Collision raycast prevents clipping through walls
#[derive(Component, Debug, Clone)]
pub struct ThirdPersonCamera {
    /// Target entity to follow (typically the player).
    pub target: Entity,

    // ── Orbit parameters (3rd person) ──
    /// Orbit radius from target.
    pub distance: f32,
    /// Current vertical pitch angle (radians).
    pub pitch: f32,
    /// Current horizontal yaw angle (radians).
    pub yaw: f32,
    /// Pitch clamping.
    pub min_pitch: f32,
    pub max_pitch: f32,

    // ── Perspective ──
    /// Current perspective mode.
    pub perspective: CameraPerspective,
    /// Interpolated perspective factor: 0.0 = full 3rd, 1.0 = full 1st.
    pub perspective_factor: f32,
    /// Desired perspective factor (0.0 or 1.0).
    pub target_perspective_factor: f32,

    // ── Shoulder ──
    pub shoulder: Shoulder,
    pub shoulder_lerp: f32,

    // ── Collision ──
    pub collision: bool,

    // ── FOV ──
    pub fov: f32,
    pub ads_factor: f32,

    // ── Smoothing ──
    pub lerp_factor: f32,
    pub freelook: bool,

    // ── Interpolated state ──
    pub current_eye_height: f32,
    pub desired_position: Vec3,
}

impl ThirdPersonCamera {
    pub fn new(target: Entity) -> Self {
        Self {
            target,
            distance: DEFAULT_DISTANCE,
            pitch: 0.45,
            yaw: 0.0,
            min_pitch: -0.52,
            max_pitch: 1.4,
            perspective: CameraPerspective::ThirdPerson,
            perspective_factor: 0.0,
            target_perspective_factor: 0.0,
            shoulder: Shoulder::Right,
            shoulder_lerp: 1.0,
            collision: true,
            fov: DEFAULT_FOV_3RD,
            ads_factor: 0.0,
            lerp_factor: 0.1,
            freelook: false,
            current_eye_height: EYE_HEIGHT_STANDING,
            desired_position: Vec3::ZERO,
        }
    }

    /// Toggle between first and third person.
    pub fn toggle_perspective(&mut self) {
        self.perspective = match self.perspective {
            CameraPerspective::ThirdPerson => CameraPerspective::FirstPerson,
            CameraPerspective::FirstPerson => CameraPerspective::ThirdPerson,
        };
        self.target_perspective_factor = match self.perspective {
            CameraPerspective::ThirdPerson => 0.0,
            CameraPerspective::FirstPerson => 1.0,
        };
    }

    /// Set perspective directly.
    pub fn set_perspective(&mut self, perspective: CameraPerspective) {
        self.perspective = perspective;
        self.target_perspective_factor = match perspective {
            CameraPerspective::ThirdPerson => 0.0,
            CameraPerspective::FirstPerson => 1.0,
        };
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESOURCE: Camera perspective state for external queries
// ═══════════════════════════════════════════════════════════════════════════════

/// Global resource so other systems can query current perspective.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PerspectiveState {
    pub current: CameraPerspective,
}

// ═══════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════════════

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PerspectiveState::default());
        app.add_systems(
            Update,
            (
                camera_look_system,
                camera_follow_system,
                perspective_toggle_system,
                camera_fov_system,
            )
                .chain()
                .run_if(is_not_paused),
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Orbit offset from spherical coords.
fn orbit_offset(distance: f32, pitch: f32, yaw: f32) -> Vec3 {
    Vec3::new(
        distance * yaw.cos() * pitch.cos(),
        distance * pitch.sin(),
        distance * yaw.sin() * pitch.cos(),
    )
}

/// Target eye height for the given stance.
fn target_eye_height(stance: &MovementState) -> f32 {
    match stance {
        MovementState::Prone => EYE_HEIGHT_PRONE,
        MovementState::Crouching | MovementState::InCover => EYE_HEIGHT_CROUCHING,
        MovementState::Sprinting => EYE_HEIGHT_SPRINTING,
        MovementState::Standing => EYE_HEIGHT_STANDING,
    }
}

/// Compute the final camera FOV based on perspective, ADS, and base.
fn compute_fov(perspective_factor: f32, _base_fov: f32, ads_factor: f32) -> f32 {
    // Blend between 3rd and 1st person base FOV.
    let blended_base = DEFAULT_FOV_3RD.lerp(DEFAULT_FOV_1ST, perspective_factor);
    // Apply ADS zoom offset.
    (blended_base + ADS_FOV_OFFSET * ads_factor)
        .max(40.0)
        .min(120.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSPECTIVE TOGGLE SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Listens for perspective toggle input (middle-mouse click or V key)
/// and updates the camera's perspective mode.
fn perspective_toggle_system(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cam_query: Query<&mut ThirdPersonCamera>,
    mut perspective_state: ResMut<PerspectiveState>,
) {
    let toggle = mouse.just_pressed(MouseButton::Middle) || keys.just_pressed(KeyCode::KeyV);

    if toggle {
        for mut cam in cam_query.iter_mut() {
            cam.toggle_perspective();
            perspective_state.current = cam.perspective;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FOV SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Applies computed FOV to camera projection based on perspective + ADS.
fn camera_fov_system(mut cam_query: Query<(&ThirdPersonCamera, &mut bevy::camera::Projection)>) {
    for (cam, mut projection) in cam_query.iter_mut() {
        if let bevy::camera::Projection::Perspective(ref mut persp) = &mut *projection {
            persp.fov = compute_fov(cam.perspective_factor, cam.fov, cam.ads_factor).to_radians();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CAMERA FOLLOW SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Positions the camera based on perspective:
/// - **3rd person:** Spherical orbit around target with shoulder offset + collision
/// - **1st person:** At eye level, no orbit, no shoulder offset
/// Smoothly interpolated via `perspective_factor`.
fn camera_follow_system(
    spatial_query: SpatialQuery,
    mut cam_query: Query<(&mut ThirdPersonCamera, &mut Transform)>,
    target_query: Query<&Transform, Without<ThirdPersonCamera>>,
    player_query: Query<&MovementState, With<Player>>,
) {
    let stance = player_query
        .iter()
        .next()
        .map_or(MovementState::Standing, |s| *s);
    let target_height = target_eye_height(&stance);

    for (mut cam, mut transform) in cam_query.iter_mut() {
        // Copy target entity to avoid borrow conflict with cam mutation below
        let target_entity = cam.target;
        let Ok(target_transform) = target_query.get(target_entity) else {
            continue;
        };

        // ── 1. Update perspective interpolation ─────────────────────
        cam.perspective_factor +=
            (cam.target_perspective_factor - cam.perspective_factor) * PERSPECTIVE_LERP_SPEED;

        // ── 2. Smoothly interpolate eye height ──────────────────────
        cam.current_eye_height += (target_height - cam.current_eye_height) * EYE_HEIGHT_LERP_FACTOR;

        let target_pos = target_transform.translation;
        let eye_pos = target_pos + Vec3::Y * cam.current_eye_height;

        // ── 3. Compute 3rd-person desired position ──────────────────
        let offset = orbit_offset(cam.distance, cam.pitch, cam.yaw);

        let target_shoulder = match cam.shoulder {
            Shoulder::Right => 1.0,
            Shoulder::Left => -1.0,
        };
        cam.shoulder_lerp += (target_shoulder - cam.shoulder_lerp) * 0.15;
        let shoulder_offset = Vec3::X * SHOULDER_OFFSET * cam.shoulder_lerp;

        let desired_3rd = eye_pos + offset + shoulder_offset;
        let desired_1st = eye_pos; // First person: camera at eye

        // ── 4. Blend between 1st and 3rd person ─────────────────────
        let pf = cam.perspective_factor;
        let mut final_desired = desired_3rd.lerp(desired_1st, pf);

        // ── 5. Camera collision (3rd person only) ───────────────────
        if cam.collision && pf < 0.95 {
            let direction = (desired_3rd - eye_pos).normalize();
            let max_dist = (desired_3rd - eye_pos).length();
            let filter = SpatialQueryFilter::default();

            if let Some(hit) = spatial_query.cast_ray(
                eye_pos,
                Dir3::new(direction).unwrap_or(Dir3::Z),
                max_dist,
                true,
                &filter,
            ) {
                let pushback = hit.distance.max(CAMERA_COLLISION_MARGIN);
                let collision_pos = eye_pos + direction * pushback + shoulder_offset;
                // Blend collision position with 1st person target
                final_desired = collision_pos.lerp(desired_1st, pf);
            }
        }

        cam.desired_position = final_desired;

        // ── 6. Smooth lerp ───────────────────────────────────────────
        let t = (1.0 - cam.lerp_factor).clamp(0.0, 1.0);
        let new_pos = transform.translation.lerp(final_desired, t);
        transform.translation = new_pos;

        // ── 7. Look direction ────────────────────────────────────────
        let cam_pos = transform.translation;
        if pf < 0.5 {
            transform.look_at(eye_pos, Vec3::Y);
        } else {
            let fwd = *target_transform.forward();
            transform.look_at(cam_pos + fwd * 100.0, Vec3::Y);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CAMERA LOOK SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Reads mouse motion and updates camera yaw/pitch.
/// Sensitivity is modulated by stance, weapon weight, and stamina.
fn camera_look_system(
    mut cam_query: Query<&mut ThirdPersonCamera>,
    mouse: Res<AccumulatedMouseMotion>,
    settings: Option<Res<GameSettings>>,
    sens_mult: Option<Res<SensitivityMultiplier>>,
) {
    let delta = mouse.delta;
    if delta == Vec2::ZERO {
        return;
    }

    let base_sens = settings
        .as_ref()
        .map_or(DEFAULT_SENSITIVITY, |s| s.sensitivity * DEFAULT_SENSITIVITY);
    let mult = sens_mult.map_or(1.0, |m| m.0);
    let sensitivity = base_sens * mult;
    let invert = settings.as_ref().is_some_and(|s| s.invert_y);

    for mut cam in cam_query.iter_mut() {
        if cam.freelook {
            continue;
        }
        cam.yaw -= delta.x * sensitivity;
        cam.pitch += delta.y * sensitivity * if invert { -1.0 } else { 1.0 };
        cam.pitch = cam.pitch.clamp(cam.min_pitch, cam.max_pitch);
    }
}
