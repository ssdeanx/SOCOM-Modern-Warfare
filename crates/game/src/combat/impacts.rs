use bevy::prelude::*;

/// Transient visual marker (bullet hole, tracer end) that despawns after a timer.
#[derive(Component)]
pub struct ImpactMarker {
    pub timer: Timer,
}

/// Despawns `ImpactMarker` entities when their timer expires.
pub fn impact_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ImpactMarker)>,
) {
    for (entity, mut marker) in query.iter_mut() {
        marker.timer.tick(time.delta());
        if marker.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
