use crate::hud::elements::HudElement;
use crate::squad::orders::ActiveOrders;
use bevy::prelude::*;
use socom_core::components::{Health, Team};

#[derive(Component)]
pub struct SquadStatusText;

pub fn spawn_squad_status(mut commands: Commands) {
    commands.spawn((
        Text::new("SQUAD"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.8, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        SquadStatusText,
        HudElement,
    ));
}

pub fn update_squad_status_system(
    teammate_query: Query<(Entity, &Health, &Team)>,
    orders: Res<ActiveOrders>,
    mut text_query: Query<&mut Text, With<SquadStatusText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };
    let mut lines = String::from("SQUAD");
    lines.push('\n');
    for (entity, health, team) in &teammate_query {
        if *team == Team::Teammate {
            let status = if !health.is_alive() { "DEAD" } else { "OK" };
            let order = orders
                .orders
                .get(&entity)
                .map(|o| format!("{:?}", o))
                .unwrap_or("FOLLOW".into());
            lines.push_str(&format!(
                "  {status} - {order} [{}]\n",
                health.current as u32
            ));
        }
    }
    text.0 = lines;
}

#[derive(Component)]
pub struct ObjectiveText;

pub fn spawn_objective_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Objective: Eliminate all enemies"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.4)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(50.0),
            ..default()
        },
        Transform::from_xyz(-150.0, 0.0, 0.0),
        GlobalTransform::default(),
        ObjectiveText,
        HudElement,
    ));
}

pub fn update_objective_text_system(
    mission: Res<crate::missions::MissionState>,
    mut text_query: Query<&mut Text, With<ObjectiveText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };
    let mut lines = String::new();
    for obj in &mission.objectives {
        let status = if obj.completed {
            "[DONE]".to_string()
        } else {
            format!("[{}/{}]", obj.current_count, obj.target_count)
        };
        lines.push_str(&format!("{} {}\n", obj.objective_type.name(), status));
    }
    text.0 = lines;
}
