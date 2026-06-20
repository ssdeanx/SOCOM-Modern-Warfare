use avian3d::prelude::*;
use bevy::prelude::*;

use socom_core::components::Player;

use crate::messages::CoverStateMessage;

/// Component marking an entity as being in cover.
#[derive(Component, Debug)]
pub struct InCover {
    pub cover_entity: Entity,
    pub cover_type: CoverType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverType {
    #[expect(dead_code, reason = "awaiting full cover system")]
    Low,
    High,
    #[expect(dead_code, reason = "awaiting full cover system")]
    CornerLeft,
    #[expect(dead_code, reason = "awaiting full cover system")]
    CornerRight,
}

/// Detects nearby walls and marks the player as in cover when appropriate.
pub fn cover_detection_system(
    spatial_query: SpatialQuery,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
    mut cover_writer: bevy::ecs::message::MessageWriter<CoverStateMessage>,
) {
    let Ok((player_entity, transform)) = player_query.single() else {
        return;
    };
    let pos = transform.translation;
    let cover_distance = 0.6;
    let directions = [Vec3::X, Vec3::NEG_X, Vec3::Z, Vec3::NEG_Z];
    let mut found_cover = false;

    for dir in &directions {
        if let Ok(dir3) = Dir3::new(*dir) {
            if spatial_query
                .cast_ray(
                    pos,
                    dir3,
                    cover_distance,
                    true,
                    &SpatialQueryFilter::default(),
                )
                .is_some()
            {
                found_cover = true;
                break;
            }
        }
    }

    if found_cover {
        commands.entity(player_entity).insert(InCover {
            cover_entity: player_entity,
            cover_type: CoverType::High,
        });
        cover_writer.write(CoverStateMessage {
            entity: player_entity,
            in_cover: true,
        });
    } else {
        commands.entity(player_entity).remove::<InCover>();
        cover_writer.write(CoverStateMessage {
            entity: player_entity,
            in_cover: false,
        });
    }
}
