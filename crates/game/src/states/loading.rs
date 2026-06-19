use bevy::prelude::*;

use crate::states::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_loading_screen);
        app.add_systems(OnExit(AppState::Loading), cleanup_loading_screen);
        app.add_systems(
            Update,
            check_loading_complete.run_if(in_state(AppState::Loading)),
        );
    }
}

#[derive(Component)]
struct LoadingUI;

#[derive(Component)]
struct LoadingBarFill;

fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((Camera2d, LoadingUI));
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            LoadingUI,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("SOCOM Tactical Shooter"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.7, 0.1)),
                LoadingUI,
            ));
            p.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                LoadingUI,
            ));
            // Progress bar background
            p.spawn((
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                LoadingUI,
            ));
            // Progress bar fill (animated)
            p.spawn((
                Node {
                    width: Val::Px(0.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.9, 0.7, 0.1)),
                LoadingBarFill,
                LoadingUI,
            ));
        });
}

fn cleanup_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn check_loading_complete(
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut fill_query: Query<&mut Node, With<LoadingBarFill>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(0.5, TimerMode::Once));
    timer.tick(time.delta());
    // Animate the progress bar
    let progress = (timer.elapsed_secs() / timer.duration().as_secs_f32()).min(1.0);
    for mut node in fill_query.iter_mut() {
        node.width = Val::Px(300.0 * progress);
    }
    if timer.just_finished() {
        next_state.set(AppState::InGame);
    }
}
