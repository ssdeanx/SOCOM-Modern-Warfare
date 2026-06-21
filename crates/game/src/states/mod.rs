use bevy::prelude::*;

pub mod ingame;
pub mod loading;
pub mod main_menu;

/// Application states for the game flow.
#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    /// Asset loading / splash screen
    Loading,
    /// Active gameplay (both campaign and training)
    InGame,
}

/// Which game mode is active — Campaign (the main story) or Training.
/// Set before transitioning to `InGame`.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameMode {
    #[default]
    Campaign,
    Training,
}
