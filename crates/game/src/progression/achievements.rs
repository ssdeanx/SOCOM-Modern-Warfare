use bevy::prelude::*;

use crate::progression::stats::PlayerStats;

/// All achievable accolades in the game.
#[expect(dead_code, reason = "awaiting gameplay loop to trigger")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Achievement {
    FirstBlood,    // First kill
    DoubleTap,     // Two rapid kills
    Survivor,      // Survive with <10 HP
    Headhunter,    // 10 headshots
    Unstoppable,   // 20 kills without dying
    Gunsmith,      // Fully mod a weapon
    Perfectionist, // Complete a mission without dying
}

#[expect(dead_code, reason = "awaiting gameplay loop to trigger")]
impl Achievement {
    pub fn name(&self) -> &'static str {
        match self {
            Achievement::FirstBlood => "First Blood",
            Achievement::DoubleTap => "Double Tap",
            Achievement::Survivor => "Survivor",
            Achievement::Headhunter => "Headhunter",
            Achievement::Unstoppable => "Unstoppable",
            Achievement::Gunsmith => "Gunsmith",
            Achievement::Perfectionist => "Perfectionist",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Achievement::FirstBlood => "Eliminate your first enemy",
            Achievement::DoubleTap => "Kill two enemies in quick succession",
            Achievement::Survivor => "Survive an encounter with less than 10 HP",
            Achievement::Headhunter => "Get 10 headshot eliminations",
            Achievement::Unstoppable => "Get 20 kills without dying",
            Achievement::Gunsmith => "Fully customize a weapon with attachments",
            Achievement::Perfectionist => "Complete a mission without dying",
        }
    }
}

/// Tracks earned and in-progress achievements.
#[derive(Resource)]
pub struct AchievementTracker {
    pub earned: Vec<Achievement>,
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self { earned: Vec::new() }
    }
}

/// Checks for achievement unlocks when stats change.
pub fn achievement_checker(stats: Res<PlayerStats>, mut tracker: ResMut<AchievementTracker>) {
    // Check kill-based achievements
    if stats.kills >= 1 && !tracker.earned.contains(&Achievement::FirstBlood) {
        tracker.earned.push(Achievement::FirstBlood);
    }
    if stats.kills >= 20 && !tracker.earned.contains(&Achievement::Unstoppable) {
        tracker.earned.push(Achievement::Unstoppable);
    }
}
