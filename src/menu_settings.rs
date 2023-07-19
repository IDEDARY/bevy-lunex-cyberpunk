use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::general::*;



pub fn setup_menu_settings (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {

    // ===========================================================
    // === SETUP STYLES ===
    //# Here we declare all the styling that will be applied later. To use global settings, we import constants declaring project scale colors and font paths.

    let rajdhani_bold: Handle<Font> = asset_server.load("Fonts/Rajdhani/Rajdhani-Bold.ttf");
    let rajdhani_medium: Handle<Font> = asset_server.load("Fonts/Rajdhani/Rajdhani-Medium.ttf");
    let blender_medium: Handle<Font> = asset_server.load("Fonts/Blender/BlenderPro-Book.ttf");


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
    spawn_image(commands, asset_server, image.clone(), ImageParams::default(), "settings/background.png");
    image.fetch_mut(system, "").unwrap().set_depth(90.0);


    //# --------------------------------------------------------------------------------------------------------------

    //# Create 'nameless' widget in SETTINGS
    let boundary = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 2.0, y: 2.0 },
        relative_2: Vec2 { x: 10.0, y: 8.0 },
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
    spawn_text(commands, button_return, TextParams::centered(style_navigation.clone(), 40.0), "RETURN");


    //# --------------------------------------------------------------------------------------------------------------

    //# Create BAR widget in SETTINGS
    let bar = Widget::create(system, &settings.end("bar"), Box::Relative {
        relative_1: Vec2 { x: 12.0, y: 2.0 },
        relative_2: Vec2 { x: 88.0, y: 8.0 },
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
    let map = [["sound"].to_vec(), ["controls"].to_vec(), ["video"].to_vec(), ["interface"].to_vec()].to_vec();
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
            spawn_text(commands, widget, TextParams::centered(style_tab.clone(), 40.0), &map[x][y].to_uppercase());
        }
    }


    //# --------------------------------------------------------------------------------------------------------------
    
    //# Create 'nameless' widget in SETTINGS
    let boundary1 = Widget::create(system, &settings.end(""), Box::Relative {
        relative_1: Vec2 { x: 5.0, y: 14.0 },
        relative_2: Vec2 { x: 95.0, y: 100.0 },
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
    spawn_image(commands, asset_server, category.clone(), ImageParams::default(), "settings/category.png");
    spawn_text(commands, category.clone(), TextParams::left(style_category.clone(), 40.0, Vec2::new(2.0, 50.0)), "Display");


    let map = [["fullscreen","window_mode","resolution", "monitor", "vsync"].to_vec()].to_vec();
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
            spawn_text(commands, widget.clone(), TextParams::left(style_item.clone(), 40.0, Vec2::new(2.0, 50.0)), &map[x][y].to_uppercase());

            //# Create 'nameless' widget in DISPLAY
            let button = Widget::create(system, &widget.end(""), Box::Relative {
                relative_1: Vec2::new(47.0, 2.0),
                relative_2: Vec2::new(95.0, 98.0),
                ..Default::default()
            }.pack()).unwrap();
            //spawn_image(commands, asset_server, button.clone(), ImageParams::default(), "settings/button_dark.png");
            let image_params = ImageParams::default();
            commands.spawn (
                ImageElementBundle {
                    widget: button,
                    element: Element {
                        relative: image_params.relative,
                        absolute: image_params.absolute,
                        scale: image_params.scale,
                        ..default()
                    },
                    texture: asset_server.load("button.png"),
                    sprite: Sprite {
                        anchor: image_params.anchor,
                        color: RED_COLOR_DIM,
                        ..default()
                    },
                    ..Default::default()
                }
            );
        }
    }

}


#[derive(Component)]
pub struct HoverEffect (Color, Color);
fn hover_effect_text(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Text, &HoverEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut text, colors) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("color_slider") {
                    Option::Some(color_slider) => {
                        if *color_slider > 0.0 {*color_slider -= 0.03} else {*color_slider = 0.0}
                        let color = tween_color_hsla_short(colors.0, colors.1, *color_slider);
                        text.sections[0].style.color = color;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn hover_effect_image(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Sprite, &Handle<Image>, &HoverEffect)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut sprite, _, colors) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.f32s.get_mut("color_slider") {
                    Option::Some(color_slider) => {
                        if *color_slider > 0.0 {*color_slider -= 0.03} else {*color_slider = 0.0}
                        let color = tween_color_hsla_short(colors.0, colors.1, *color_slider);
                        sprite.color = color;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}


