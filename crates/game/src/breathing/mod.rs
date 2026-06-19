use crate::stamina::Stamina;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use socom_core::components::Player;
use socom_input::actions::PlayerAction;

#[derive(Component, Debug)]
pub struct Breathing {
    pub holding: bool,
    pub hold_timer: f32,
    pub cooldown_timer: Timer,
    pub steadiness: f32,
}
impl Default for Breathing {
    fn default() -> Self {
        Self {
            holding: false,
            hold_timer: 0.0,
            cooldown_timer: Timer::from_seconds(2.0, TimerMode::Once),
            steadiness: 1.0,
        }
    }
}

pub fn breathing_system(
    time: Res<Time>,
    mut player_query: Query<
        (&ActionState<PlayerAction>, &mut Breathing, &mut Stamina),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let Ok((action, mut breath, mut stamina)) = player_query.single_mut() else {
        return;
    };
    let hold = action.pressed(&PlayerAction::Aim) && action.pressed(&PlayerAction::Sprint);
    breath.cooldown_timer.tick(time.delta());
    if hold && stamina.current > 5.0 && breath.cooldown_timer.just_finished() {
        breath.holding = true;
        breath.hold_timer += dt;
        stamina.current = (stamina.current - 8.0 * dt).max(0.0);
        breath.steadiness = 1.0 - (breath.hold_timer * 2.0).min(0.5);
    } else {
        if breath.holding {
            breath.cooldown_timer.reset();
        }
        breath.holding = false;
        breath.hold_timer = 0.0;
        breath.steadiness = 1.0;
    }
}

pub struct BreathingPlugin;
impl Plugin for BreathingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, breathing_system);
    }
}
