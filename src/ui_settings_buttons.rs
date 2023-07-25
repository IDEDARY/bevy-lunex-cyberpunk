use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::{general::*, style::*};


// ===========================================================
// === BUTTON DEFINITIONS ===

#[derive(Component)]
pub struct OptionButtonText ();
pub fn option_button_text_update (mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Text, &OptionButtonText)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut text, _) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.strings.get_mut("option_button_text") {
                    Option::Some(txt) => {
                        text.sections[0].value = txt.to_string();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}


#[derive(Component)]
pub struct OptionButton {
    state_change: bool,
    current: usize,
    options: Vec<String>,
}
impl OptionButton {
    fn update_data (&self, system: &mut Hierarchy, widget: Widget) {
        let widget = widget.fetch_mut(system, "").unwrap();
        let data_option = widget.data_get_mut();
        match data_option {
            Option::Some ( data ) => {
                match data.strings.get_mut("option_button_text") {
                    Option::Some(txt) => {
                        *txt = self.options[self.current].to_string();
                    }
                    Option::None => {
                        data.strings.insert("option_button_text".to_string(), self.options[self.current].to_string());
                    },
                }
            }
            Option::None => {
                let mut data = Data::new();
                data.strings.insert("option_button_text".to_string(), self.options[self.current].to_string());
                *data_option = Option::Some(data);
            },
        }
    }
    pub fn create (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy, path: &str, position: Layout, options: Vec<String>) -> (Widget, OptionButton){
        
        let widget = Widget::create(system, path, position).unwrap();
        image_element_spawn!(commands, asset_server, widget.clone(), &ImageParams::default(), "settings/button_dark.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.3), GLOBAL_COLOR_HOVER),
            ColorHighlightEffectUpdater ()
        );


        let cycle_left = Widget::create(system, &widget.end("button_cycle_left"), Box::Relative {
            relative_1: Vec2::new(3.0, 18.0),
            relative_2: Vec2::new(25.0, 82.0),
            ..Default::default()
        }.pack()).unwrap();
        let image_box = Widget::create(system, &cycle_left.end(""), Box::Solid {
            width: 1,
            height: 1,
            horizontal_anchor: -0.8,
            ..Default::default()
        }.pack()).unwrap();
        image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "settings/arrow_left_empty.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER),
            ColorHighlightEffectUpdater ()
        );


        let cycle_right = Widget::create(system, &widget.end("button_cycle_right"), Box::Relative {
            relative_1: Vec2::new(75.0, 18.0),
            relative_2: Vec2::new(97.0, 82.0),
            ..Default::default()
        }.pack()).unwrap();
        let image_box = Widget::create(system, &cycle_right.end(""), Box::Solid {
            width: 1,
            height: 1,
            horizontal_anchor: 0.8,
            ..Default::default()
        }.pack()).unwrap();
        image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "settings/arrow_right_empty.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER),
            ColorHighlightEffectUpdater ()
        );

        let style = TextStyle {
            font: asset_server.load("Fonts/Rajdhani/Rajdhani-SemiBold.ttf"),
            font_size: 40.0,
            color: GLOBAL_COLOR_STANDBY,
        };
        text_element_spawn!(commands, widget.clone(), &TextParams::center().styled(&style).scaled(90.0).with_height(40.0).at(50.0, 35.0), &options[0],
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY, GLOBAL_COLOR_HOVER),
            OptionButtonText ()
        );

        (widget, OptionButton {
            state_change: true,
            current: 0,
            options,
        })

    }
    pub fn cycle_left (&mut self, system: &mut Hierarchy, widget: Widget) {
        if self.current > 0 {
            self.current -= 1;
            self.state_change = true;
            self.update_data(system, widget);
        } else {
            self.current = self.options.len() - 1;
            self.state_change = true;
            self.update_data(system, widget);
        }
    }
    pub fn cycle_right (&mut self, system: &mut Hierarchy, widget: Widget) {
        if self.current < self.options.len() - 1 {
            self.current += 1;
            self.state_change = true;
            self.update_data(system, widget);
        } else {
            self.current = 0;
            self.state_change = true;
            self.update_data(system, widget);
        }
    }
    pub fn get_current (&self) -> &str {
        &self.options[self.current]
    }
}
pub fn option_button_update (mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&Widget, &mut OptionButton)>, mouse_button_input: Res<Input<MouseButton>>, mut windows: Query<&mut Window>) {
    
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    let mut window = windows.get_single_mut().unwrap();

    for (widget, mut button) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            widget.fetch_data_set_f32(&mut system, "", "color_highlight_effect_slider", 1.0).unwrap();
            widget.fetch_data_set_f32(&mut system, "button_cycle_left/#0", "color_highlight_effect_slider", 1.0).unwrap();
            widget.fetch_data_set_f32(&mut system, "button_cycle_right/#0", "color_highlight_effect_slider", 1.0).unwrap();

            if mouse_button_input.just_pressed(MouseButton::Left) {

                if widget.is_within(&system, "button_cycle_left", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                    button.cycle_left(&mut system, widget.clone());
                }
                if widget.is_within(&system, "button_cycle_right", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                    button.cycle_right(&mut system, widget.clone());
                }

                if button.state_change == true {
                    button.state_change = false;
                    match widget.fetch(&mut system, "").unwrap().get_name().as_str() {
                        "Window mode" => {
                            match button.get_current() {
                                "Windowed" => {window.mode = bevy::window::WindowMode::Windowed},
                                "Borderless" => {window.mode = bevy::window::WindowMode::BorderlessFullscreen},
                                _ => (),
                            };
                        },
                        "Decorations" => {
                            match button.get_current() {
                                "Enabled" => {window.decorations = true},
                                "Disabled" => {window.decorations = false},
                                _ => (),
                            };
                        },
                        "Resizable window" => {
                            match button.get_current() {
                                "Enabled" => {window.resizable = true},
                                "Disabled" => {window.resizable = false},
                                _ => (),
                            };
                        },
                        _ => {},
                    }
                }
            }

        }
    }
}
