use std::collections::VecDeque;
use bevy::input::mouse::MouseMotion;

use crate::*;

pub const SMOOTH_SAMPLES_LEN: usize = 3;

#[derive(Resource)]
pub struct MouseCapture {
    focus: bool,
    pub delta: Vec2,
    delta_buffer: VecDeque<Vec2>,
}
impl Default for MouseCapture {
    fn default() -> Self {
        MouseCapture {
            focus: false,
            delta: Default::default(),
            delta_buffer: Default::default(),
        }
    }
}
fn update_mouse_capture(mut mouse_motion_events: EventReader<MouseMotion>, mut capture: ResMut<MouseCapture>) {
    let mut delta: Vec2 = mouse_motion_events.read().map(|e| e.delta).sum();
    if !capture.focus { delta = Vec2::ZERO }
    while capture.delta_buffer.len() >= SMOOTH_SAMPLES_LEN { capture.delta_buffer.pop_front(); }
    capture.delta_buffer.push_back(delta);
    capture.delta = capture.delta_buffer.iter().fold(Vec2::ZERO, |sum, &vec| sum + vec) / capture.delta_buffer.len() as f32;
}
fn switch_mouse_capture_focus(keyboard_input: Res<ButtonInput<KeyCode>>, mut capture: ResMut<MouseCapture>,
    mut event1: EventWriter<actions::HideCursor2d>,
    mut event2: EventWriter<actions::ConfineCursor2d>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        capture.focus = !capture.focus;
        event1.send(actions::HideCursor2d(capture.focus));
        event2.send(actions::ConfineCursor2d(capture.focus));
    }
}


// #=====================#
// #=== MODULE PLUGIN ===#

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MouseCapture::default())
            .add_systems(Update, (update_mouse_capture, switch_mouse_capture_focus));
    }
}
