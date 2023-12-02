use crate::prelude::*;

/// # Lunex Input System Set
/// All systems dealing with input for:
/// * [`InputCursorHover`]
/// * [`InputMouseClick`]
/// 
/// Make sure you run your logic correspondingly to this set, if you use these components
#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub struct InputSystemSet;

/// # Input Cursor Hover
/// Component that checks if cursor hovers over widget
#[derive(Component, Clone)]
pub struct InputCursorHover {
    pub hover: bool,
    request_cursor_to: Option<usize>,
}
impl InputCursorHover {
    pub fn new() -> Self {
        InputCursorHover {
            hover: false,
            request_cursor_to: None,
        }
    }
    pub fn request_cursor(mut self, index: usize) -> Self {
        self.request_cursor_to = Some(index);
        self
    }
}
pub (super) fn input_mouse_hover_system<T:Component + Default>(
    mut trees: Query<&mut UiTree<T>>,
    mut cursors: Query<&mut Cursor>,
    mut query: Query<(&Widget, &mut InputCursorHover)>,
) {
    for tree in &mut trees {
        for (widget, mut source) in &mut query {

            if ! match widget.fetch(&tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.is_visible() {return;}

            let mut trigger = false;
            for mut cursor in &mut cursors {
                if widget.contains_position(&tree, &cursor.location_world().invert_y()).unwrap() {
                    trigger = true;
                    if let Some(index) = source.request_cursor_to {
                        cursor.request_cursor_index(index);
                    }
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
    /// # New
    /// WARINING: This component is not enough, requires additional components to work as expected. See `new_bundle()` method.
    pub fn new() -> InputMouseClick {
        InputMouseClick {
            left: false,
            right: false
        }
    }
    /// # New Bundle
    /// Returns a bundle of components, that are required for this component to work as expected
    pub fn new_bundle() -> (InputMouseClick, InputCursorHover) {
        (
            InputMouseClick {
                left: false,
                right: false
            },
            InputCursorHover::new()
        )
    }
}
pub (super) fn input_mouse_click_system(
    mut query: Query<(&InputCursorHover, &mut InputMouseClick), With<Widget>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for (hover, mut source) in &mut query {
        if !hover.hover {continue;}
        source.left = mouse_button_input.just_pressed(MouseButton::Left);
        source.right = mouse_button_input.just_pressed(MouseButton::Right);
    }
}