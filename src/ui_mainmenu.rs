use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::general::*;
use crate::style::*;


// ===========================================================
// === SETUP MAIN MENU LAYOUT ===

pub fn setup_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {


    //# Create MAIN_MENU widget
    let main_menu = Widget::create(system, "main_menu", Box::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# All widgets created in ROOT have visibility == false by default
    main_menu.fetch_mut(system, "").unwrap().set_visibility(true);
    


    //# Create BACKGROUND in MAIN_MENU
    let background = Widget::create(system, &main_menu.end("background"), Box::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0 + 2.6*2.0,
        height_relative: 100.0 + 2.0*2.0,
        ..Default::default()
    }.pack()).unwrap();
    
    //# Spawn entity with widget for querying
    /*commands.spawn ((
        background.clone(),
        SmoothWiggle {..Default::default()},
    ));*/
    widget_spawn!(commands, background.clone(),
        SmoothWiggleEffect::new(0.007, 0.002, 2.6, 2.0)
    );
    



    //# Create 'nameless' widget in BACKGROUND (useful when widget is not important and is used only for layout purposes (no interaction), it is skipped in path)
    let image = Widget::create(system, &background.end(""), Box::Solid {
        width: 2560,
        height: 1440,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, image.clone(), &ImageParams::default(), "background.png");

    //# Set depth to IMAGE widget so the image renders behind other widgets (All widgets start at 100 + level == Menu/Display -> 102, Menu/Display/Button -> 103)
    image.fetch_mut(system, "").unwrap().set_depth(50.0);



    //# Create BOARD in MAIN_MENU
    let board = Widget::create(system, &main_menu.end("board"), Box::Solid {
        width: 807,
        height: 1432,
        horizontal_anchor: -0.80,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, board.clone(), &ImageParams::default(), "board.png");



    //# Create 'nameless' widget in BOARD
    let nameless_boundary = Widget::create(system, &board.end(""), Box::Relative {
        relative_1: Vec2 { x: -5.0, y: 15.0 },
        relative_2: Vec2 { x: 105.0, y: 30.0 },
        ..Default::default()
    }.pack()).unwrap();



    //# Create LOGO in 'nameless' widget and omit 'nameless' from path (BOARD/'nameless'/LOGO -> BOARD/LOGO)
    let logo = Widget::create(system, &nameless_boundary.end("logo"), Box::Solid {
        width: 681,
        height: 166,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, logo.clone(), &ImageParams::default(), "logo.png");



    //# Create 'nameless' widget in LOGO. Further down in the application the widget is not used, so we can leave it nameless and forget about it.
    let logo_shadow = Widget::create(system, &logo.end(""), Box::Relative {
        relative_1: Vec2 { x: -5.0, y: -10.0 },
        relative_2: Vec2 { x: 105.0, y: 110.0 },
        ..Default::default()
    }.pack()).unwrap();
    image_element_spawn!(commands, asset_server, logo_shadow.clone(), &ImageParams::default(), "logo_shadow.png");




    //################################################################################
    //# == Button Layout ==
    //# Here we will create a ButtonList widget which will contain all the buttons.

    //# Create BUTTONLIST in BOARD
    let _button_list = Widget::create(system, &board.end("buttons"), Box::Relative {
        relative_1: Vec2 { x: 17.0, y: 33.0 },
        relative_2: Vec2 { x: 82.0, y: 79.0 },
        ..Default::default()
    }.pack()).unwrap();


    //# Create a list with names for iteration
    //let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    //let button_name_list = ["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"];
    
    let font = asset_server.load("Fonts/Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };

    let map = [["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"].to_vec()].to_vec();
    let map_name = [["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"].to_vec()].to_vec();
    let grid = Grid {
        width_relative: 96.0,
        height_relative: 11.0,
        width_padding_gap: true,
        height_padding_gap: true,
        gap_relative: Vec2::new(2.0, 2.0),
        ..Default::default()
    };
    let widget = grid.create(system, &_button_list.end(""), &map, Vec2::new(0.0, 0.0)).unwrap();
    for x in 0..map.len() {
        for y in 0..map[0].len() {

            let new_widget = Widget::new(&widget.end(map[x][y]));
            widget_spawn!(commands, new_widget.clone(),
                Button ()
            );

            let widget = Widget::create(system, &new_widget.end(""), Box::Window {
                width_relative: 100.0,
                height_relative: 100.0,
                ..Default::default()
            }.pack()).unwrap();

            text_element_spawn!(commands, widget.clone(), &TextParams::centerleft().at(5.0, 50.0).scaled(35.0).styled(&text_style).with_height(90.0), &map_name[x][y],
                ColorHighlightEffect (text_style.color, GLOBAL_COLOR_HOVER),
                AnimateWidgetEffect (Vec2::default(), Vec2::new(5.0, 0.0))
            );
            image_element_spawn!(commands, asset_server, widget.clone(), &ImageParams::default(), "button.png",
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.0), GLOBAL_COLOR_HOVER.with_a(0.8)),
                Effect ()
            );
        }
    }


}


// ===========================================================
// === INTERACTION SYSTEMS ===

#[derive(Component)]
pub struct Button ();
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
                    "settings" => {
                        Widget::new("main_menu").fetch_mut(&mut system, "").unwrap().set_visibility(false);
                        Widget::new("settings").fetch_mut(&mut system, "").unwrap().set_visibility(true);
                    },
                    "load_game" => {
                        window.decorations = false;
                    },
                    "continue" => {
                        window.mode = bevy::window::WindowMode::BorderlessFullscreen;
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

pub struct UIMainMenuPlugin;
impl Plugin for UIMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, button_tick);
    }
}
