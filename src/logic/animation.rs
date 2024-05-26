use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::LerpColor;


// #=========================#
// #=== EXPOSED COMPONENT ===#


/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Hover {
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
impl Hover {
    /// Creates new struct
    pub fn new() -> Self {
        Hover {
            animation_direction: 0.0,
            animation_transition: 0.0,
            previous_transition: 0.0,
            animation_speed_backward: 8.0,
            animation_speed_forward: 8.0,
        }
    }
    /// Replaces the forward_speed with a new value.
    pub fn forward_speed(mut self, speed: f32) -> Self {
        self.animation_speed_forward = speed;
        self
    }
    /// Replaces the backward_speed with a new value.
    pub fn backward_speed(mut self, speed: f32) -> Self {
        self.animation_speed_backward = speed;
        self
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


/// Requests cursor icon on hover
#[derive(Component, Debug, Clone, PartialEq)]
pub struct HoverCursor {
    /// Cursor type to request on hover
    pub cursor: CursorIcon,
}
impl HoverCursor {
    /// Creates new struct
    pub fn new(cursor: CursorIcon) -> Self {
        HoverCursor {
            cursor
        }
    }
}


/// Changes color of entities on cursor hover
#[derive(Component, Clone, PartialEq)]
pub struct HoverColor {
    /// Should the change be applied to self
    pub itself: bool,
    /// All entities to change color of (with optional color override)
    pub entity: Vec<(Entity, Option<Color>)>,
    /// The color to change into
    pub color: Color,
}
impl HoverColor {
    /// Creates new struct
    pub fn new(color: Color) -> Self {
        HoverColor {
            itself: true,
            entity: Vec::new(),
            color,
        }
    }
    /// Replaces the itself boolean with a new value. (If the change should be applied to self)
    pub fn itself(mut self, itself: bool) -> Self {
        self.itself = itself;
        self
    }
    /// Replaces the entity list with a new one. (All entities that should change color)
    pub fn entity(mut self, entities: Vec<(Entity, Option<Color>)>) -> Self {
        self.entity = entities;
        self
    }
}


/// Default base color component
#[derive(Component, Debug, Clone, PartialEq)]
pub struct BaseColor {
    /// The base color
    pub color: Color,
}
impl BaseColor {
    /// Creates new struct
    pub fn new(color: Color) -> Self {
        BaseColor {
            color,
        }
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that changes animation direction on hover
fn hover_enter_system(mut events: EventReader<Pointer<Over>>, mut query: Query<&mut Hover>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = 1.0;
        }
    }
}

/// System that changes animation direction on hover
fn hover_leave_system(mut events: EventReader<Pointer<Out>>, mut query: Query<&mut Hover>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = -1.0;
        }
    }
}

/// System that updates the hover transition
fn hover_update_system(time: Res<Time>, mut query: Query<&mut Hover>) {
    for mut control in &mut query {
        control.previous_transition = control.animation_transition;
        control.animation_transition += time.delta_seconds() * control.animation_direction * if control.animation_direction == 1.0 { control.animation_speed_forward } else { control.animation_speed_backward };
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);
    }
}

/// System that request cursor icon on hover
fn hover_cursor_request_system(query: Query<(&Hover, &HoverCursor)>, mut cursor: Query<&mut Cursor2d>) {
    for (control, hover_cursor) in &query {
        if control.is_forward() {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(hover_cursor.cursor, 1.0);
        }
    }
}

/// System that sends color change events on hover
fn hover_color_update_system(query: Query<(&Hover, &BaseColor, &HoverColor, Entity)>, mut set_color: EventWriter<SetColor>) {
    for (hover, basecolor, hovercolor, entity) in &query {
        if hover.is_changing() {

            let color = basecolor.color.lerp(hovercolor.color, hover.animation_transition);

            if hovercolor.itself {
                set_color.send(SetColor {
                    target: entity,
                    color,
                });
            }

            for (e, overwrite) in &hovercolor.entity {
                set_color.send(SetColor {
                    target: *e,
                    color: if let Some(c) = overwrite { c.lerp(hovercolor.color, hover.animation_transition) } else { color },
                });
            }
        }
    }
}

/* fn hover_update_layout_system(time: Res<Time>, mut query: Query<&mut Hover>, mut set_color: EventWriter<SetColor>, mut set_layout: EventWriter<SetUiLayout>,) {
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
            .add_systems(Update, hover_cursor_request_system)
            .add_systems(Update, hover_color_update_system);
    }
}