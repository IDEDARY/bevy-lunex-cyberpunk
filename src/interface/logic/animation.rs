use crate::prelude::*;
use mathio::tween;

/// # Lunex Animation System Set
/// All systems dealing with input for:
/// * [`CursorHoverAsAnimateInput`]
/// * [`AnimateControl`]
/// * [`AnimateWindowPosition`]
/// * [`AnimateColor`]
/// 
/// Make sure you run your logic correspondingly to this set, if you use these components
#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub struct AnimateSystemSet;


/// # Animate
/// Holds control value that other animation components use.
#[derive(Component, Clone)]
pub struct Animate {
    /// The value representing state of the animation. Range: 0.0 - 1.0
    pub slider: f32
}
impl Animate {
    pub fn new() -> Self {
        Animate {
            slider: 0.0,
        }
    }
}

/// # Animate Control
/// Changes the slider value in [`Animate`] if trigger is true
/// ## Requires:
/// * [`lg::Animate`]
#[derive(Component, Clone)]
pub struct AnimateControl {
    /// If true, `value` will move to 1.0, else it will move to 0.0
    pub trigger: bool,
    /// The speed at which value will change if trigger is true
    pub forward_speed: f32,
    /// The speed at which value will change if trigger is true
    pub backward_speed: f32,
}
impl AnimateControl {
    pub fn new(forward_speed: f32, backward_speed: f32) -> Self {
        AnimateControl {
            trigger: false,
            forward_speed,
            backward_speed,
        }
    }
}
pub (super) fn animate_system(mut query: Query<(&mut Animate, &AnimateControl)>) {
    for (mut slider, trigger) in &mut query {
        if trigger.trigger {
            if slider.slider < 1.0 {slider.slider += trigger.forward_speed} else {slider.slider = 1.0}
        } else {
            if slider.slider > 0.0 {slider.slider -= trigger.backward_speed} else {slider.slider = 0.0}
        }
    }
}

/// # DEPRACTED Animate Window Position
/// Based on slider value from [`Animate`] component updates
/// window position. Will panic if the container is not a [`WindowLayout`].
/// ## Requires:
/// * [`lg::Animate`]
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
pub (super) fn animate_window_position_system<T:Component + Default>(mut trees: Query<&mut UiTree<T>>, query: Query<(&Widget, &AnimateWindowPosition, &Animate)>) {
    for mut tree in &mut trees {
        for (widget, source1, source2) in &query {

            let container = match widget.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_container_mut();

            let window = container.get_layout_mut().expect_window_mut();
            window.pos_relative.x = tween(source1.pos1.x, source1.pos2.x, source2.slider);
            window.pos_relative.y = tween(source1.pos1.y, source1.pos2.y, source2.slider);

        }
    }
}



/// # Animate into Solid Layout
/// Based on slider value from [`Animate`] component updates
/// solid position. Will panic if the container is not a [`SolidLayout`].
/// ## Requires:
/// * [`lg::Animate`]
#[derive(Component, Clone)]
pub struct AnimateIntoSolidLayout {
    pub layout1: SolidLayout,
    pub layout2: SolidLayout,
}
impl AnimateIntoSolidLayout {
    pub fn new(layout1: SolidLayout, layout2: SolidLayout) -> Self {
        AnimateIntoSolidLayout {
            layout1,
            layout2,
        }
    }
}
pub (super) fn animate_into_solid_layout_system<T:Component + Default>(mut trees: Query<&mut UiTree<T>>, query: Query<(&Widget, &AnimateIntoSolidLayout, &Animate)>) {
    for mut tree in &mut trees {
        for (widget, source, slider) in &query {

            let container = match widget.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_container_mut();

            let layout = container.get_layout_mut().expect_solid_mut();
            layout.tween(&source.layout1, &source.layout2, slider.slider);
            layout.tween(&source.layout1, &source.layout2, slider.slider);

        }
    }
}


/// # Animate Color
/// Based on slider value from [`Animate`] component updates
/// color values of image and text.
/// ## Requires:
/// * [`lg::Animate`]
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
pub (super) fn animate_color_text_system(mut query: Query<(&mut Text, &AnimateColor, &Animate)>) {
    for (mut text, color, slider) in &mut query {
        let color = tween_color_hsla_short(color.color1, color.color2, slider.slider);
        text.sections[0].style.color = color;
    }
}
pub (super) fn animate_color_image_system(mut query: Query<(&mut Sprite, &AnimateColor, &Animate)>) {
    for (mut sprite, color, slider) in &mut query {
        let color = tween_color_hsla_short(color.color1, color.color2, slider.slider);
        sprite.color = color;
    }
}


/// # Animate Mouse Input
/// Updates trigger value in [`AnimateControl`] component by values from [`lg::InputCursorHover`]
/// ## Requires:
/// * [`lg::AnimateControl`]
/// * [`lg::InputCursorHover`]
#[derive(Component, Clone)]
pub struct CursorHoverAsAnimateInput;
impl CursorHoverAsAnimateInput {
    pub fn new() -> Self {
        CursorHoverAsAnimateInput
    }
}
pub(super) fn animate_cursor_input(mut query: Query<(&mut lg::AnimateControl, &lg::InputCursorHover), With<CursorHoverAsAnimateInput>>) {
    for (mut control, input) in &mut query {
        control.trigger = input.hover
    }
}


// =========================
// SCOPED TO <T> = MyData


/// # Pipe Cursor Hover as Animate Input
/// Send current [`lg::InputCursorHover`] to specific widget's [`MyData`]
/// ## Requires:
/// * [`lg::InputCursorHover`]
#[derive(Component, Clone)]
pub struct PipeCursorHoverAsAnimateInput(pub String);
pub(super) fn pipe_cursor_hover_as_animate_input(mut trees: Query<&mut UiTree<MyData>>, query: Query<(&Widget, &lg::InputCursorHover, &PipeCursorHoverAsAnimateInput)>) {
    for mut tree in &mut trees {
        for (widget, input, location) in &query {
            let data: &mut MyData = match widget.fetch_mut_ext(&mut tree, &*location.0) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data_mut();
            data.animate_trigger = input.hover;
        }
    }
}


/// # Pipe Animate Input from Tree
/// Pull `animate_trigger` from widget's [`MyData`]
/// ## Requires:
/// * [`lg::AnimateControl`]
#[derive(Component, Clone)]
pub struct PipeAnimateInputFromTree;
pub(super) fn pipe_animate_input_from_tree(mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&Widget, &mut lg::AnimateControl), With<PipeAnimateInputFromTree>>) {
    for mut tree in &mut trees {
        for (widget, mut control) in &mut query {
            let data: &MyData = match widget.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data();
            control.trigger = data.animate_trigger;
        }
    }
}


/// # Pipe Animate to Tree
/// Send `animate_slider` to specific widget's [`MyData`]
/// ## Requires:
/// * [`lg::Animate`]
#[derive(Component, Clone)]
pub struct PipeAnimateToTree(pub String);
pub(super) fn pipe_animate_to_tree(mut trees: Query<&mut UiTree<MyData>>, query: Query<(&Widget, &lg::Animate, &PipeAnimateToTree)>) {
    for mut tree in &mut trees {
        for (widget, slider, location) in &query {
            let data: &mut MyData = match widget.fetch_mut_ext(&mut tree, &*location.0) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data_mut();
            data.animate_slider = slider.slider;
        }
    }
}

/// # Pipe Animate from Tree
/// Pull `animate_slider` from widget's [`MyData`]
/// ## Requires:
/// * [`lg::Animate`]
#[derive(Component, Clone)]
pub struct PipeAnimateFromTree;
pub(super) fn pipe_animate_from_tree(mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&Widget, &mut lg::Animate), With<PipeAnimateFromTree>>) {
    for mut tree in &mut trees {
        for (widget, mut slider) in &mut query {
            let data: &MyData = match widget.fetch_mut(&mut tree) {
                Ok(d) => d,
                Err(_) => continue,
            }.get_data();
            slider.slider = data.animate_slider;
        }
    }
}
