use bevy_lunex::prelude::*;
use bevy::prelude::*;
use mathio::tween;

use super::InputMouseHover;


#[derive(Component)]
pub struct AnimateWindow {
    value: f32,
}
impl AnimateWindow {
    pub fn new() -> impl Component {
        AnimateWindow {
            value: 0.0,
        }
    }
}
pub (super) fn animate_window_system(mut query: Query<(&mut AnimateWindow, &InputMouseHover), With<Widget>>) {
    for (mut source1, source2) in &mut query {
        if source2.hover { source1.value = 1.0 }
        if source1.value > 0.0 {source1.value -= 0.05} else {source1.value = 0.0}
    }
}


#[derive(Component)]
pub struct AnimateWindowPosition {
    pos1: Vec2,
    pos2: Vec2,
}
impl AnimateWindowPosition {
    pub fn new(pos1: Vec2, pos2: Vec2) -> impl Bundle {
        (
            AnimateWindowPosition {
                pos1,
                pos2,
            },
            AnimateWindow::new()
        )
    }
}
pub (super) fn animate_window_position_system(mut trees: Query<&mut UiTree>, mut query: Query<(&Widget, &AnimateWindowPosition, &AnimateWindow)>) {
    for mut tree in &mut trees {
        for (widget, source1, source2) in &mut query {

            let container = widget.fetch_mut_ext(&mut tree, ".Button").unwrap().get_container_mut();

            let window = container.get_layout_mut().expect_window_mut();
            window.relative.x = tween(source1.pos1.x, source1.pos2.x, source2.value);
            window.relative.y = tween(source1.pos1.y, source1.pos2.y, source2.value);

        }
    }
}