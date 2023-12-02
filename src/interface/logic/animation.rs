use crate::prelude::*;
use mathio::tween;


/// # Animate Control
/// Holds control values that other animation components use.
#[derive(Component, Clone)]
pub struct AnimateControl {
    /// If true, `value` will move to 1.0, else it will move to 0.0
    pub trigger: bool,
    /// The value representing state of the animation. Range: 0.0 - 1.0
    pub value: f32,
}
impl AnimateControl {
    pub fn new() -> Self {
        AnimateControl {
            trigger: false,
            value: 0.0,
        }
    }
}
pub (super) fn animate_system(mut query: Query<&mut AnimateControl>) {
    for mut source in &mut query {
        if source.trigger {
            if source.value < 1.0 {source.value += 0.25} else {source.value = 1.0}
        } else {
            if source.value > 0.0 {source.value -= 0.05} else {source.value = 0.0}
        }
    }
}

/// # Animate Window Position
/// Takes control values from [`AnimateControl`] component and updates
/// window position. Will panic if the container is not a window layout.
/// ## Requires:
/// * [`lg::AnimateControl`]
#[derive(Component, Clone)]
pub struct AnimateWindowPosition {
    pub pos1: Vec2,
    pub pos2: Vec2,
}
impl AnimateWindowPosition {
    pub fn new(pos1: Vec2, pos2: Vec2) -> Self {
        AnimateWindowPosition {
            pos1,
            pos2
        }
    }
}
pub (super) fn animate_window_position_system<T:Component + Default>(mut trees: Query<&mut UiTree<T>>, query: Query<(&Widget, &AnimateWindowPosition, &AnimateControl)>) {
    for mut tree in &mut trees {
        for (widget, source1, source2) in &query {

            let container = match widget.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_container_mut();

            let window = container.get_layout_mut().expect_window_mut();
            window.pos_relative.x = tween(source1.pos1.x, source1.pos2.x, source2.value);
            window.pos_relative.y = tween(source1.pos1.y, source1.pos2.y, source2.value);

        }
    }
}

/// # Animate Color
/// Takes control values from [`AnimateControl`] component and updates
/// color values of image and text.
/// ## Requires:
/// * [`lg::AnimateControl`]
#[derive(Component, Clone)]
pub struct AnimateColor {
    color1: Color,
    color2: Color,
}
impl AnimateColor {
    pub fn new(color1: Color, color2: Color) -> Self {
        AnimateColor {
            color1,
            color2,
        }
    }
}
pub (super) fn animate_color_text_system(mut query: Query<(&mut Text, &AnimateColor, &AnimateControl)>) {
    for (mut text, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        text.sections[0].style.color = color;
    }
}
pub (super) fn animate_color_image_system(mut query: Query<(&mut Sprite, &AnimateColor, &AnimateControl)>) {
    for (mut sprite, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        sprite.color = color;
    }
}


/// # Animate Mouse Input
/// Updates values from [`AnimateControl`] component by values from [`lg::InputCursorHover`]
/// ## Requires:
/// * [`lg::AnimateControl`]
/// * [`lg::InputCursorHover`]
#[derive(Component, Clone)]
pub struct AnimateCursorInput;
impl AnimateCursorInput {
    pub fn new() -> Self {
        AnimateCursorInput
    }
}
pub(super) fn animate_cursor_input(mut query: Query<(&mut lg::AnimateControl, &lg::InputCursorHover), With<AnimateCursorInput>>) {
    for (mut control, input) in &mut query {
        control.trigger = input.hover
    }
}


// =========================
// SCOPED TO <T> = MyData


/// Send trigger bool to the MyData of specified widget
#[derive(Component, Clone)]
pub struct AnimateSendInputToTree(pub String);
pub(super) fn animate_send_input_to_tree(mut trees: Query<&mut UiTree<MyData>>, query: Query<(&Widget, &lg::InputCursorHover, &AnimateSendInputToTree)>) {
    for mut tree in &mut trees {
        for (source, input, location) in &query {
            let data: &mut MyData = match source.fetch_mut_ext(&mut tree, &*location.0) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data_mut();
            data.animate = input.hover;
        }
    }
}

/// Pull trigger bool from widget's MyData
#[derive(Component, Clone)]
pub struct AnimatePullInputFromTree;
pub(super) fn animate_pull_input_from_tree(mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&Widget, &mut lg::AnimateControl), With<AnimatePullInputFromTree>>) {
    for mut tree in &mut trees {
        for (source, mut destination) in &mut query {
            let data: &MyData = match source.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data();
            destination.trigger = data.animate;
        }
    }
}
