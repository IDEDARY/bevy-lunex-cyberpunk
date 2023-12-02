use std::f32::consts::PI;
use crate::prelude::*;

/// # Main Menu
/// The callable route struct containing the whole main menu.
#[derive(Default)]
pub struct Menu;
impl Menu {
    pub fn construct<T:Component + Default>(commands: &mut Commands, assets: &MenuAssetCache, tree: &mut UiTree<T>) -> Result<(), LunexError> {

        let menu = RelativeLayout::new().with_rel_1(Vec2::splat(-1.0)).with_rel_2(Vec2::splat(101.0)).build_as(tree, "Menu")?;

        let wiggle_amplitude = Vec2::new(2.4, 4.0);
        let background = WindowLayout::new().size_rel((100.0 + wiggle_amplitude.x * 2.0, 100.0 + wiggle_amplitude.y * 2.0)).build_in(tree, &menu)?;
        commands.spawn((
            background.clone(),
            WiggleBackgroundWidget {
                speed: Vec2::new(0.005, 0.003),
                amplitude: wiggle_amplitude,
                degree: Vec2::new(PI/6., PI/4.),
            }
        ));

        let image = SolidLayout::new()
            .with_scaling(SolidScale::Fill)
            .with_size(1920.0, 1080.0)
            .build_in(tree, &background)?;

        image.fetch_mut(tree)?.get_container_mut().set_render_depth(Modifier::Set(90.0));
        commands.spawn(ImageElementBundle::new(image, ImageParams::default().with_depth(-0.5), assets.main_background.clone(), Vec2::new(1920.0, 1080.0)));
        
        let board = SolidLayout::new()
            .with_size(807.0, 1432.0)
            .with_horizontal_anchor(-0.75)
            .build_as(tree, menu.end("Board"))?;
        commands.spawn(ImageElementBundle::new(&board, ImageParams::default(), assets.main_board.clone(), Vec2::new(807.0, 1432.0)));
        
        let boundary = RelativeLayout::new()
            .with_rel_1(Vec2::new(-5.0, 12.0))
            .with_rel_2(Vec2::new(105.0, 32.0))
            .build_as(tree, board.end("boundary"))?;

        let logo = SolidLayout::new()
            .with_size(1240.0, 381.0)
            .build_as(tree, boundary.end("Logo"))?;
        commands.spawn(ImageElementBundle::new(logo, ImageParams::default(), assets.main_logo.clone(), Vec2::new(1240.0, 381.0)));

        
        // Define grid boundary
        let list = RelativeLayout::new()
            .with_rel_1(Vec2::new(17.0, 33.0))
            .with_rel_2(Vec2::new(79.0, 78.0))
            .build_as(tree, board.end("list"))?;


        // Define the grid layout
        let mut segment = GridSegment::new();
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CONTINUE"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "NEW GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "LOAD GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "SETTINGS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "ADDITIONAL CONTENT"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CREDITS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "QUIT GAME"));
        

        // Build the grid
        let button_widget_list = segment.add_gaps(1.0).build_in(tree, list, GridOrientation::Vertical)?;

        // Create an array of components to append to each widget in button_widget_list 
        let component_array = [
            MainMenuButton::Continue,
            MainMenuButton::NewGame,
            MainMenuButton::LoadGame,
            MainMenuButton::Settings,
            MainMenuButton::AdditionalContent,
            MainMenuButton::Credits,
            MainMenuButton::QuitGame
        ];

        // Loop over the button widgets and append logic to them
        for i in 0..button_widget_list.len() {

            // These components will get passed to the new button entities
            let button_components = lg::AnimateWindowPosition::new(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0));

            // This will create a new widget with preset logic components + custom button_components
            ui::MainMenuButton::new(button_widget_list[i].name())
                .construct(commands, assets, tree, button_widget_list[i].end(".Button"), button_components)?;
            
            // Spawn logic for the stationary widget, the one that owns ".Button"
            commands.spawn((
                button_widget_list[i].to_owned(),
                component_array[i],
                lg::InputCursorHover::new().request_cursor(1),
                lg::InputMouseClick::new(),
                lg::PipeCursorHoverAsAnimateInput(".Button".into()),
            ));
        }

        Ok(())
    }
}


/// All of custom Main Menu logic
mod script {
    use std::f32::consts::TAU;
    use crate::prelude::*;

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
    pub(super) fn main_menu_button_actions(mut commands: Commands, assets: Res<MenuAssetCache>, mut trees: Query<&mut UiTree<MyData>>, mut query: Query<(&MainMenuButton, &lg::InputMouseClick), With<Widget>>, mut exit: EventWriter<bevy::app::AppExit>) {
        for mut tree in &mut trees {
            for (category, clicked) in &mut query {
                if clicked.left {
                    match category {
                        MainMenuButton::Settings => {
                            tree.drop_branch("Menu").unwrap();
                            rt::Settings::construct(&mut commands, &assets, &mut tree).unwrap();
                            return;
                            //tree.borrow_branch_mut("Menu").unwrap().set_visibility(false);
                            //tree.borrow_branch_mut("Settings").unwrap().set_visibility(true);
                        },
                        MainMenuButton::QuitGame => {
                            exit.send(bevy::app::AppExit);
                        },
                        _ => {},
                    }
                }
            }
        }
    }

    /// Wiggle the background widget
    #[derive(Component, Clone, Default)]
    pub(super) struct WiggleBackgroundWidget {
        pub speed: Vec2,
        pub amplitude: Vec2,
        pub degree: Vec2,
    }
    pub(super) fn wiggle_background_widget_animation<T:Component + Default>(mut trees: Query<&mut UiTree<T>>, mut query: Query<(&Widget, &mut WiggleBackgroundWidget)>) {
        for mut tree in &mut trees {
            for (widget, mut animation) in &mut query {
                animation.degree.x += animation.speed.x;
                animation.degree.y += animation.speed.y;

                if animation.degree.x >= TAU { animation.degree.x -= TAU; }
                if animation.degree.y >= TAU { animation.degree.y -= TAU; }

                let container = match widget.fetch_mut(&mut tree){
                    Ok(d) => d,
                    Err(_) => continue,
                }.get_container_mut();
                let window = container.get_layout_mut().expect_window_mut();
                window.pos_relative.x = animation.degree.x.sin()*animation.amplitude.x - animation.amplitude.x;
                window.pos_relative.y = animation.degree.y.sin()*animation.amplitude.y - animation.amplitude.y;
            }
        }
    }
}
use script::*;
script_plugin!(MenuPlugin,
    add_systems(Update, main_menu_button_actions.after(lg::InputSystemSet).before(LunexUiSystemSet2D)),
    add_systems(Update, wiggle_background_widget_animation::<T>)
);