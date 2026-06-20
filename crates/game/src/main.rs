mod ai;
mod ammo_type;
mod breathing;
mod camera_control;
mod combat;
mod console;
mod controls;
mod drones;
mod feedback;
mod gear;
mod hud;
mod level;
mod menu;
mod messages;
mod missions;
mod pause;
mod physics;
mod player;
mod progression;
mod save_load;
mod settings;
mod settings_applier;
mod squad;
mod stamina;
mod states;
mod tactical;
mod weapon_handling;
mod weapons;

use avian3d::prelude::*;
use bevy::prelude::*;

use socom_audio::AudioPlugin as SocomAudioPlugin;
use socom_input::InputPlugin;
use socom_rendering::camera::CameraPlugin;
use states::{ingame::InGamePlugin, loading::LoadingPlugin, main_menu::MainMenuPlugin, AppState};

use crate::breathing::BreathingPlugin;
use crate::combat::RespawnState;
use crate::drones::DronePlugin;
use crate::gear::GearPlugin;
use crate::missions::MissionPlugin;
use crate::progression::ProgressionPlugin;

fn main() {
    let mut app = App::new();

    // ── Core Plugins ──
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SOCOM Tactical Shooter".into(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),
    ));

    // ── Dev Tools ──
    app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default());
    // Runtime entity inspector (press F1 to open)
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    // ── GPU Particles ──
    app.add_plugins(bevy_hanabi::HanabiPlugin);

    // ── Post Processing ──
    app.add_plugins(socom_rendering::post_processing::PostProcessingPlugin);

    // ── Custom Asset Formats (for loading RON/JSON level files) ──
    // Disabled until we have actual asset types to load
    // app.add_plugins(bevy_common_assets::ron::RonAssetPlugin::<ron::Value>::new(&["ron"]));

    // ── Core Resources ──
    app.insert_resource(RespawnState::default());
    // Fixed timestep for physics: 120 Hz for smooth tactical shooter feel
    app.insert_resource(bevy::time::Time::new_with(bevy::time::Timer::from_seconds(
        1.0 / 120.0,
        bevy::time::TimerMode::Repeating,
    )));

    // ─── Register All Messages ───
    app.add_message::<messages::WeaponFiredMessage>();
    app.add_message::<messages::PlayerDamagedMessage>();
    app.add_message::<messages::HitConfirmedMessage>();
    app.add_message::<messages::XpGainedMessage>();
    app.add_message::<messages::LevelUpMessage>();
    app.add_message::<messages::AchievementUnlockMessage>();
    app.add_message::<messages::SquadStatusMessage>();
    app.add_message::<messages::CoverStateMessage>();
    app.add_message::<messages::SuppressionMessage>();
    app.add_message::<messages::ItemPickupMessage>();
    app.add_message::<messages::ItemEquipMessage>();
    app.add_message::<messages::EquipmentUsedMessage>();
    app.add_message::<messages::GrenadeDetonatedMessage>();
    app.add_message::<messages::MeleeHitMessage>();

    // ── Internal Plugins ──
    app.add_plugins((
        InputPlugin,
        CameraPlugin,
        SocomAudioPlugin,
        pause::PausePlugin,
        camera_control::CameraControlPlugin,
        feedback::FeedbackPlugin,
        settings::SettingsPlugin,
        console::ConsolePlugin,
    ));

    // ── State Machine ──
    app.init_state::<AppState>();
    // ── Additional Global Systems ──
    app.add_systems(
        Update,
        (
            save_load::auto_save_system,
            save_load::quick_load_system,
            settings_applier::apply_settings_system,
            stamina::stamina_system,
            weapon_handling::weapon_handling_system,
            socom_rendering::post_processing::apply_post_processing_system,
        ),
    );

    // ── Additional Direct Plugins ──
    app.add_plugins((
        DronePlugin,
        BreathingPlugin,
        MissionPlugin,
        ProgressionPlugin,
        GearPlugin,
    ));

    app.add_plugins((MainMenuPlugin, LoadingPlugin, InGamePlugin));

    app.run();
}
