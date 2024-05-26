use crate::*;


// #==================#
// #=== COMPONENTS ===#


/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Hover {
    /// -1.0 backwards, 1.0 forward
    animation_direction: f32,
    /// Range from `0.0` to `1.0`
    animation_transition: f32,
    /// Range from `0.0` to `1.0`, animation_transition from last tick
    previous_transition: f32,
    /// Setting this to true will disable logic with intention that something else will pipe the control data instead
    pub receiver: bool,
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
            receiver: false,
            animation_speed_backward: 8.0,
            animation_speed_forward: 8.0,
        }
    }
    /// Marks this hover as receiver
    pub fn receiver(mut self, receiver: bool) -> Self {
        self.receiver = receiver;
        self
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

/// This struct synchronizes different entities hover state.
/// It takes corresponding [`Hover`] and pipes it into specified entities.
#[derive(Component, Clone, PartialEq)]
pub struct HoverPipe {
    /// All entities to to pipe hover state control data to
    pub entity: Vec<Entity>,
}
impl HoverPipe {
    /// Creates new struct
    pub fn new(entity: Vec<Entity>) -> Self {
        HoverPipe {
            entity,
        }
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

/// Changes color of entity on cursor hover
#[derive(Component, Clone, PartialEq)]
pub struct HoverColor {
    /// The color to change into
    pub color: Color,
}
impl HoverColor {
    /// Creates new struct
    pub fn new(color: Color) -> Self {
        HoverColor {
            color,
        }
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


// #==============#
// #=== EVENTS ===#

/// This event will override hover transition state of targetted entity
#[derive(Event, PartialEq, Clone, Copy)]
pub struct SetHoverTransition {
    pub target: Entity,
    pub transition: f32,
}
fn apply_event_set_hover_transition(mut events: EventReader<SetHoverTransition>, mut query: Query<&mut Hover>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            if hover.animation_transition != event.transition {
                hover.animation_transition = event.transition
            }
        }
    }
}


// #=====================#
// #=== INTERACTIVITY ===#


// #=== Core systems

/// System that updates the hover transition
fn hover_update_system(time: Res<Time>, mut query: Query<&mut Hover>) {
    for mut control in &mut query {
        control.previous_transition = control.animation_transition;
        if control.receiver { continue }
        control.animation_transition += time.delta_seconds() * control.animation_direction * if control.animation_direction == 1.0 { control.animation_speed_forward } else { control.animation_speed_backward };
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);
    }
}

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


// #=== Piping systems

/// System that sends color change events on hover
fn hover_pipe_update_system(query: Query<(&Hover, &HoverPipe)>, mut event: EventWriter<SetHoverTransition>) {
    for (hover, pipe) in &query {
        if hover.is_changing() {
            for e in &pipe.entity {
                event.send(SetHoverTransition {
                    target: *e,
                    transition: hover.animation_transition,
                });
            }
        }
    }
}


// #=== Styling systems

/// System that sends color change events on hover
fn hover_color_update_system(query: Query<(&Hover, &BaseColor, &HoverColor, Entity)>, mut set_color: EventWriter<SetColor>) {
    for (hover, basecolor, hovercolor, entity) in &query {
        if hover.is_changing() {
            let color = basecolor.color.lerp(hovercolor.color, hover.animation_transition);
            set_color.send(SetColor {
                target: entity,
                color,
            });
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

/// System that request cursor icon on hover
fn hover_cursor_request_system(query: Query<(&Hover, &HoverCursor)>, mut cursor: Query<&mut Cursor2d>) {
    for (control, hover_cursor) in &query {
        if control.is_forward() {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(hover_cursor.cursor, 1.0);
        }
    }
}


// #====================#
// #=== HOVER PLUGIN ===#

/// Plugin adding all our logic
pub struct HoverPlugin;
impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add our event
            .add_event::<SetHoverTransition>()
            .add_systems(Update, apply_event_set_hover_transition.run_if(on_event::<SetHoverTransition>()))

            // Core systems
            .add_systems(Update, hover_update_system)
            .add_systems(Update, hover_enter_system.run_if(on_event::<Pointer<Over>>()))
            .add_systems(Update, hover_leave_system.run_if(on_event::<Pointer<Out>>()))

            // Piping system
            .add_systems(Update, hover_pipe_update_system)

            // Styling systems
            .add_systems(Update, hover_color_update_system.before(hover_update_system))
            .add_systems(Update, hover_cursor_request_system);
    }
}