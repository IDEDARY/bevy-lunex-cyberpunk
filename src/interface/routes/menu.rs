use std::borrow::Borrow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;

use crate::MyData;
use crate::UiComponent;
use crate::components as ui;
use crate::logic as lg;

// COMPONENT

#[derive(Default)]
pub struct Menu;
impl UiComponent for Menu {
    fn construct<T: Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {

        let menu = RelativeLayout::new().build(tree, "Menu")?;

        let image = SolidLayout::new()
            .with_scaling(SolidScale::Fill)
            .with_size(2560.0, 1440.0)
            .build(tree, menu.end(".background image"))?;
        commands.spawn(ImageElementBundle::new(image, ImageParams::default().with_depth(-0.5), asset_server.load("images/main_menu/53_.png"), Vec2::new(1920.0, 1080.0)));
        
        let board = SolidLayout::new()
            .with_size(807.0, 1432.0)
            .with_horizontal_anchor(-0.8)
            .build(tree, menu.end("Board"))?;
        commands.spawn(ImageElementBundle::new(&board, ImageParams::default(), asset_server.load("images/main_menu/board.png"), Vec2::new(807.0, 1432.0)));
        
        let boundary = RelativeLayout::new()
            .with_rel_1(Vec2::new(-5.0, 12.0))
            .with_rel_2(Vec2::new(105.0, 32.0))
            .build(tree, board.end("boundary"))?;

        let logo = SolidLayout::new()
            .with_size(1240.0, 381.0)
            .build(tree, boundary.end("Logo"))?;
        commands.spawn(ImageElementBundle::new(logo, ImageParams::default(), asset_server.load("images/main_menu/bevypunk2.png"), Vec2::new(1240.0, 381.0)));


        let list = RelativeLayout::new()
            .with_rel_1(Vec2::new(17.0, 33.0))
            .with_rel_2(Vec2::new(82.0, 79.0))
            .build(tree, board.end("list"))?;


        let mut segment = GridSegment::new();
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CONTINUE"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "NEW GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "LOAD GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "SETTINGS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "ADDITIONAL CONTENT"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CREDITS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "QUIT GAME"));
        let widget_list = segment.add_gaps(1.0).build_in(tree, list, GridOrientation::Vertical)?;

        let array = [
            MainMenuButton::Continue,
            MainMenuButton::NewGame,
            MainMenuButton::LoadGame,
            MainMenuButton::Settings,
            MainMenuButton::AdditionalContent,
            MainMenuButton::Credits,
            MainMenuButton::QuitGame
        ];

        let mut i = 0;
        for x in widget_list {

            // These components will get passed to the button entities
            let button_components = (
                lg::AnimateWindowPosition::new(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0)),
                SyncAnimationInput
            );

            // This will create a new widget with preset logic components + custom button_components
            ui::Button::new(x.name()).construct(commands, asset_server, tree, x.end(".Button"), button_components)?;
            
            // Spawn logic for the stationary widget, the one that owns ".Button"
            commands.spawn((
                x,
                array[i],
                lg::InputMouseHover::new()
            ));

            i += 1;
        }

        println!("{}", tree.tree());
        Ok(Widget::new(""))
    }
}




// BOILERPLATE

pub (super) struct MenuPlugin<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default> Plugin for MenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, main_menu_button_actions::<T>)
           .add_systems(Update, main_menu_button_position)
           .add_systems(Update, synchronize_animation_input);
    }
}

// SCRIPT

#[derive(Component, Clone, Copy)]
enum MainMenuButton {
    Continue,
    NewGame,
    LoadGame,
    Settings,
    AdditionalContent,
    Credits,
    QuitGame,
}
/// What to do when the button is pressed
fn main_menu_button_actions<T:Component + Default>(
    mut trees: Query<&mut UiTree<T>>,
    cursors: Query<&Cursor>,
    mut query: Query<(&Widget, &MainMenuButton)>,

    mouse_button_input: Res<Input<MouseButton>>,
    mut exit: EventWriter<bevy::app::AppExit>
) {
    for tree in &mut trees {
        for (widget, category) in &mut query {

            if !widget.fetch(&tree).unwrap().is_visible() {return;}
            if !mouse_button_input.just_pressed(MouseButton::Left) {return;}

            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.position_world().invert_y()).unwrap() {
                    trigger = true;
                    break;
                }
            }

            if trigger {
                match category {
                    MainMenuButton::QuitGame => {
                        exit.send(bevy::app::AppExit);
                    },
                    _ => {},
                }
            }
        }
    }
}

/// Trigger the hover effects of the owned button
fn main_menu_button_position(mut trees: Query<&mut UiTree<MyData>>, query: Query<(&Widget, &lg::InputMouseHover), With<MainMenuButton>>) {
    for mut tree in &mut trees {
        for (source, input) in &query {
            let data: &mut MyData = source.fetch_mut_ext(&mut tree, ".Button").unwrap().get_data_mut();
            data.animate = input.hover
        }
    }
}


#[derive(Component, Clone)]
struct SyncAnimationInput;
fn synchronize_animation_input(mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&Widget, &mut lg::Animate), With<SyncAnimationInput>>) {
    for mut tree in &mut trees {
        for (source, mut destination) in &mut query {
            let data: &MyData = source.fetch_mut(&mut tree).unwrap().get_data();
            destination.trigger = data.animate;
        }
    }
}