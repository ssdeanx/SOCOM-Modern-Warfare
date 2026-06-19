use bevy::prelude::*;
use ron::ser::PrettyConfig;
use std::path::PathBuf;

use crate::gear::inventory::PlayerInventory;
use crate::progression::achievements::Achievement;
use crate::progression::stats::PlayerStats;
use crate::progression::xp::PlayerProgression;

/// Complete serializable save data for the game.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SaveData {
    pub version: u32,
    pub player_name: String,
    pub progression: SaveProgression,
    pub inventory: SaveInventory,
    pub achievements: Vec<String>,
    pub settings: SaveSettings,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SaveProgression {
    pub xp: u64,
    pub level: u32,
    pub total_xp: u64,
    pub kills: u32,
    pub deaths: u32,
    pub shots_fired: u32,
    pub shots_hit: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SaveInventory {
    pub credits: u64,
    pub equipped_ids: Vec<Option<String>>,
    pub stash_ids: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SaveSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub sensitivity: f32,
    pub invert_y: bool,
    pub fullscreen: bool,
    pub resolution: (u32, u32),
}

/// Manages save/load operations.
pub struct SaveManager;

impl SaveManager {
    fn save_path() -> PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        let mut path = PathBuf::from(home);
        path.push(".socom");
        let _ = std::fs::create_dir_all(&path);
        path.push("save.ron");
        path
    }

    pub fn save(
        progression: &PlayerProgression,
        stats: &PlayerStats,
        inventory: &PlayerInventory,
        achievements: &[Achievement],
    ) -> Result<(), String> {
        let data = SaveData {
            version: 1,
            player_name: "Operator".into(),
            progression: SaveProgression {
                xp: progression.xp,
                level: progression.level,
                total_xp: progression.total_xp_earned,
                kills: stats.kills,
                deaths: stats.deaths,
                shots_fired: stats.shots_fired,
                shots_hit: stats.shots_hit,
                damage_dealt: stats.damage_dealt,
                damage_taken: stats.damage_taken,
            },
            inventory: SaveInventory {
                credits: inventory.credits,
                equipped_ids: inventory
                    .equipped
                    .iter()
                    .map(|e| e.as_ref().map(|i| i.id.clone()))
                    .collect(),
                stash_ids: inventory.stash.iter().map(|i| i.id.clone()).collect(),
            },
            achievements: achievements.iter().map(|a| format!("{:?}", a)).collect(),
            settings: SaveSettings {
                master_volume: 0.8,
                sfx_volume: 1.0,
                music_volume: 0.5,
                sensitivity: 1.0,
                invert_y: false,
                fullscreen: false,
                resolution: (1280, 720),
            },
        };
        let path = Self::save_path();
        let content = ron::ser::to_string_pretty(&data, PrettyConfig::default())
            .map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load() -> Result<SaveData, String> {
        let path = Self::save_path();
        if !path.exists() {
            return Err("No save file".into());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        ron::from_str::<SaveData>(&content).map_err(|e| e.to_string())
    }
}

/// System that auto-saves on certain triggers.
pub fn auto_save_system(
    progression: Res<PlayerProgression>,
    stats: Res<PlayerStats>,
    inventory: Res<PlayerInventory>,
    achievements: Res<crate::progression::achievements::AchievementTracker>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F5) {
        let _ = SaveManager::save(&progression, &stats, &inventory, &achievements.earned);
    }
}

/// System that handles F9 load key.
pub fn quick_load_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut progression: ResMut<PlayerProgression>,
    mut stats: ResMut<PlayerStats>,
    mut inventory: ResMut<PlayerInventory>,
    mut achievements: ResMut<crate::progression::achievements::AchievementTracker>,
) {
    if !keys.just_pressed(KeyCode::F9) {
        return;
    }
    if let Ok(data) = SaveManager::load() {
        progression.xp = data.progression.xp;
        progression.level = data.progression.level;
        progression.total_xp_earned = data.progression.total_xp;
        stats.kills = data.progression.kills;
        stats.deaths = data.progression.deaths;
        stats.shots_fired = data.progression.shots_fired;
        stats.shots_hit = data.progression.shots_hit;
        stats.damage_dealt = data.progression.damage_dealt;
        stats.damage_taken = data.progression.damage_taken;
    }
}
