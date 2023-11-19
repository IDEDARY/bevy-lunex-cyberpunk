use crate::prelude::*;

/// # Main Menu
/// The callable UiComponent struct containing the whole main menu.
#[derive(Default)]
pub struct Menu;
impl UiComponent for Menu {
    fn construct<T:Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, _path: impl Borrow<str>, _bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {

        let menu = RelativeLayout::new().build_as(tree, "Menu")?;

        let image = SolidLayout::new()
            .with_scaling(SolidScale::Fill)
            .with_size(2560.0, 1440.0)
            .build_as(tree, menu.end(".background image"))?;
        commands.spawn(ImageElementBundle::new(image, ImageParams::default().with_depth(-0.5), asset_server.load("images/main_menu/background.png"), Vec2::new(1920.0, 1080.0)));
        
        let board = SolidLayout::new()
            .with_size(807.0, 1432.0)
            .with_horizontal_anchor(-0.8)
            .build_as(tree, menu.end("Board"))?;
        commands.spawn(ImageElementBundle::new(&board, ImageParams::default(), asset_server.load("images/main_menu/board.png"), Vec2::new(807.0, 1432.0)));
        
        let boundary = RelativeLayout::new()
            .with_rel_1(Vec2::new(-5.0, 12.0))
            .with_rel_2(Vec2::new(105.0, 32.0))
            .build_as(tree, board.end("boundary"))?;

        let logo = SolidLayout::new()
            .with_size(1240.0, 381.0)
            .build_as(tree, boundary.end("Logo"))?;
        commands.spawn(ImageElementBundle::new(logo, ImageParams::default(), asset_server.load("images/main_menu/bevypunk.png"), Vec2::new(1240.0, 381.0)));


        let list = RelativeLayout::new()
            .with_rel_1(Vec2::new(17.0, 33.0))
            .with_rel_2(Vec2::new(82.0, 79.0))
            .build_as(tree, board.end("list"))?;


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
                PullAnimationInput
            );

            // This will create a new widget with preset logic components + custom button_components
            ui::Button::new(x.name()).construct(commands, asset_server, tree, x.end(".Button"), button_components)?;
            
            // Spawn logic for the stationary widget, the one that owns ".Button"
            commands.spawn((
                x,
                array[i],
                lg::InputMouseClick::new()
            ));

            i += 1;
        }

        println!("{}", tree.tree());
        Ok(Widget::new(""))
    }
}


/// All of custom Main Menu logic
mod script {
    use bevy::prelude::*;
    use bevy_lunex::prelude::*;
    use crate::MyData;
    use crate::logic as lg;

    #[derive(Component, Clone, Copy)]
    pub(super) enum MainMenuButton {
        Continue,
        NewGame,
        LoadGame,
        Settings,
        AdditionalContent,
        Credits,
        QuitGame,
    }
    /// What to do when the button is pressed
    pub(super) fn main_menu_button_actions(mut query: Query<(&MainMenuButton, &lg::InputMouseClick), With<Widget>>, mut exit: EventWriter<bevy::app::AppExit>) {
        for (category, clicked) in &mut query {
            if clicked.left {
                match category {
                    MainMenuButton::QuitGame => {
                        exit.send(bevy::app::AppExit);
                    },
                    _ => {},
                }
            }
        }
    }

    /// Send trigger bool to the MyData of ./.Button widget
    pub(super) fn main_menu_button_trigger_animation(mut trees: Query<&mut UiTree<MyData>>, query: Query<(&Widget, &lg::InputMouseHover), With<MainMenuButton>>) {
        for mut tree in &mut trees {
            for (source, input) in &query {
                let data: &mut MyData = source.fetch_mut_ext(&mut tree, ".Button").unwrap().get_data_mut();
                data.animate = input.hover
            }
        }
    }

    /// Pull trigger bool from MyData (used by ./Button widget)
    #[derive(Component, Clone)]
    pub(super) struct PullAnimationInput;
    pub(super) fn pull_animation_from_main_menu_button(mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&Widget, &mut lg::Animate), With<PullAnimationInput>>) {
        for mut tree in &mut trees {
            for (source, mut destination) in &mut query {
                let data: &MyData = source.fetch_mut(&mut tree).unwrap().get_data();
                destination.trigger = data.animate;
            }
        }
    }
}
use script::*;
script_plugin!(MenuPlugin,
    add_systems(Update, main_menu_button_actions),
    add_systems(Update, main_menu_button_trigger_animation),
    add_systems(Update, pull_animation_from_main_menu_button)
);