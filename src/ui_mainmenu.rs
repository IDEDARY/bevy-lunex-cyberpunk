use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::general::*;
use crate::style::*;


// ===========================================================
// === SETUP MAIN MENU LAYOUT ===

pub fn setup_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut UITree) {

    // ===========================================================
    // === SETUP WIDGETS AND ENTITIES ===
    //# This is where the layouting magic happens. Here we declare the positions and spawn relevant entities.

    //# Create MAIN_MENU widget
    let main_menu = Widget::create(system, "main_menu", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.pack()).unwrap();

    //# All widgets created in ROOT have visibility == false by default
    main_menu.fetch_mut(system, "").unwrap().set_visibility(true);
    
    //# --------------------------------------------------------------------------------------------------------------

    //# Create BACKGROUND in MAIN_MENU
    let background = Widget::create(system, &main_menu.end("background"), Layout::Window {
        relative: Vec2 { x: 0.0, y: 0.0 },
        width_relative: 100.0 + 2.6*2.0,
        height_relative: 100.0 + 2.0*2.0,
        ..Default::default()
    }.pack()).unwrap();
    
    //# Spawn entity with widget for querying
    commands.spawn((
        background.clone(),
        SmoothWiggleEffect::new(0.007, 0.002, 2.6, 2.0),
    ));

    //# Create 'nameless' widget in BACKGROUND (useful when widget is not important and is used only for layout purposes (no interaction), it is skipped in path)
    let image = Widget::create(system, &background.end(""), Layout::Solid {
        width: 2560.0,
        height: 1440.0,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();

    commands.spawn(ImageElementBundle::new(image.clone(), &ImageParams::default().with_depth(-0.1), asset_server.load("images/main_menu/screen_10.png"), Vec2::default()));

    commands.spawn((
        ImageElementBundle::new(image.clone(), &ImageParams::default(), asset_server.load("images/main_menu/screen_10_red.png"), Vec2::default()),
        FastFlickerEffect::new(0.9, 0.75, 1.0),
        OnOffImageEffect::new(0.4, true),
    ));

    commands.spawn((
        ImageElementBundle::new(image.clone(), &ImageParams::default(), asset_server.load("images/main_menu/screen_10_blue.png"), Vec2::default()),
        FastFlickerEffect::new(0.9, 0.75, 1.0),
        OnOffImageEffect::new(0.4, false)
    ));

    commands.spawn((
        ImageElementBundle::new(image.clone(), &ImageParams::default(), asset_server.load("images/main_menu/screen_10_blink1.png"), Vec2::default()),
        FastFlickerEffect::new(0.01, 0.2, 0.9),
    ));
    commands.spawn((
        ImageElementBundle::new(image.clone(), &ImageParams::default(), asset_server.load("images/main_menu/screen_10_blink2.png"), Vec2::default()),
        FastFlickerEffect::new(0.01, 0.9, 0.2),
    ));

    //# Set depth to IMAGE widget so the image renders behind other widgets (All widgets start at 100 + level == Menu/Display -> 102, Menu/Display/Button -> 103)
    image.fetch_mut(system, "").unwrap().set_depth(50.0);

    //# --------------------------------------------------------------------------------------------------------------

    //# Create BOARD in MAIN_MENU
    let board = Widget::create(system, &main_menu.end("board"), Layout::Solid {
        width: 807.0,
        height: 1432.0,
        horizontal_anchor: -0.80,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    commands.spawn(ImageElementBundle::new(board.clone(), &ImageParams::default(), asset_server.load("images/main_menu/board.png"), Vec2::default()));


    //# Create 'nameless' widget in BOARD
    let nameless_boundary = Widget::create(system, &board.end(""), Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: 15.0 },
        relative_2: Vec2 { x: 105.0, y: 30.0 },
        ..Default::default()
    }.pack()).unwrap();



    //# Create LOGO in 'nameless' widget and omit 'nameless' from path (BOARD/'nameless'/LOGO -> BOARD/LOGO)
    let logo = Widget::create(system, &nameless_boundary.end("logo"), Layout::Solid {
        width: 681.0,
        height: 166.0,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    commands.spawn((ImageElementBundle::new(logo.clone(), &ImageParams::default(), asset_server.load("images/main_menu/logo.png"), Vec2::default()), FastFlickerEffect::new(0.05, 0.9, 1.0)));



    //# Create 'nameless' widget in LOGO. Further down in the application the widget is not used, so we can leave it nameless and forget about it.
    let logo_shadow = Widget::create(system, &logo.end(""), Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: -10.0 },
        relative_2: Vec2 { x: 105.0, y: 110.0 },
        ..Default::default()
    }.pack()).unwrap();
    //image_element_spawn!(commands, asset_server, logo_shadow.clone(), &ImageParams::default(), "images/main_menu/logo_shadow.png");
    commands.spawn(ImageElementBundle::new(logo_shadow.clone(), &ImageParams::default(), asset_server.load("images/main_menu/logo_shadow.png"), Vec2::default()));

    //# --------------------------------------------------------------------------------------------------------------

    // ===========================================================
    // === BUTTON LAYOUT ===

    //# Create BUTTONLIST in BOARD
    let button_list = Widget::create(system, &board.end("buttons"), Layout::Relative {
        relative_1: Vec2 { x: 17.0, y: 33.0 },
        relative_2: Vec2 { x: 82.0, y: 79.0 },
        ..Default::default()
    }.pack()).unwrap();


    let text_style = TextStyle {
        font: asset_server.load(MAIN_MENU_BUTTON_FONT),
        font_size: 40.0,
        color: GLOBAL_COLOR_STANDBY,
    };

    let names = textgrid![["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"]];
    let names_display = textgrid![["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"]];

    let grid = GridParams::new(&names).with_width(100.0).with_height(11.0);
    let widget = grid_generate(system, &button_list.end(""), Vec2::default(), &grid).unwrap();
    
    for x in 0..names.len() {
        for y in 0..names[0].len() {

            let new_widget = Widget::new(&widget.end(&names[x][y]));
            commands.spawn((
                new_widget.clone(),
                MainMenuButton (),
            ));

            let widget = Widget::create(system, &new_widget.end(""), Layout::Window {
                width_relative: 100.0,
                height_relative: 100.0,
                ..Default::default()
            }.pack()).unwrap();
            commands.spawn((
                TextElementBundle::new(widget.clone(), &TextParams::centerleft().at(5.0, 50.0).scaled(35.0).styled(&text_style).with_height(90.0), &names_display[x][y]),
                ColorHighlightEffect (text_style.color, GLOBAL_COLOR_HOVER),
                AnimateWidgetEffect (Vec2::default(), Vec2::new(5.0, 0.0)),
            ));
            commands.spawn((
                ImageElementBundle::new(widget.clone(), &ImageParams::default(), asset_server.load("images/main_menu/button.png"), Vec2::default()),
                ColorHighlightEffect (GLOBAL_COLOR_STANDBY.with_a(0.0), GLOBAL_COLOR_HOVER.with_a(0.8)),
                ColorHighlightEffectUpdater (),
            ));
        }
    }


}


// ===========================================================
// === INTERACTION SYSTEMS ===

#[derive(Component)]
struct MainMenuButton ();
fn button_tick(mut systems: Query<(&mut UITree, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &MainMenuButton)>, mouse_button_input: Res<Input<MouseButton>>, mut exit: EventWriter<bevy::app::AppExit>) {
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();
    for (widget, _) in &mut query {
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            widget.fetch_data_set_f32(&mut system, "#0", "color_highlight_effect_slider", 1.0).unwrap();
            widget.fetch_data_set_f32(&mut system, "#0", "animate_widget_effect_slider", 1.0).unwrap();

            if mouse_button_input.just_pressed(MouseButton::Left) {
                match widget.fetch(&mut system, "").unwrap().get_name().as_str() {
                    "settings" => {
                        Widget::new("main_menu").fetch_mut(&mut system, "").unwrap().set_visibility(false);
                        Widget::new("settings").fetch_mut(&mut system, "").unwrap().set_visibility(true);
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


/// ## Image On/Off image effect
/// This effect is used to alternate between the red and blue light image for the background
#[derive(Component, Default)]
struct OnOffImageEffect {
    x: f32,
    x_speed: f32,
    inverse: bool,
}
impl OnOffImageEffect {
    pub fn new (x_speed: f32, inverse: bool) -> OnOffImageEffect {
        OnOffImageEffect {
            x: 0.0,
            x_speed,
            inverse,
        }
    }
}
fn on_off_image_effect_update (mut query: Query<(&mut Sprite, &mut OnOffImageEffect)>) {
    for (mut sprite, mut timer) in &mut query {
        timer.x += timer.x_speed;
        if timer.x > 100.0 { timer.x = 0.0 }

        if timer.inverse {
            if timer.x > 50.0 { sprite.color.set_a(0.0); }
        } else {
            if timer.x < 50.0 { sprite.color.set_a(0.0); }
        }
    }
}


// ===========================================================
// === PACK ALL SYSTEMS TO PLUGIN ===

pub struct UIMainMenuPlugin;
impl Plugin for UIMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, button_tick)
            .add_systems(Update, on_off_image_effect_update.after(fast_flicker_effect_update));
    }
}
