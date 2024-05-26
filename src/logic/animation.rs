use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{BevypunkColorPalette, LerpColor};


// #=========================#
// #=== EXPOSED COMPONENT ===#


/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
pub struct HoverControl {
    /// -1.0 backwards, 1.0 forward
    animation_direction: f32,
    /// Range from `0.0` to `1.0`
    animation_transition: f32,
    /// Range from `0.0` to `1.0`, animation_transition from last tick
    previous_transition: f32,
    /// Hover animation speed when transitioning to state
    pub animation_speed_forward: f32,
    /// Hover animation speed when transitioning back to default
    pub animation_speed_backward: f32,
}
impl HoverControl {
    /// Creates new struct
    pub fn new() -> Self {
        HoverControl {
            animation_direction: 0.0,
            animation_transition: 0.0,
            previous_transition: 0.0,
            animation_speed_backward: 8.0,
            animation_speed_forward: 8.0,
        }
    }
    /// Checks if animation is moving forward
    pub fn is_forward(&self) -> bool {
        self.animation_direction == 1.0
    }
    /// Checks if animation is currently transitioning
    pub fn is_changing(&self) -> bool {
        self.previous_transition != self.animation_transition
    }
}


/// Requests cursor on hover
#[derive(Component, Debug, Clone, PartialEq)]
pub struct HoverCursor {
    /// Cursor type to request on hover
    pub cursor: CursorIcon,
}
#[derive(Bundle)]
pub struct HoverCursorBundle {
    pub control: HoverControl,
    pub cursor: HoverCursor,
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that changes animation direction on hover
fn hover_enter_system(mut events: EventReader<Pointer<Over>>, mut query: Query<&mut HoverControl>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = 1.0;
        }
    }
}

/// System that changes animation direction on hover
fn hover_leave_system(mut events: EventReader<Pointer<Out>>, mut query: Query<&mut HoverControl>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = -1.0;
        }
    }
}

/// System that updates the hover transition
fn hover_update_system(time: Res<Time>, mut query: Query<&mut HoverControl>) {
    for mut control in &mut query {
        control.previous_transition = control.animation_transition;
        control.animation_transition += time.delta_seconds() * control.animation_direction * if control.animation_direction == 1.0 { control.animation_speed_forward } else { control.animation_speed_backward };
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);
    }
}

/// System that request cursor icon on hover
fn hover_cursor_request_system(query: Query<(&HoverControl, &HoverCursor)>, mut cursor: Query<&mut Cursor2d>) {
    for (control, hover_cursor) in &query {
        if control.is_forward() {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(hover_cursor.cursor, 1.0);
        }
    }
}

/* fn hover_update_system(time: Res<Time>, mut query: Query<&mut HoverControl>, mut set_color: EventWriter<SetColor>, mut set_layout: EventWriter<SetUiLayout>,) {
    for mut control in &mut query {
        if control.is_changing() {

            // Set the color from transition
            let color = Color::BEVYPUNK_RED.lerp(Color::BEVYPUNK_YELLOW.with_l(0.68), control.animation_transition);
            set_color.send(SetColor {
                target: control.image_entity,
                color: color.with_a(control.animation_transition),
            });
            set_color.send(SetColor {
                target: control.text_entity,
                color,
            });

            // Set the layout from transition
            set_layout.send(SetUiLayout {
                target: control.image_entity,
                layout: UiLayout::window_full().x(Rl(10.0 * control.animation_transition)).pack(),
            });
        }
    }
} */


// #====================#
// #=== LOGIC PLUGIN ===#

/// Plugin adding all our logic
pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hover_enter_system.run_if(on_event::<Pointer<Over>>()))
            .add_systems(Update, hover_leave_system.run_if(on_event::<Pointer<Out>>()))
            .add_systems(Update, hover_update_system)
            .add_systems(Update, hover_cursor_request_system);
    }
}