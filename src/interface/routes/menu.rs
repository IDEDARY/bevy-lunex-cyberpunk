use std::borrow::Borrow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;

use crate::UiComponent;
use crate::components as ui;
use crate::logic as lg;

// COMPONENT

#[derive(Default)]
pub struct Menu;
impl UiComponent for Menu {
    fn construct<T: Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>) -> Result<Widget, LunexError> {

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
            ui::Button::new(x.name()).construct(commands, asset_server, tree, x.end(".Button"))?;
            commands.spawn((
                x,
                array[i],
                lg::AnimateWindowPosition::new(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0)),
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
        app.add_systems(Update, main_menu_button_system::<T>)
           .add_systems(Update, main_menu_button_position);
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
fn main_menu_button_system<T:Component + Default>(
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
fn main_menu_button_position(mut query: Query<(&mut lg::Animate, &lg::InputMouseHover), With<MainMenuButton>>) {
    for (mut source1, source2) in &mut query {
        if source2.hover { source1.value = 1.0 }
    }
}