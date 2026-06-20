/// Vehicle damage state machine (design.md §13.3, P5.5.8).
///
/// 4 states: Operational → Disabled (50% HP, speed 0) → Burning (25% HP, fire fx)
/// → Wreck (0% HP, destroyed). Uses `DestructionState` with special transition rules
/// that check the `VehicleDamage` component for state-specific effects.
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use crate::combat::damage::DamageMessage;
use crate::combat::destruction::{DestructionLevel, DestructionState, DestructionTransitionMessage};

/// Vehicle-specific damage overlay component.
///
/// Tracks the specialised vehicle damage state machine on top of the base
/// DestructionState. Transitions:
///   Operational  → Disabled  (health ≤ 50% max)
///   Disabled     → Burning   (health ≤ 25% max)
///   Burning      → Wreck     (health = 0)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleDamageState {
    Operational,
    Disabled,
    Burning,
    Wreck,
}

impl Default for VehicleDamageState {
    fn default() -> Self {
        Self::Operational
    }
}

impl VehicleDamageState {
    /// Determine the next vehicle state based on health ratio and current state.
    pub fn transition(ratio: f32, current: &VehicleDamageState) -> VehicleDamageState {
        match current {
            VehicleDamageState::Wreck => VehicleDamageState::Wreck,
            VehicleDamageState::Burning if ratio <= 0.0 => VehicleDamageState::Wreck,
            VehicleDamageState::Disabled if ratio <= 0.25 => VehicleDamageState::Burning,
            VehicleDamageState::Disabled if ratio <= 0.0 => VehicleDamageState::Wreck,
            VehicleDamageState::Operational if ratio <= 0.50 => VehicleDamageState::Disabled,
            VehicleDamageState::Operational if ratio <= 0.25 => VehicleDamageState::Burning,
            VehicleDamageState::Operational if ratio <= 0.0 => VehicleDamageState::Wreck,
            _ => *current,
        }
    }
}

/// Vehicle component marker — identifies an entity as a vehicle.
#[derive(Component, Debug, Clone)]
pub struct Vehicle {
    /// Top speed multiplier (0.0 when Disabled/Wreck).
    pub speed_mult: f32,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self { speed_mult: 1.0 }
    }
}

/// Reads `DamageMessage`s targeting vehicles and updates both the generic
/// `DestructionState` and the specialised `VehicleDamageState`.
///
/// Also enforces vehicle-specific rules: Disabled sets speed_mult to 0,
/// Burning applies continuous fire damage, Wreck prevents further damage.
pub fn vehicle_damage_state_system(
    mut damage_reader: MessageReader<DamageMessage>,
    mut vehicle_query: Query<(
        Entity,
        &mut DestructionState,
        &mut VehicleDamageState,
        &mut Vehicle,
    )>,
    mut transition_writer: bevy::ecs::message::MessageWriter<DestructionTransitionMessage>,
) {
    for msg in damage_reader.read() {
        let Ok((entity, mut state, mut vehicle_state, mut vehicle)) =
            vehicle_query.get_mut(msg.target)
        else {
            continue;
        };

        // Apply damage to structural health.
        state.health = (state.health - msg.amount.max(0.0)).max(0.0);
        let ratio = state.ratio();

        let new_vs = VehicleDamageState::transition(ratio, &vehicle_state);
        if new_vs == *vehicle_state {
            continue;
        }
        let _old_vs = std::mem::replace(&mut *vehicle_state, new_vs);

        // Enforce vehicle-specific rules.
        match *vehicle_state {
            VehicleDamageState::Disabled | VehicleDamageState::Wreck => {
                vehicle.speed_mult = 0.0;
            }
            VehicleDamageState::Burning => {
                vehicle.speed_mult = 0.0; // Can't move while burning.
            }
            VehicleDamageState::Operational => {
                vehicle.speed_mult = 1.0;
            }
        }

        // Propagate to base destruction state for debris/transition messages.
        let base_level = match *vehicle_state {
            VehicleDamageState::Operational => DestructionLevel::Pristine,
            VehicleDamageState::Disabled => DestructionLevel::Damaged,
            VehicleDamageState::Burning => DestructionLevel::Breached,
            VehicleDamageState::Wreck => DestructionLevel::Destroyed,
        };

        if base_level != state.state {
            let old_base = state.state;
            state.state = base_level;
            transition_writer.write(DestructionTransitionMessage {
                entity,
                from_state: old_base,
                to_state: base_level,
                position: msg.hit_point,
            });
        }
    }
}
