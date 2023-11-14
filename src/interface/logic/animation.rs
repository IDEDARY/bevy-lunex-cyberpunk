use bevy_lunex::prelude::*;
use bevy::prelude::*;
use mathio::tween;


/// # Animate
/// Holds control values that other components use.
#[derive(Component, Clone)]
pub struct Animate {
    /// If true, `value` will move to 1.0, else it will move to 0.0
    pub trigger: bool,
    /// The value representing state of the animation. Range: 0.0 - 1.0
    pub value: f32,
}
impl Animate {
    pub fn new() -> Self {
        Animate {
            trigger: false,
            value: 0.0,
        }
    }
}
pub (super) fn animate_system(mut query: Query<&mut Animate>) {
    for mut source in &mut query {
        if source.trigger {
            if source.value < 1.0 {source.value += 0.05} else {source.value = 1.0}
        } else {
            if source.value > 0.0 {source.value -= 0.05} else {source.value = 0.0}
        }
    }
}

/// # Animate Window Position
/// Takes control value from [`Animate`] component and updates
/// window position. Will panic if the container is not a window layout.
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

            let container = widget.fetch_mut(&mut tree).unwrap().get_container_mut();

            let window = container.get_layout_mut().expect_window_mut();
            window.relative.x = tween(source1.pos1.x, source1.pos2.x, source2.value);
            window.relative.y = tween(source1.pos1.y, source1.pos2.y, source2.value);

        }
    }
}

/// # Animate Color
/// Takes control value from [`Animate`] component and updates
/// color values of image and text.
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
    for (mut text, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        text.sections[0].style.color = color;
    }
}
pub (super) fn animate_color_image_system(mut query: Query<(&mut Sprite, &AnimateColor, &Animate)>) {
    for (mut sprite, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        sprite.color = color;
    }
}