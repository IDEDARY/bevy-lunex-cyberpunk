use ahash::AHashMap as HashMap;
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

    let style_navigation = TextStyle {
        font: asset_server.load(GLOBAL_NAVIGATION_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_tab = TextStyle {
        font: asset_server.load(GLOBAL_TAB_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };
    let style_category = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 40.0,
        color: SETTINGS_COLOR_CATEGORY,
    };
    let style_item = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };

    
    // ===========================================================
    // === SETUP WIDGETS AND ENTITIES ===
    //# This is where the layouting magic happens. Here we declare the positions and spawn relevant entities.

    //# Create SETTINGS in ROOT
    let settings = Widget::create(system, "settings", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BACKGROUND in SETTINGS
    let background = Widget::create(system, &settings.end("background"), Layout::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0,
        height_relative: 100.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BACKGROUND
    let image = Widget::create(system, &background.end(""), Layout::Solid {
        width: 3840.0,
        height: 2160.0,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, image.clone(), &ImageParams::default(), "images/settings/background.png");
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Layout::Relative {
        relative_1: Vec2 { x: 3.0, y: 1.0 },
        relative_2: Vec2 { x: 15.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let button_return = Widget::create(system, &boundary.end("return"), Layout::Solid {
        width: 3.0,
        height: 1.0,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    text_element_spawn!(commands, button_return, &TextParams::centerleft().styled(&style_navigation).scaled(35.0).with_height(80.0).at(10.0, 50.0), "RETURN",
        ColorHighlightEffect (style_navigation.color, GLOBAL_COLOR_HOVER),
        ReturnButton (),
        HoverEffectInput (),
        ColorHighlightEffectUpdater ()
    );

    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Layout::Relative {
        relative_1: Vec2 { x: 3.0, y: 9.0 },
        relative_2: Vec2 { x: 90.0, y: 13.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create BUTTON widget in 'nameless'
    let line = Widget::create(system, &boundary.end(""), Layout::Solid {
        width: 3522.0,
        height: 4.0,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, line, &ImageParams::default(), "images/settings/line.png");


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BAR widget in SETTINGS
    let bar = Widget::create(system, &settings.end("bar"), Layout::Relative {
        relative_1: Vec2 { x: 18.0, y: 1.0 },
        relative_2: Vec2 { x: 82.0, y: 8.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in BAR
    let boundary = Widget::create(system, &bar.end(""), Layout::Solid {
        width: 28.0,
        height: 1.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();

    //# Generate grid of widgets in 'nameless'
    let names = textgrid![["Display"], ["Sound"], ["tab 3"], ["tab 4"], ["tab 5"], ["tab 6"], ["tab 7"], ["tab 8"]];
    let grid = GridParams::new(&names).with_width(100.0).with_height(20.0).with_width_gap(10.0);
    grid_generate_inside(system, &boundary, &grid).unwrap();

    //# Loop over grid of widgets in 'nameless'
    for x in 0..names.len() {
        for y in 0..names[0].len() {

            //# Spawn image for widgets in 'nameless'
            let widget = Widget::new(&boundary.end(&names[x][y]));
            text_element_spawn!(commands, widget, &TextParams::center().styled(&style_tab).scaled(50.0).with_height(80.0), &names[x][y].to_uppercase(),
                ColorHighlightEffect (style_tab.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                ColorHighlightEffectUpdater ()
            );
        }
    }


    //# --------------------------------------------------------------------------------------------------------------
    
    //# Create 'nameless' widget in SETTINGS
    let boundary1 = Widget::create(system, &settings.end(""), Layout::Relative {
        relative_1: Vec2 { x: 10.0, y: 14.0 },
        relative_2: Vec2 { x: 90.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in 'nameless'
    let boundary2 = Widget::create(system, &boundary1.end(""), Layout::Solid {
        width: 105.0,
        height: 100.0,
        scaling: SolidScale::Fit,
        vertical_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create DISPLAY widget in 'nameless'/'nameless' (skipping 2 nameless widgets at once)
    let display = Widget::create(system, &settings.add(&boundary1).add(&boundary2).end("display"), Layout::Window {
        relative: Vec2::new(0.0, 0.0),
        width_relative: 100.0,
        height_relative: 40.0,
        ..Default::default()
    }.pack()).unwrap();

    //# Create 'nameless' widget in DISPLAY
    let category = Widget::create(system, &display.end(""), Layout::Solid {
        width: 1934.0,
        height: 96.0,
        vertical_anchor: -1.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, category.clone(), &ImageParams::default(), "images/settings/category.png");
    text_element_spawn!(commands, category.clone(), &TextParams::centerleft().styled(&style_category).scaled(40.0).at(2.0, 50.0), "Display");



    let names = textgrid![["Window mode", "Decorations", "Resizable window", "Resolution", "Profiler Overlay"]];
    let mut options = HashMap::new();
    options.insert("Window mode", (textrow!["Borderless", "Windowed"], 0));
    options.insert("Resolution", (textrow!["1920x1080", "1280x720", "720x720"], 0));
    options.insert("Profiler Overlay", (textrow!["Enabled", "Disabled"], 1));


    let grid = GridParams::new(&names).with_width(96.0).with_height(11.0).with_width_gap_border(true).with_height_gap_border(true);
    let widget = grid_generate(system, &display.end("list"), Vec2::new(0.0, 16.0), &grid).unwrap();

    for x in 0..names.len() {
        for y in 0..names[0].len() {

            //# --------------------------------------------------------------------------------------------------------------

            //# Spawn text element in the grid item
            let boundary = Widget::new(&widget.end(&names[x][y]));
            text_element_spawn!(commands, boundary.clone(), &TextParams::centerleft().styled(&style_item).scaled(40.0).at(2.0, 50.0), &names[x][y],
                ColorHighlightEffect (style_item.color, GLOBAL_COLOR_HOVER),
                HoverEffectInput (),
                ColorHighlightEffectUpdater ()
            );

            let highlight = Widget::create(system, &boundary.end(""), Layout::Relative {
                relative_1: Vec2 { x: -5.0, y: 15.0 },
                relative_2: Vec2 { x: 46.0, y: 85.0 },
                ..Default::default()
            }.pack()).unwrap();
            image_element_spawn!(commands, asset_server, highlight, &ImageParams::default(), "images/settings/selection_shadow.png",
                ColorHighlightEffect (style_item.color.with_a(0.0), GLOBAL_COLOR_HOVER.with_a(0.15)),
                ColorHighlightEffectUpdater ()
            );

            //# Create BUTTON in the grid item
            let mut option = (textrow!["Enabled", "Disabled"], 0);
            if let Some (custom) = options.get(names[x][y].as_str()) {
                option = custom.clone();
            }

            let position = Layout::Relative {
                relative_1: Vec2::new(47.0, 0.0),
                relative_2: Vec2::new(95.0, 100.0),
                ..Default::default()
            }.pack();

            let _ = OptionButton::create(commands, asset_server, system, &boundary.end(&format!("{} button", &names[x][y])), position, &names[x][y], option.0, option.1);

            //# --------------------------------------------------------------------------------------------------------------

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
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap() {

            //SET COLOR SLIDER ON SELF
            widget.fetch_data_set_f32(&mut system, "", "color_highlight_effect_slider", 1.0).unwrap();

            //THIS WILL SET THE COLOR SLIDER OF THE BUTTON, IF IT DOESNT HAVE THAT WIDGET IT WILL ERROR OUT, BUT WE DON'T CARE
            let _ = widget.fetch_data_set_f32(&mut system, "#0", "color_highlight_effect_slider", 1.0);
            let _ = widget.fetch_data_set_f32(&mut system, "#1", "color_highlight_effect_slider", 1.0);
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


#[derive(Component)]
pub struct OptionButton {
    name: String,
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
                match data.strings.get_mut("widget_text") {
                    Option::Some(txt) => {
                        *txt = self.options[self.current].to_string();
                    }
                    Option::None => {
                        data.strings.insert("widget_text".to_string(), self.options[self.current].to_string());
                    },
                }
            }
            Option::None => {
                let mut data = Data::new();
                data.strings.insert("widget_text".to_string(), self.options[self.current].to_string());
                *data_option = Option::Some(data);
            },
        }
    }
    pub fn create (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy, path: &str, position: LayoutPackage, name: &str, options: Vec<String>, current: usize) -> Widget {
        
        let widget = Widget::create(system, path, position).unwrap();
        image_element_spawn!(commands, asset_server, widget.clone(), &ImageParams::default(), "images/settings/button_dark.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.3), GLOBAL_COLOR_HOVER.with_a(0.35)),
            ColorHighlightEffectUpdater ()
        );


        let cycle_left = Widget::create(system, &widget.end("button_cycle_left"), Layout::Relative {
            relative_1: Vec2::new(3.0, 18.0),
            relative_2: Vec2::new(25.0, 82.0),
            ..Default::default()
        }.pack()).unwrap();
        let image_box = Widget::create(system, &cycle_left.end(""), Layout::Solid {
            width: 1.0,
            height: 1.0,
            horizontal_anchor: -0.8,
            ..Default::default()
        }.pack()).unwrap();
        image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "images/settings/arrow_left_empty.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER)
        );


        let cycle_right = Widget::create(system, &widget.end("button_cycle_right"), Layout::Relative {
            relative_1: Vec2::new(75.0, 18.0),
            relative_2: Vec2::new(97.0, 82.0),
            ..Default::default()
        }.pack()).unwrap();
        let image_box = Widget::create(system, &cycle_right.end(""), Layout::Solid {
            width: 1.0,
            height: 1.0,
            horizontal_anchor: 0.8,
            ..Default::default()
        }.pack()).unwrap();
        image_element_spawn!(commands, asset_server, image_box, &ImageParams::default(), "images/settings/arrow_right_empty.png",
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.6), GLOBAL_COLOR_HOVER)
        );

        let style = TextStyle {
            font: asset_server.load(GLOBAL_OPTION_BUTTON_FONT),
            font_size: 40.0,
            color: GLOBAL_COLOR_STANDBY,
        };
        text_element_spawn!(commands, widget.clone(), &TextParams::center().styled(&style).scaled(90.0).with_height(40.0).at(50.0, 40.0), &options[current],
            ColorHighlightEffect (GLOBAL_COLOR_STANDBY, GLOBAL_COLOR_HOVER),
            LiveWidgetText ()
        );

        let mut grid: Vec<Vec<String>> = Vec::new();
        for i in 0..options.len() {
            grid.push(vec![format!("selector {}", i)]);
        }
        let params = GridParams::new(&grid).with_anchor(bevy::sprite::Anchor::Center).with_width(6.0).with_height(10.0).with_width_gap(0.5);
        let grid_widget = grid_generate(system, &widget.end("grid"), Vec2::new(50.0, 80.0), &params).unwrap();

        for i in 0..options.len() {
            image_element_spawn!(commands, asset_server, Widget::new(&grid_widget.end(&format!("selector {}", i))), &ImageParams::default(), "images/settings/underline_dark.png",
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY, GLOBAL_COLOR_HOVER)
            );
        }

        widget_spawn!(commands, widget.clone(), OptionButton {
            name: name.to_string(),
            state_change: true,
            current,
            options,
        });

        widget

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
    pub fn get_name (&self) -> &str {
        &self.name
    }
}
pub fn option_button_update (mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&Widget, &mut OptionButton)>, mouse_button_input: Res<Input<MouseButton>>, mut windows: Query<&mut Window>) {
    
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    let mut window = windows.get_single_mut().unwrap();

    for (widget, mut button) in &mut query {

        // Mirror the color slider value
        match widget.fetch_data(&system, "").unwrap() {
            Option::Some(data) => {
                let val = data.f32s.get("color_highlight_effect_slider").unwrap().clone();
                widget.fetch_data_set_f32(&mut system, "button_cycle_left/#0", "color_highlight_effect_slider", val).unwrap();
                widget.fetch_data_set_f32(&mut system, "button_cycle_right/#0", "color_highlight_effect_slider", val).unwrap();
                for i in 0..button.options.len() {
                    widget.fetch_data_set_f32(&mut system, &format!("grid/selector {}", i), "color_highlight_effect_slider", val).unwrap();
                }
            },
            Option::None => {},
        }

        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
            if mouse_button_input.just_pressed(MouseButton::Left) {

                if widget.is_within(&system, "button_cycle_left", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                    button.cycle_left(&mut system, widget.clone());
                }
                if widget.is_within(&system, "button_cycle_right", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
                    button.cycle_right(&mut system, widget.clone());
                }

                if button.state_change == true {
                    button.state_change = false;
                    match button.get_name() {
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
                        "Resolution" => {
                            if window.mode == bevy::window::WindowMode::Windowed {
                                match button.get_current() {
                                    "1920x1080" => {window.resolution.set(1920.0, 1080.0)},
                                    "1280x720" => {window.resolution.set(1280.0, 720.0)},
                                    "720x720" => {window.resolution.set(720.0, 720.0)},
                                    _ => (),
                                };
                            }
                        },
                        "Profiler Overlay" => {
                            match button.get_current() {
                                "Enabled" => {Widget::new("profiler").fetch_mut(&mut system, "").unwrap().set_visibility(true)},
                                "Disabled" => {Widget::new("profiler").fetch_mut(&mut system, "").unwrap().set_visibility(false);},
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


// ===========================================================
// === PACK ALL SYSTEMS TO PLUGIN ===

pub struct UISettingsPlugin;
impl Plugin for UISettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hover_effect_input)
            .add_systems(Update, return_button_update)
            .add_systems(Update, option_button_update);
    }
}