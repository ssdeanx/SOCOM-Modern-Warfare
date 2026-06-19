use crate::physics::layers::CharacterController;
use avian3d::character_controller::move_and_slide::{
    MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse, MoveAndSlideOutput,
};
use avian3d::math::{AdjustPrecision, AsF32};
use avian3d::prelude::*;
use bevy::prelude::*;
use socom_core::components::{Player, Team};

const GRAVITY: f32 = -19.6;
const MAX_FALL_SPEED: f32 = -30.0;

type AiMovementQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (
        Entity,
        &'a Collider,
        &'a mut CharacterController,
        &'a mut Transform,
    ),
    (With<Team>, Without<Player>),
>;

pub fn enemy_movement_system(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut query: AiMovementQuery,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (entity, collider, mut controller, mut transform) in query.iter_mut() {
        controller.velocity.y += GRAVITY * dt;
        if controller.velocity.y < MAX_FALL_SPEED {
            controller.velocity.y = MAX_FALL_SPEED;
        }
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let config = MoveAndSlideConfig::default();
        let shape_position = transform.translation.adjust_precision();
        let shape_rotation = transform.rotation.adjust_precision();
        let vel = controller.velocity.adjust_precision();
        let MoveAndSlideOutput {
            position: new_position,
            projected_velocity: new_velocity,
        } = move_and_slide.move_and_slide(
            collider,
            shape_position,
            shape_rotation,
            vel,
            time.delta(),
            &config,
            &filter,
            |_hit| MoveAndSlideHitResponse::Accept,
        );
        transform.translation = new_position.f32();
        controller.velocity = new_velocity.f32();
    }
}
