use bevy_lunex::prelude::*;
use bevy::prelude::*;

/// # Input Mouse Hover
/// Component that checks if cursor hovers over widget
#[derive(Component, Clone)]
pub struct InputMouseHover {
    pub hover: bool,
}
impl InputMouseHover {
    pub fn new() -> Self {
        InputMouseHover {
            hover: false,
        }
    }
}
pub (super) fn input_mouse_hover_system<T:Component + Default>(
    mut trees: Query<&mut UiTree<T>>,
    cursors: Query<&Cursor>,
    mut query: Query<(&Widget, &mut InputMouseHover)>,
) {
    for tree in &mut trees {
        for (widget, mut source) in &mut query {

            if !widget.fetch(&tree).unwrap().is_visible() {return;}

            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.position_world().invert_y()).unwrap() {
                    trigger = true;
                    break;
                }
            }

            source.hover = trigger;
        }
    }
}

/// # Input Mouse Click
/// Component that checks if widget was clicked on
#[derive(Component, Clone)]
pub struct InputMouseClick {
    pub left: bool,
    pub right: bool,
}
impl InputMouseClick {
    pub fn new() -> (InputMouseClick, InputMouseHover) {
        (
            InputMouseClick {
                left: false,
                right: false
            },
            InputMouseHover::new()
        )
    }
}
pub (super) fn input_mouse_click_system(
    mut query: Query<(&InputMouseHover, &mut InputMouseClick), With<Widget>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for (hover, mut source) in &mut query {
        if !hover.hover {return;}
        source.left = mouse_button_input.just_pressed(MouseButton::Left);
        source.right = mouse_button_input.just_pressed(MouseButton::Right);
    }
}