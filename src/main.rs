use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() {
    App::new()
        .add_plugins((default_plugins(), UiPlugin::<NoData, NoData, MenuUi>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MenuUi>::new())
        .add_systems(PreStartup, presetup)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetCache>) {

    // Spawn camera
    commands.spawn(( MenuUi, Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() } ));

    // Spawn the master ui
    commands.spawn((
        UiTreeBundle::<NoData, NoData, MenuUi>::from(UiTree::new("Bevypunk")),
        MovableByCamera,
    )).with_children(|parent| {

        // Spawn the root div
        let root = UiLink::path("Root");
        parent.spawn((
            MenuUi,
            root.clone(),
            UiLayout::Window::FULL.pack(),
        ));

        // Spawn the background
        parent.spawn((
            MenuUi,
            root.add("Background"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).cover(Cover::Full).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));

        // Spawn the board
        parent.spawn((
            MenuUi,
            root.add("Board"),
            UiLayout::Solid::new().size(Abs((807.0, 1432.0))).align_x(Align(-0.8)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));


        /* parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::SM).br().pack(),
            /*UiText2dBundle {
                text: Text::from_section("hello world!",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::RED,
                    }),
                ..default()
            }*/
            //UiImage2dBundle::from(assets.main_background.clone())
        )); */




    });

}

