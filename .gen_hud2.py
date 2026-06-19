
import os
os.chdir("C:/Users/ssdsk/projects/SOCOM")
base = "crates/game/src/hud"

# stamina_bar.rs
with open(os.path.join(base, "stamina_bar.rs"), "w", encoding="utf-8") as f:
    f.write("""use bevy::prelude::*;
use socom_core::components::Player;
use crate::stamina::Stamina;
use crate::hud::elements::HudElement;

#[derive(Component)]
pub struct StaminaBarFill;

pub fn spawn_stamina_bar(commands: &mut Commands) {
    commands.spawn((
        Node { position_type: PositionType::Absolute, bottom: Val::Px(55.0), left: Val::Px(20.0), width: Val::Px(100.0), height: Val::Px(6.0), border: UiRect::all(Val::Px(1.0)), ..default() },
        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), BackgroundColor(Color::srgb(0.05, 0.05, 0.15)), HudElement,
    ));
    commands.spawn((
        Node { position_type: PositionType::Absolute, bottom: Val::Px(56.0), left: Val::Px(21.0), width: Val::Px(98.0), height: Val::Px(4.0), ..default() },
        BackgroundColor(Color::srgb(0.2, 0.5, 1.0)), StaminaBarFill, HudElement,
    ));
}

pub fn update_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut fill_query: Query<&mut Node, With<StaminaBarFill>>,
) {
    let Ok(stamina) = player_query.single() else { return; };
    let ratio = stamina.ratio();
    for mut node in fill_query.iter_mut() { node.width = Val::Px(98.0 * ratio.clamp(0.0, 1.0)); }
}
""")
print("stamina_bar created")

# achievement_popup.rs
with open(os.path.join(base, "achievement_popup.rs"), "w", encoding="utf-8") as f:
    f.write("""use bevy::prelude::*;
use crate::messages::AchievementUnlockMessage;

#[derive(Component)]
pub struct AchievementPopup;

pub fn achievement_popup_system(
    mut ach_reader: bevy::ecs::message::MessageReader<AchievementUnlockMessage>,
    mut commands: Commands,
    popup_query: Query<Entity, With<AchievementPopup>>,
) {
    for msg in ach_reader.read() {
        for e in &popup_query { commands.entity(e).despawn(); }
        commands.spawn((
            Text::new(format!("Achievement: {}", msg.achievement)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(1.0, 0.6, 0.0)),
            Node { position_type: PositionType::Absolute, top: Val::Percent(12.0), left: Val::Percent(50.0), ..default() },
            Transform::from_xyz(-100.0, 0.0, 0.0), GlobalTransform::default(), AchievementPopup,
        ));
    }
}

pub fn ach_popup_lifetime_system(
    time: Res<Time>, mut commands: Commands,
    query: Query<Entity, With<AchievementPopup>>,
    mut timer: Local<Option<Timer>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(4.0, TimerMode::Once));
    timer.tick(time.delta());
    if timer.just_finished() { for e in &query { commands.entity(e).despawn(); } }
}
""")
print("achievement_popup created")

# kill_feed.rs
with open(os.path.join(base, "kill_feed.rs"), "w", encoding="utf-8") as f:
    f.write("""use bevy::prelude::*;
use crate::combat::death::DeathMessage;
use socom_core::components::{Player, Team};

#[derive(Component)]
pub struct KillFeedEntry;

pub fn kill_feed_system(
    mut death_reader: bevy::ecs::message::MessageReader<DeathMessage>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    team_query: Query<&Team>,
) {
    let Ok(player_entity) = player_query.single() else { return; };
    for msg in death_reader.read() {
        let text = if msg.source == Some(player_entity) {
            let name = team_query.get(msg.entity).map(|t| match t { Team::Enemy => "Enemy", _ => "Target" }).unwrap_or("Unknown");
            format!("Killed {}", name)
        } else if msg.entity == player_entity { "You died".to_string() }
        else { continue; };
        commands.spawn((
            Text::new(text), TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node { position_type: PositionType::Absolute, top: Val::Px(10.0), right: Val::Px(10.0), ..default() }, KillFeedEntry,
        ));
    }
}

pub fn kill_feed_cleanup_system(
    time: Res<Time>, mut commands: Commands,
    query: Query<(Entity, &KillFeedEntry)>,
    mut timers: Local<Vec<(Entity, Timer)>>,
) {
    timers.retain(|(entity, timer)| {
        timer.tick(time.delta());
        if timer.just_finished() { commands.entity(*entity).despawn(); false } else { true }
    });
    for (entity, _) in &query {
        if !timers.iter().any(|(e, _)| *e == entity) { timers.push((entity, Timer::from_seconds(5.0, TimerMode::Once))); }
    }
}
""")
print("kill_feed created")

# squad_status.rs
with open(os.path.join(base, "squad_status.rs"), "w", encoding="utf-8") as f:
    f.write("""use bevy::prelude::*;
use socom_core::components::{Health, Team};
use crate::hud::elements::HudElement;
use crate::squad::orders::ActiveOrders;

#[derive(Component)]
pub struct SquadStatusText;

pub fn spawn_squad_status(commands: &mut Commands) {
    commands.spawn((
        Text::new("SQUAD"), TextFont { font_size: 12.0, ..default() },
        TextColor(Color::srgb(0.6, 0.8, 1.0)),
        Node { position_type: PositionType::Absolute, top: Val::Px(10.0), left: Val::Px(10.0), ..default() },
        SquadStatusText, HudElement,
    ));
}

pub fn update_squad_status_system(
    teammate_query: Query<(Entity, &Health, &Team)>,
    orders: Res<ActiveOrders>,
    mut text_query: Query<&mut Text, With<SquadStatusText>>,
) {
    let Ok(mut text) = text_query.single_mut() else { return; };
    let mut lines = String::from("SQUAD"); lines.push('\n');
    for (entity, health, team) in &teammate_query {
        if *team == Team::Teammate {
            let status = if !health.is_alive() { "DEAD" } else { "OK" };
            let order = orders.orders.get(&entity).map(|o| format!("{:?}", o)).unwrap_or("FOLLOW".into());
            lines.push_str(&format!("  {status} - {order} [{}]\n", health.current as u32));
        }
    }
    text.0 = lines;
}

#[derive(Component)]
pub struct ObjectiveText;

pub fn spawn_objective_text(commands: &mut Commands) {
    commands.spawn((
        Text::new("Objective: Eliminate all enemies"), TextFont { font_size: 14.0, ..default() },
        TextColor(Color::srgb(0.9, 0.9, 0.4)),
        Node { position_type: PositionType::Absolute, top: Val::Px(5.0), left: Val::Percent(50.0), ..default() },
        Transform::from_xyz(-150.0, 0.0, 0.0), GlobalTransform::default(), ObjectiveText, HudElement,
    ));
}

pub fn update_objective_text_system(
    mission: Res<crate::missions::MissionState>,
    mut text_query: Query<&mut Text, With<ObjectiveText>>,
) {
    let Ok(mut text) = text_query.single_mut() else { return; };
    let mut lines = String::new();
    for obj in &mission.objectives {
        let status = if obj.completed { "[DONE]" } else { format!("[{}/{}]", obj.current_count, obj.target_count) };
        lines.push_str(&format!("{} {}\n", obj.objective_type.name(), status));
    }
    text.0 = lines;
}
""")
print("squad_status created")

print("ALL HUD FILES CREATED")
