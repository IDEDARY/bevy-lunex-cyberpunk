use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use crate::general::*;

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
        relative: Vec2 { x: -5.0, y: -5.0 },
        width_relative: 110.0,
        height_relative: 110.0,
        ..Default::default()
    }.pack()).unwrap();
    
    //# Spawn entity with widget for querying
    commands.spawn ((
        background.clone(),
        SmoothWiggle {..Default::default()},
    ));
    



    //# Create 'nameless' widget in BACKGROUND (useful when widget is not important and is used only for layout purposes (no interaction), it is skipped in path)
    let image = Widget::create(system, &background.end(""), Box::Solid {
        width: 2560,
        height: 1440,
        scaling: SolidScale::Fill,
        ..Default::default()
    }.pack()).unwrap();
    spawn_image(commands, asset_server, image.clone(), ImageParams::default(), "background.png");

    //# Set depth to IMAGE widget so the image renders behind other widgets (All widgets start at 100 + level == Menu/Display -> 102, Menu/Display/Button -> 103)
    image.fetch_mut(system, "").unwrap().set_depth(90.0);



    //# Create BOARD in MAIN_MENU
    let board = Widget::create(system, &main_menu.end("board"), Box::Solid {
        width: 807,
        height: 1432,
        horizontal_anchor: -0.80,
        scaling: SolidScale::Fit,
        ..Default::default()
    }.pack()).unwrap();
    spawn_image(commands, asset_server, board.clone(), ImageParams::default(), "board.png");



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
    spawn_image(commands, asset_server, logo.clone(), ImageParams::default(), "logo.png");



    //# Create 'nameless' widget in LOGO. Further down in the application the widget is not used, so we can leave it nameless and forget about it.
    let logo_shadow = Widget::create(system, &logo.end(""), Box::Relative {
        relative_1: Vec2 { x: -5.0, y: -10.0 },
        relative_2: Vec2 { x: 105.0, y: 110.0 },
        ..Default::default()
    }.pack()).unwrap();
    spawn_image(commands, asset_server, logo_shadow.clone(), ImageParams::default(), "logo_shadow.png");




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
    let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    let button_name_list = ["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"];
    
    let font = asset_server.load("Fonts/Rajdhani/Rajdhani-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: MAIN_MENU_COLOR_STANDBY,
    };

    //# Create buttons in BUTTONLIST
    let step = 2.0/button_list.len() as f32;        //Distribute the containers equally
    for i in 0..button_list.len() {


        //# Create a BUTTON widget that will be used as boundary for input detection only
        let _button = Widget::create(system, &_button_list.end(button_list[i]), Box::Solid {
            width: 532,
            height: 75,
            scaling: SolidScale::Fit,
            vertical_anchor: -1.0 + step * i as f32,      //Where should the container be on Y axis (range: 1.0 to -1.0)
            ..Default::default()
        }.pack()).unwrap();
        //# Spawn button
        commands.spawn ((
            _button.clone(),
            MainMenuButton {}
        ));


        //# Create a nameless button that we will style and animate under BUTTON widget
        let _button_decoration = Widget::create(system, &_button.end(""), Box::Window {
            width_relative: 100.0,
            height_relative: 100.0,
            ..Default::default()
        }.pack()).unwrap();
        //# Spawn button decoration image
        commands.spawn ((
            _button_decoration.clone(),
            MainMenuButtonDecoration (),
            SpriteBundle {
                texture: asset_server.load("button.png"),
                transform: Transform { translation: Vec3 { x: 0., y: 0., z: 15. }, ..default() },
                sprite: Sprite {
                    color: Color::rgba(1., 1., 1., 0.0),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            }
        )).with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(button_name_list[i], text_style.clone()).with_alignment(TextAlignment::Left),
                transform: Transform { translation: Vec3 { x: 30., y: -75./2., z: 15. }, ..default() },
                text_anchor: Anchor::CenterLeft,
                ..default()
            });
        });


        //# Create a data stored in hierarchy for sharing
        let data = _button_decoration.fetch_mut(system, "").unwrap().data_get_mut();
        *data = Option::Some(Data::new());

    }

}


//################################################################################
//# == Button Logic ==
//# These two components and systems do all the button logic. Due to Bevy ECS, there is no "clean" way of modifying values of specific entities. (At least that I am not aware of)
//# So because each of the buttons are made of a mix of 2 entities that interact between each other, I save the changes of one entity to the Hierarchy as metadata and the other
//# entity fetches that data and synchronizes itself. This way there is a direct access to data, no looping over querries and finding corresponding entity, etc.
//# Might not be as much of an ECS solution as people want but it works and it is nice and simple. Sometimes mix of both worlds is the best solution.

//# The main entitity that will interact with cursor (Hitbox)
#[derive(Component)]
pub struct MainMenuButton ();
fn button_update(mut systems: Query<(&mut Hierarchy, &UserInterface)>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &MainMenuButton)>, mouse_button_input: Res<Input<MouseButton>>) {
    
    //# Get Hierarchy and cursor
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();

    //# Loop through all widgets in the query (MainMenuButton)
    for (widget, _) in &mut query {

        //# Check if the cursor is within the current widget boundaries
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){

            //# Fetch the nameless widget data from Hierarchy and update it (Image alpha and layout of the decoration widget)
            match widget.fetch_mut(&mut system, "#0").unwrap().data_get_mut() {
                Option::Some ( data ) => {
                    data.f32s.insert("alpha".to_string()        , 0.8);
                    data.f32s.insert("window_x".to_string()     , 5.0);
                    data.f32s.insert("color_slider".to_string() , 1.0);
                },
                Option::None => (),
            }

            if mouse_button_input.just_pressed(MouseButton::Left) && widget.fetch(&mut system, "").unwrap().get_name() == "settings" {
                let visibility = Widget::new("main_menu").fetch(&system, "").unwrap().get_visibility();
                Widget::new("main_menu").fetch_mut(&mut system, "").unwrap().set_visibility(!visibility);
                Widget::new("settings").fetch_mut(&mut system, "").unwrap().set_visibility(visibility);
            }



        }
    }
}

//# The secondary entity that will get updated by the main entity
#[derive(Component)]
pub struct MainMenuButtonDecoration ();
fn button_update_decoration(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Sprite, &mut Children, &MainMenuButtonDecoration)>, mut text_query: Query<&mut Text>) {
    
    //# Get Hierarchy
    let mut system = systems.get_single_mut().unwrap();

    //# Loop through all widgets in the query (MainMenuButtonDecoration)
    for (widget, mut sprite, children,  _) in &mut query {

        //# Fetch the current widget data from Hierarchy and synchronize itself
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::None => (),
            Option::Some ( data ) => {

                match data.f32s.get_mut("alpha") {
                    Option::None => (),
                    Option::Some(alpha) => {

                        if *alpha > 0.0 {*alpha -= 0.03} else {*alpha = 0.0}
                        sprite.color.set_a(*alpha);

                    }
                }

                match data.f32s.get_mut("color_slider") {
                    Option::None => (),
                    Option::Some(color_slider) => {

                        if *color_slider > 0.0 {*color_slider -= 0.03} else {*color_slider = 0.0}

                        for child in &children {
                            if let Ok(mut text) = text_query.get_mut(*child) {
                                let color = tween_color_hsla_short(MAIN_MENU_COLOR_STANDBY, GLOBAL_COLOR_HOVER, *color_slider);
                                text.sections[0].style.color = color;
                                sprite.color.set_r(color.r());
                                sprite.color.set_g(color.g());
                                sprite.color.set_b(color.b());
                            }
                        }

                    }
                }

                match data.f32s.get_mut("window_x") {
                    Option::None => (),
                    Option::Some(window_x) => {

                        if *window_x > 0.0 {*window_x -= 1.0} else {*window_x = 0.0}
                        let value = *window_x;
                        let window = widget.layout_get_mut().expect_window_mut();
                        window.relative.x = value;
                    }
                }

            }
        }
    }
}

//# Wrap it into plugin for code clarity
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (button_update, button_update_decoration).chain());
    }
}




//################################################################################
//# == Smooth Wiggle effect ==
//# Here are just some basic systems to update widgets layout in a stacked sine wawe to immitate natural camera movement
#[derive(Component, Default)]
pub struct SmoothWiggle {
    pub x: f32,
    pub y: f32,
}
fn smooth_wiggle (mut query: Query<(&mut SmoothWiggle, &mut Transform)>) {
    for (mut smoothslider, mut transform) in &mut query {
        smoothslider.x += 0.005;
        smoothslider.y += 0.003;
        transform.translation.x = smoothslider.x.sin()*9.;
        transform.translation.y = smoothslider.y.sin()*3.;
    }
}
fn smooth_wiggle_widget (mut query: Query<(&mut SmoothWiggle, &Widget)>, mut systems: Query<&mut Hierarchy>) {
    let mut system = systems.get_single_mut().unwrap();
    for (mut smoothslider, widget) in &mut query {
        
        let pos = widget.fetch_mut(&mut system, "").unwrap().layout_get_mut().expect_window_mut();
        smoothslider.x += 0.007;
        smoothslider.y += 0.002;

        pos.relative.x = -5.0 + smoothslider.x.sin()*1.3*2.;
        pos.relative.y = -5.0 + smoothslider.y.sin()*1.0*2.;
    }
}

//# Wrap it into plugin for code clarity
pub struct WigglePlugin;
impl Plugin for WigglePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, smooth_wiggle)
            .add_systems(Update, smooth_wiggle_widget);
    }
}
