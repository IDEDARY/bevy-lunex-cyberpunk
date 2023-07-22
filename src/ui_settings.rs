use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::general::*;
use crate::style::*;


// ===========================================================
// === SETUP SETTINGS LAYOUT ===

pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    // ===========================================================
    // === SETUP STYLES ===
    //# Here we declare all the styling that will be applied later. To use global settings, we import constants declaring project scale colors and font paths.

    let rajdhani_bold: Handle<Font> = asset_server.load("Fonts/Rajdhani/Rajdhani-Bold.ttf");
    let rajdhani_medium: Handle<Font> = asset_server.load("Fonts/Rajdhani/Rajdhani-Medium.ttf");
    let blender_medium: Handle<Font> = asset_server.load("Fonts/Blender/BlenderPro-Medium.ttf");


    let style_navigation = TextStyle {
        font: rajdhani_bold.clone(),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_tab = TextStyle {
        font: blender_medium.clone(),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_category = TextStyle {
        font: rajdhani_medium.clone(),
        font_size: 40.0,
        color: SETTINGS_COLOR_CATEGORY,
    };
    let style_item = TextStyle {
        font: rajdhani_medium.clone(),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };

    // ===========================================================
    // === SETUP WIDGETS AND ENTITIES ===
    //# This is where the layouting magic happens. Here we declare the positions and spawn relevant entities.

    //# Create SETTINGS in ROOT
    let settings = Widget::create(system, "settings", Box::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BACKGROUND in SETTINGS
    let background = Widget::create(system, &settings.end("background"), Box::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BACKGROUND
    let image = Widget::create(system, &background.end(""), Box::Solid {
        width: 3840,
        height: 2160,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, image.clone(), &ImageParams::default(), "settings/background.png");
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 3.0, y: 1.0 },
        relative_2: Vec2 { x: 15.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let button_return = Widget::create(system, &boundary.end("return"), Box::Solid {
        width: 3,
        height: 1,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    text_element_spawn!(commands, button_return, &TextParams::centerleft().styled(&style_navigation).scaled(35.0).with_height(80.0).at(10.0, 50.0), "RETURN",
        ColorHighlightEffect (style_navigation.color, GLOBAL_COLOR_HOVER),
        ReturnButton (),
        HoverEffectInput (),
        Effect ()
    );

    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 3.0, y: 9.0 },
        relative_2: Vec2 { x: 90.0, y: 13.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let line = Widget::create(system, &boundary.end(""), Box::Solid {
        width: 3522,
        height: 4,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, line, &ImageParams::default(), "settings/line.png");


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BAR widget in SETTINGS
    let bar = Widget::create(system, &settings.end("bar"), Box::Relative {
        relative_1: Vec2 { x: 18.0, y: 1.0 },
        relative_2: Vec2 { x: 82.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BAR
    let boundary = Widget::create(system, &bar.end(""), Box::Solid {
        width: 28,
        height: 1,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();

    //# Generate grid of widgets in 'nameless'
    let map = [["tab 1"].to_vec(), ["tab 2"].to_vec(), ["tab 3"].to_vec(), ["tab 4"].to_vec(), ["tab 5"].to_vec(), ["tab 6"].to_vec(), ["tab 7"].to_vec(), ["tab 8"].to_vec()].to_vec();
    let grid = Grid {
        width_relative: 100.0,
        height_relative: 20.0,
        width_padding_gap: true,
        gap_relative: Vec2::new(10.0, 0.0),
        ..Default::default()
    };
    grid.create_inside(system, &boundary, &map).unwrap();

    //# Loop over grid of widgets in 'nameless'
    for x in 0..map.len() {
        for y in 0..map[0].len() {

            //# Spawn image for widgets in 'nameless'
            let widget = Widget::new(&boundary.end(map[x][y]));
            text_element_spawn!(commands, widget, &TextParams::center().styled(&style_tab).scaled(50.0).with_height(80.0), &map[x][y].to_uppercase(),
                ColorHighlightEffect (style_tab.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                Effect ()
            );
        }
    }


    //# --------------------------------------------------------------------------------------------------------------
    
    //# Create 'nameless' widget in SETTINGS
    let boundary1 = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 10.0, y: 14.0 },
        relative_2: Vec2 { x: 90.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in 'nameless'
    let boundary2 = Widget::create(system, &boundary1.end(""), Box::Solid {
        width: 105,
        height: 100,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create DISPLAY widget in 'nameless'/'nameless' (skipping 2 nameless widgets at once)
    let display = Widget::create(system, &settings.add(&boundary1).add(&boundary2).end("display"), Box::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in DISPLAY
    let category = Widget::create(system, &display.end(""), Box::Solid {
        width: 1934,
        height: 96,
        vertical_anchor: -1.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, category.clone(), &ImageParams::default(), "settings/category.png");
    text_element_spawn!(commands, category.clone(), &TextParams::centerleft().styled(&style_category).scaled(40.0).at(2.0, 50.0), "Display");


    let map = [["fullscreen", "window_mode", "title", "resizable", "decorations", "window mode"].to_vec()].to_vec();
    let grid = Grid {
        width_relative: 96.0,
        height_relative: 11.0,
        width_padding_gap: true,
        height_padding_gap: true,
        gap_relative: Vec2::new(2.0, 2.0),
        ..Default::default()
    };
    let widget = grid.create(system, &display.end("list"), &map, Vec2::new(0.0, 16.0)).unwrap();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let widget = Widget::new(&widget.end(map[x][y]));
            text_element_spawn!(commands, widget.clone(), &TextParams::centerleft().styled(&style_item).scaled(40.0).at(2.0, 50.0), &map[x][y].to_uppercase(),
                ColorHighlightEffect (style_item.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                Effect ()
            );

            //# Create 'nameless' widget in DISPLAY
            let button = Widget::create(system, &widget.end(""), Box::Relative {
                relative_1: Vec2::new(47.0, 2.0),
                relative_2: Vec2::new(95.0, 98.0),
                ..Default::default()
            }.pack()).unwrap();

            let bb = buttons::OptionButton::create(commands, asset_server, system, button.clone(), ["Enabled".to_string(), "Disabled".to_string()].to_vec());

            image_element_spawn!(commands, asset_server, button.clone(), &ImageParams::default(), "settings/button_dark.png",
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.3), GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                Effect (),
                bb
            );
        }
    }

}


// ===========================================================
// === INTERACTION SYSTEMS ===

#[derive(Component)]
pub struct HoverEffectInput ();
fn hover_effect_input(mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &HoverEffectInput)>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            let data_option = widget.fetch_mut(&mut system, "").unwrap().data_get_mut();
            match data_option {
                Option::Some ( data ) => {
                    data.f32s.insert("color_highlight_effect_slider".to_string() , 1.0);
                },
                Option::None => {
                    *data_option = Option::Some(Data::new());
                },
            }
        }
    }
}


#[derive(Component)]
struct ReturnButton ();
fn return_button_update (mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &ReturnButton)>, mouse_button_input: Res<Input<MouseButton>>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            if mouse_button_input.just_pressed(MouseButton::Left) {
                Widget::new("main_menu").fetch_mut(&mut system, "").unwrap().set_visibility(true);
                Widget::new("settings").fetch_mut(&mut system, "").unwrap().set_visibility(false);
            }

        }
    }
}

mod buttons {
    use bevy::prelude::*;
    use bevy_lunex::prelude::*;
    use crate::{style::*, ui_settings::*};

    #[derive(Component)]
    pub struct OptionButtonText ();
    pub fn option_button_text_update (mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Text, &OptionButtonText)>) {
        let mut system = systems.get_single_mut().unwrap();
        for (widget, mut text, colors) in &mut query {
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
        pub fn create (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy, widget: Widget, options: Vec<String>) -> OptionButton {

            let cycle_left = Widget::create(system, &widget.end("button_cycle_left"), Box::Relative {
                relative_1: Vec2::new(4.0, 20.0),
                relative_2: Vec2::new(15.0, 80.0),
                ..Default::default()
            }.pack()).unwrap();
            let image_box = Widget::create(system, &cycle_left.end(""), Box::Solid {
                width: 1,
                height: 1,
                ..Default::default()
            }.pack()).unwrap();
            image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "settings/arrow_left_empty.png",
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                Effect ()
            );


            let cycle_right = Widget::create(system, &widget.end("button_cycle_right"), Box::Relative {
                relative_1: Vec2::new(85.0, 20.0),
                relative_2: Vec2::new(96.0, 80.0),
                ..Default::default()
            }.pack()).unwrap();
            let image_box = Widget::create(system, &cycle_right.end(""), Box::Solid {
                width: 1,
                height: 1,
                ..Default::default()
            }.pack()).unwrap();
            image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "settings/arrow_right_empty.png",
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                Effect ()
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

            OptionButton {
                state_change: true,
                current: 0,
                options,
            }
        }
        pub fn cycle_left (&mut self, system: &mut Hierarchy, widget: Widget) {
            if self.current > 0 {
                self.current -= 1;
                self.state_change = true;
            
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
        }
        pub fn cycle_right (&mut self, system: &mut Hierarchy, widget: Widget) {
            if self.current < self.options.len() - 1 {
                self.current += 1;
                self.state_change = true;
            
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
        }
    }
    pub fn option_button_update (mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &mut OptionButton)>, mouse_button_input: Res<Input<MouseButton>>) {
        let (mut system, placement) = systems.get_single_mut().unwrap();
        let cursor = cursors.get_single().unwrap();
        for (widget, mut button) in &mut query {
            if widget.is_within(&system, "button_cycle_left", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    button.cycle_left(&mut system, widget.clone());
                }
            }
            if widget.is_within(&system, "button_cycle_right", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    button.cycle_right(&mut system, widget.clone());
                }
            }
        }
    }

    
}




struct SettingButton ();
fn button_tick(mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &Button)>, mouse_button_input: Res<Input<MouseButton>>, mut windows: Query<&mut Window>, mut exit: EventWriter<bevy::app::AppExit>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    let mut window = windows.get_single_mut().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            let data_option = widget.fetch_mut(&mut system, "#0").unwrap().data_get_mut();
            match data_option {
                Option::Some ( data ) => {
                    data.f32s.insert("color_highlight_effect_slider".to_string() , 1.0);
                    data.f32s.insert("animate_widget_effect_slider".to_string() , 1.0);
                },
                Option::None => {
                    *data_option = Option::Some(Data::new());
                },
            }

            if mouse_button_input.just_pressed(MouseButton::Left) {
                match widget.fetch(&mut system, "").unwrap().get_name().as_str() {
                    "fullscreen" => {
                        if window.mode == bevy::window::WindowMode::BorderlessFullscreen {
                            window.mode = bevy::window::WindowMode::Windowed;
                        } else {
                            window.mode = bevy::window::WindowMode::BorderlessFullscreen;
                        }
                    },
                    "decorations" => {
                        window.decorations = !window.decorations;
                    },
                    "resizable" => {
                        window.resizable = !window.resizable;
                    },
                    "quit_game" => {
                        exit.send(bevy::app::AppExit);
                    },
                    _ => {},
                }
            }

        }
    }
}




// ===========================================================
// === PACK ALL SYSTEMS TO PLUGIN ===

pub struct UISettingsPlugin;
impl Plugin for UISettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hover_effect_input)
            .add_systems(Update, return_button_update)
            .add_systems(Update, (buttons::option_button_text_update, buttons::option_button_update));
    }
}