#![allow(dead_code)]

use bevy::prelude::*;
use bevy_lunex::prelude::*;  


// ===========================================================
// === COLOR DEFINITIONS ===

pub const RED_COLOR: Color = Color::rgba(255./255., 98./255., 81./255., 1.1);
pub const RED_COLOR_DIM: Color = Color::rgba(204./255., 56./255., 51./255., 1.0);
pub const BLUE_COLOR: Color = Color::rgba(42./255., 237./255., 247./255., 1.3);
pub const PURPLE_COLOR: Color = Color::rgba(255./255., 34./255., 245./255., 1.3);
pub const YELLOW_COLOR: Color = Color::rgba(255./255., 245./255., 34./255., 1.3);
pub const GREY_COLOR: Color = Color::rgba(199./255., 186./255., 174./255., 1.0);


pub const GLOBAL_COLOR_STANDBY: Color = RED_COLOR;
pub const GLOBAL_COLOR_HOVER: Color = BLUE_COLOR;
pub const SETTINGS_COLOR_CATEGORY: Color = GREY_COLOR;


// ===========================================================
// === WIDGET EFFECTS ===
#[derive(Component)]
pub struct Effect ();
fn effect_tick(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &Effect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, _) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("color_highlight_effect_slider") {
                    Option::Some(slider) => {
                        if *slider > 0.0 {*slider -= 0.05} else {*slider = 0.0}
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

#[derive(Component)]
pub struct ColorHighlightEffect (pub Color, pub Color);
fn color_highlight_effect_update_text(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Text, &ColorHighlightEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut text, colors) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("color_highlight_effect_slider") {
                    Option::Some(slider) => {
                        let color = tween_color_hsla_short(colors.0, colors.1, *slider);
                        text.sections[0].style.color = color;
                    }
                    _ => {
                        text.sections[0].style.color = colors.0;
                    },
                }
            }
            _ => {
                text.sections[0].style.color = colors.0;
            },
        }
    }
}
fn color_highlight_effect_update_image(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Sprite, &Handle<Image>, &ColorHighlightEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut sprite, _, colors) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("color_highlight_effect_slider") {
                    Option::Some(slider) => {
                        let color = tween_color_hsla_short(colors.0, colors.1, *slider);
                        sprite.color = color;
                    }
                    _ => {
                        sprite.color = colors.0;
                    },
                }
            }
            _ => {
                sprite.color = colors.0;
            },
        }
    }
}


#[derive(Component)]
pub struct AnimateWidgetEffect (pub Vec2, pub Vec2);
fn animate_widget_effect_update(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &AnimateWidgetEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, positions) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("animate_widget_effect_slider") {
                    Option::Some(slider) => {
                        if *slider > 0.0 {*slider -= 0.05} else {*slider = 0.0}
                        let value = *slider;
                        let window = widget.layout_get_mut().expect_window_mut();
                        window.relative.x = tween(positions.0.x, positions.1.x, value);
                        window.relative.y = tween(positions.0.y, positions.1.y, value);

                    }
                    _ => {
                        let window = widget.layout_get_mut().expect_window_mut();
                        window.relative.x = positions.0.x;
                        window.relative.y = positions.0.y;
                    },
                }
            }
            _ => {
                let window = widget.layout_get_mut().expect_window_mut();
                window.relative.x = positions.0.x;
                window.relative.y = positions.0.y;
            },
        }
    }
}


#[derive(Component, Default)]
pub struct SmoothWiggleEffect {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
    x_amplitude: f32,
    y_amplitude: f32,
}
impl SmoothWiggleEffect {
    pub fn new (x_speed: f32, y_speed: f32, x_amplitude: f32, y_amplitude: f32) -> SmoothWiggleEffect {
        SmoothWiggleEffect {
            x: 0.0,
            y: 0.0,
            x_speed,
            y_speed,
            x_amplitude,
            y_amplitude,
        }
    }
}
fn smooth_wiggle_effect_update (mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut SmoothWiggleEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut wiggle) in &mut query {
        
        let window = widget.fetch_mut(&mut system, "").unwrap().layout_get_mut().expect_window_mut();

        wiggle.x += wiggle.x_speed;
        wiggle.y += wiggle.y_speed;

        window.relative.x = -wiggle.x_amplitude + wiggle.x.sin() * wiggle.x_amplitude;
        window.relative.y = -wiggle.y_amplitude + wiggle.y.sin() * wiggle.y_amplitude;
    }
}

// ===========================================================
// === PACK ALL SYSTEMS TO PLUGIN ===

pub struct HoverEffectPlugin;
impl Plugin for HoverEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, effect_tick)
            .add_systems(Update, (color_highlight_effect_update_text, color_highlight_effect_update_image).chain())
            .add_systems(Update, animate_widget_effect_update)
            .add_systems(Update, smooth_wiggle_effect_update);
    }
}