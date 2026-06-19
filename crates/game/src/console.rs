use bevy::ecs::message::MessageReader;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ConsoleState {
    pub open: bool,
    pub input: String,
    pub history: Vec<String>,
    pub history_index: usize,
    pub output: Vec<String>,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            open: false,
            input: String::new(),
            history: Vec::new(),
            history_index: 0,
            output: vec![
                "SOCOM Dev Console".into(),
                "Type 'help' for commands.".into(),
            ],
        }
    }
}

impl ConsoleState {
    pub fn execute(&mut self) {
        let cmd = self.input.trim().to_string();
        if cmd.is_empty() {
            return;
        }
        self.history.push(cmd.clone());
        self.history_index = self.history.len();
        let response = match cmd.split_whitespace().next().unwrap_or("") {
            "help" => "Available: help, god, noclip, spawn, killall, timescale, tp".into(),
            "god" => "God mode toggled".into(),
            "noclip" => "Noclip toggled".into(),
            "killall" => "All enemies killed".into(),
            "timescale" => format!(
                "Timescale: {}",
                cmd.strip_prefix("timescale ")
                    .unwrap_or("usage: timescale [0.1-10]")
            ),
            "spawn" => "Spawned".into(),
            "tp" => "Teleported".into(),
            _ => format!("Unknown: '{}'", cmd),
        };
        self.output.push(format!("> {cmd}"));
        self.output.push(response);
        self.input.clear();
    }
}

#[derive(Component)]
struct ConsoleUI;
#[derive(Component)]
struct ConsoleOutputText;
#[derive(Component)]
struct ConsoleInputText;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleState::default());
        app.add_systems(Update, (console_toggle_system, console_input_system));
    }
}

fn console_toggle_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ConsoleState>,
    mut commands: Commands,
    ui_query: Query<Entity, With<ConsoleUI>>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        state.open = !state.open;
        if state.open && ui_query.is_empty() {
            spawn_console_ui(&mut commands);
        } else if !state.open {
            for e in &ui_query {
                commands.entity(e).despawn();
            }
        }
    }
}

fn console_input_system(
    mut state: ResMut<ConsoleState>,
    mut evr_kbd: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut output_query: Query<&mut Text, With<ConsoleOutputText>>,
    mut input_query: Query<&mut Text, (With<ConsoleInputText>, Without<ConsoleOutputText>)>,
) {
    if !state.open {
        return;
    }

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    for ev in evr_kbd.read() {
        if ev.state == bevy::input::ButtonState::Pressed {
            match ev.key_code {
                KeyCode::Enter => state.execute(),
                KeyCode::Backspace => {
                    state.input.pop();
                }
                KeyCode::Escape => {}
                k => {
                    if let Some(c) = keycode_to_char(k, shift) {
                        state.input.push(c);
                    }
                }
            }
        }
    }

    if let Ok(mut t) = output_query.single_mut() {
        let n = state.output.len();
        let start = n.saturating_sub(10);
        t.0 = state.output[start..].join(
            "
",
        );
    }
    if let Ok(mut t) = input_query.single_mut() {
        t.0 = format!("> {}_", state.input);
    }
}

fn keycode_to_char(k: KeyCode, shift: bool) -> Option<char> {
    Some(match (k, shift) {
        (KeyCode::Space, _) => ' ',
        (KeyCode::Minus, _) => '-',
        (KeyCode::Equal, s) => {
            if s {
                '+'
            } else {
                '='
            }
        }
        (KeyCode::KeyA, s) => {
            if s {
                'A'
            } else {
                'a'
            }
        }
        (KeyCode::KeyB, s) => {
            if s {
                'B'
            } else {
                'b'
            }
        }
        (KeyCode::KeyC, s) => {
            if s {
                'C'
            } else {
                'c'
            }
        }
        (KeyCode::KeyD, s) => {
            if s {
                'D'
            } else {
                'd'
            }
        }
        (KeyCode::KeyE, s) => {
            if s {
                'E'
            } else {
                'e'
            }
        }
        (KeyCode::KeyF, s) => {
            if s {
                'F'
            } else {
                'f'
            }
        }
        (KeyCode::KeyG, s) => {
            if s {
                'G'
            } else {
                'g'
            }
        }
        (KeyCode::KeyH, s) => {
            if s {
                'H'
            } else {
                'h'
            }
        }
        (KeyCode::KeyI, s) => {
            if s {
                'I'
            } else {
                'i'
            }
        }
        (KeyCode::KeyJ, s) => {
            if s {
                'J'
            } else {
                'j'
            }
        }
        (KeyCode::KeyK, s) => {
            if s {
                'K'
            } else {
                'k'
            }
        }
        (KeyCode::KeyL, s) => {
            if s {
                'L'
            } else {
                'l'
            }
        }
        (KeyCode::KeyM, s) => {
            if s {
                'M'
            } else {
                'm'
            }
        }
        (KeyCode::KeyN, s) => {
            if s {
                'N'
            } else {
                'n'
            }
        }
        (KeyCode::KeyO, s) => {
            if s {
                'O'
            } else {
                'o'
            }
        }
        (KeyCode::KeyP, s) => {
            if s {
                'P'
            } else {
                'p'
            }
        }
        (KeyCode::KeyQ, s) => {
            if s {
                'Q'
            } else {
                'q'
            }
        }
        (KeyCode::KeyR, s) => {
            if s {
                'R'
            } else {
                'r'
            }
        }
        (KeyCode::KeyS, s) => {
            if s {
                'S'
            } else {
                's'
            }
        }
        (KeyCode::KeyT, s) => {
            if s {
                'T'
            } else {
                't'
            }
        }
        (KeyCode::KeyU, s) => {
            if s {
                'U'
            } else {
                'u'
            }
        }
        (KeyCode::KeyV, s) => {
            if s {
                'V'
            } else {
                'v'
            }
        }
        (KeyCode::KeyW, s) => {
            if s {
                'W'
            } else {
                'w'
            }
        }
        (KeyCode::KeyX, s) => {
            if s {
                'X'
            } else {
                'x'
            }
        }
        (KeyCode::KeyY, s) => {
            if s {
                'Y'
            } else {
                'y'
            }
        }
        (KeyCode::KeyZ, s) => {
            if s {
                'Z'
            } else {
                'z'
            }
        }
        (KeyCode::Digit0, _) => '0',
        (KeyCode::Digit1, _) => '1',
        (KeyCode::Digit2, _) => '2',
        (KeyCode::Digit3, _) => '3',
        (KeyCode::Digit4, _) => '4',
        (KeyCode::Digit5, _) => '5',
        (KeyCode::Digit6, _) => '6',
        (KeyCode::Digit7, _) => '7',
        (KeyCode::Digit8, _) => '8',
        (KeyCode::Digit9, _) => '9',
        (KeyCode::Period, _) => '.',
        (KeyCode::Comma, _) => ',',
        (KeyCode::Slash, _) => '/',
        (KeyCode::Semicolon, _) => ';',
        _ => return None,
    })
}

fn spawn_console_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(40.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            ConsoleUI,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(85.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                ConsoleOutputText,
            ));
            p.spawn((
                Text::new("> _"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(15.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                ConsoleInputText,
            ));
        });
}
