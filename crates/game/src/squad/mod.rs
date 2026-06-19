pub mod formation;
pub mod orders;

use bevy::prelude::*;

pub struct SquadPlugin;

impl Plugin for SquadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<orders::SquadOrderMessage>();
        app.init_resource::<orders::ActiveOrders>();
        app.add_systems(
            Update,
            (
                orders::squad_order_dispatch_system,
                formation::squad_formation_system,
            ),
        );
    }
}
