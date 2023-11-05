use std::borrow::Borrow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;
use mathio::tween;


#[derive(Component)]
pub struct AnimateWindowSlider {
    pub value: f32,
}
impl AnimateWindowSlider {
    pub fn new() -> impl Component {
        AnimateWindowSlider {
            value: 0.0,
        }
    }
}
pub (super) fn animate_window_slider_system(mut query: Query<&mut AnimateWindowSlider>) {
    for mut source in &mut query {
        if source.value > 0.0 {source.value -= 0.05} else {source.value = 0.0}
    }
}

#[derive(Component)]
pub struct AnimateWindowPosition {
    pub pos1: Vec2,
    pub pos2: Vec2,
    path: String,
}
impl AnimateWindowPosition {
    pub fn new(pos1: Vec2, pos2: Vec2, path: impl Borrow<str>) -> impl Bundle {
        (
            AnimateWindowPosition {
                pos1,
                pos2,
                path: path.borrow().into()
            },
            AnimateWindowSlider::new()
        )
    }
}
pub (super) fn animate_window_position_system(mut trees: Query<&mut UiTree>, query: Query<(&Widget, &AnimateWindowPosition, &AnimateWindowSlider)>) {
    for mut tree in &mut trees {
        for (widget, source1, source2) in &query {

            let container = widget.fetch_mut_ext(&mut tree, source1.path.clone()).unwrap().get_container_mut();

            let window = container.get_layout_mut().expect_window_mut();
            window.relative.x = tween(source1.pos1.x, source1.pos2.x, source2.value);
            window.relative.y = tween(source1.pos1.y, source1.pos2.y, source2.value);

        }
    }
}



#[derive(Component)]
pub struct AnimateColorSlider {
    pub value: f32,
}
impl AnimateColorSlider {
    pub fn new() -> impl Component {
        AnimateColorSlider {
            value: 0.0,
        }
    }
}
pub (super) fn animate_color_slider_system(mut query: Query<&mut AnimateColorSlider>) {
    for mut source in &mut query {
        if source.value > 0.0 {source.value -= 0.05} else {source.value = 0.0}
    }
}

#[derive(Component)]
pub struct AnimateColor {
    color1: Color,
    color2: Color,
}
impl AnimateColor {
    pub fn new(color1: Color, color2: Color) -> impl Bundle {
        (
            AnimateColor {
                color1,
                color2,
            },
            AnimateColorSlider::new()
        )
    }
}
pub (super) fn animate_color_text_system(mut query: Query<(&mut Text, &AnimateColor, &AnimateColorSlider)>) {
    for (mut text, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        text.sections[0].style.color = color;
    }
}
pub (super) fn animate_color_image_system(mut query: Query<(&mut Sprite, &AnimateColor, &AnimateColorSlider)>) {
    for (mut sprite, source1, source2) in &mut query {
        let color = tween_color_hsla_short(source1.color1, source1.color2, source2.value);
        sprite.color = color;
    }
}