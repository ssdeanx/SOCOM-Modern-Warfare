use bevy::prelude::*;

pub mod ingame;
pub mod loading;
pub mod main_menu;

/// Application states for the game flow.
#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    /// Asset loading screen
    Loading,
    /// Active gameplay
    InGame,
}
