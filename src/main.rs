use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() {
    App::new()
        .add_plugins((default_plugins(), UiPlugin::<NoData, NoData, MenuUi>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MenuUi>::new())
        .add_plugins(VFXPlugin)
        .add_systems(PreStartup, presetup)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetCache>) {

    // Spawn camera
    commands.spawn(camera());

    // Spawn audio
    commands.spawn( AudioBundle { source: assets.music.clone(), settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.5)) } );

    // Spawn the master ui
    commands.spawn((
        UiTreeBundle::<NoData, NoData, MenuUi>::from(UiTree::new("Bevypunk")),
        MovableByCamera,
    )).with_children(|ui| {

        // Spawn the root div
        let root = UiLink::path("Root");
        ui.spawn((
            MenuUi,
            root.clone(),
            UiLayout::Window::FULL.pack(),
        ));

        // Spawn the background
        ui.spawn((
            MenuUi,
            root.add("Background"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).cover(Cover::Full).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));

        // Spawn the board
        let board = root.add("Board");
        ui.spawn((
            MenuUi,
            board.clone(),
            UiLayout::Solid::new().size(Abs((807.0, 1432.0))).align_x(Align(-0.8)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));

        // Spawn the logo
        ui.spawn((
            MenuUi,
            board.add("Boundary"),
            UiLayout::Window::new().pos(Prc((-5.0, 12.0))).size(Prc((110.0, 20.0))).pack(),
        ));
        ui.spawn((
            MenuUi,
            board.add("Boundary/Logo"),
            UiLayout::Solid::new().size(Abs((1240.0, 381.0))).pack(),
            UiImage2dBundle::from(assets.main_logo.clone())
        ));

        // Spawn the buttons
        ui.spawn((
            MenuUi,
            board.add("List"),
            UiLayout::Window::new().pos(Prc((17.0, 33.0))).size(Prc((62.0, 45.0))).pack(),
        ));
        ui.spawn((
            MenuUi,
            board.add("List/Text"),
            UiLayout::Div::new().margin_t(Abs::SM).br().pack(),
            UiContent::new((220.0, 35.0)),
            UiText2dBundle {
                text: Text::from_section("hello world!",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::YELLOW,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Text2"),
            UiLayout::Div::new().margin_t(Abs::SM).br().pack(),
            UiContent::new((220.0, 35.0)),
            UiText2dBundle {
                text: Text::from_section("hello world!",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::YELLOW,
                    }),
                ..default()
            }
        ));




    });

}

