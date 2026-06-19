use bevy::ecs::message::Message;
use bevy::prelude::*;

use socom_core::components::{Player, Team};

/// Orders that can be issued to the squad.
#[derive(Clone, Debug, PartialEq)]
pub enum SquadOrder {
    MoveToTarget(Entity),
    EngageTarget(Entity),
    SuppressPosition(Entity),
    RegroupOnPlayer,
    HoldPosition,
}

/// Message fired when a squad order is issued.
#[derive(Message, Clone, Debug)]
pub struct SquadOrderMessage {
    pub order: SquadOrder,
    pub source: Entity,
}

/// Tracks currently active orders per squad member.
#[derive(Resource)]
pub struct ActiveOrders {
    pub orders: std::collections::HashMap<Entity, SquadOrder>,
}

impl Default for ActiveOrders {
    fn default() -> Self {
        Self {
            orders: std::collections::HashMap::new(),
        }
    }
}

/// Dispatches squad orders to all teammate entities.
pub fn squad_order_dispatch_system(
    mut messages: bevy::ecs::message::MessageReader<SquadOrderMessage>,
    mut active: ResMut<ActiveOrders>,
    teammate_query: Query<Entity, (With<Team>, Without<Player>)>,
) {
    for msg in messages.read() {
        for entity in &teammate_query {
            active.orders.insert(entity, msg.order.clone());
        }
    }
}
