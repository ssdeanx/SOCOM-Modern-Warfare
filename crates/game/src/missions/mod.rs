use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveType { EliminateAll, ReachLocation, DefendPosition, CollectIntel, Extract }
impl ObjectiveType {
    pub fn name(&self) -> &'static str { match self { ObjectiveType::EliminateAll => "Eliminate", ObjectiveType::ReachLocation => "Reach", ObjectiveType::DefendPosition => "Defend", ObjectiveType::CollectIntel => "Intel", ObjectiveType::Extract => "Extract" } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub objective_type: ObjectiveType,
    pub description: String,
    pub target_count: u32,
    pub current_count: u32,
    pub completed: bool,
    pub position: Option<Vec3>,
}
impl Objective {
    pub fn new(t: ObjectiveType, desc: &str, count: u32) -> Self {
        Self { objective_type: t, description: desc.to_string(), target_count: count, current_count: 0, completed: false, position: None }
    }
    pub fn progress(&mut self) -> bool {
        if self.completed { return false; }
        self.current_count = (self.current_count + 1).min(self.target_count);
        if self.current_count >= self.target_count { self.completed = true; true } else { false }
    }
}

#[derive(Resource, Debug)]
pub struct MissionState {
    pub objectives: Vec<Objective>,
    pub all_completed: bool,
    pub mission_name: String,
    pub briefing: String,
}
impl Default for MissionState {
    fn default() -> Self { Self { objectives: vec![Objective::new(ObjectiveType::EliminateAll, "Eliminate all enemies", 2)], all_completed: false, mission_name: "Training Exercise".into(), briefing: "Eliminate all hostile targets.".into() } }
}
impl MissionState { pub fn check_completion(&mut self) { self.all_completed = self.objectives.iter().all(|o| o.completed); } }

use crate::combat::death::DeathMessage;
fn update_objectives_system(mut mission: ResMut<MissionState>, mut death_reader: bevy::ecs::message::MessageReader<DeathMessage>) {
    for msg in death_reader.read() {
        for obj in mission.objectives.iter_mut() {
            if obj.objective_type == ObjectiveType::EliminateAll && !obj.completed && msg.source.is_some() { obj.progress(); }
        }
    }
}
fn check_mission_system(mut mission: ResMut<MissionState>) { mission.check_completion(); }

pub struct MissionPlugin;
impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MissionState>();
        app.add_systems(Update, (update_objectives_system, check_mission_system));
    }
}
