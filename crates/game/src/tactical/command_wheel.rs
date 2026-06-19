use crate::squad::orders::{SquadOrder, SquadOrderMessage};
use bevy::prelude::*;
use socom_core::components::Player;

#[derive(Resource)]
pub struct CommandWheelState {
    pub open: bool,
    pub selected_index: usize,
}
impl Default for CommandWheelState {
    fn default() -> Self {
        Self {
            open: false,
            selected_index: 0,
        }
    }
}

const ORDERS: [(&str, SquadOrder); 4] = [
    ("MOVE", SquadOrder::HoldPosition),
    ("ENGAGE", SquadOrder::HoldPosition),
    ("SUPPRESS", SquadOrder::HoldPosition),
    ("REGROUP", SquadOrder::RegroupOnPlayer),
];

pub fn command_wheel_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel_state: ResMut<CommandWheelState>,
    mut commands: Commands,
    mut order_writer: bevy::ecs::message::MessageWriter<SquadOrderMessage>,
    player_query: Query<Entity, With<Player>>,
    wheel_ui_query: Query<Entity, With<CommandWheelUI>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        wheel_state.open = !wheel_state.open;
        if !wheel_state.open {
            for e in &wheel_ui_query {
                commands.entity(e).despawn();
            }
        }
    }
    for (i, (_, order)) in ORDERS.iter().enumerate() {
        let key = match i {
            0 => KeyCode::Digit1,
            1 => KeyCode::Digit2,
            2 => KeyCode::Digit3,
            3 => KeyCode::Digit4,
            _ => continue,
        };
        if wheel_state.open && keys.just_pressed(key) {
            wheel_state.selected_index = i;
            wheel_state.open = false;
            if let Ok(player_entity) = player_query.single() {
                order_writer.write(SquadOrderMessage {
                    order: order.clone(),
                    source: player_entity,
                });
            }
            for e in &wheel_ui_query {
                commands.entity(e).despawn();
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct CommandWheelUI;

pub fn command_wheel_ui_system(
    wheel_state: Res<CommandWheelState>,
    mut commands: Commands,
    wheel_ui_query: Query<Entity, With<CommandWheelUI>>,
) {
    if wheel_state.open && wheel_ui_query.is_empty() {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    ..default()
                },
                CommandWheelUI,
            ))
            .with_children(|p| {
                for (i, (name, _)) in ORDERS.iter().enumerate() {
                    let angle = i as f32 * std::f32::consts::TAU / 4.0;
                    let x = 80.0 * angle.cos();
                    let y = 80.0 * angle.sin();
                    p.spawn((
                        Text::new(*name),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.2)),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(100.0 + y - 10.0),
                            left: Val::Px(100.0 + x - 30.0),
                            ..default()
                        },
                        CommandWheelUI,
                    ));
                }
            });
    } else if !wheel_state.open && !wheel_ui_query.is_empty() {
        for e in &wheel_ui_query {
            commands.entity(e).despawn();
        }
    }
}
