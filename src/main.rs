use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() {
    App::new()
        .add_plugins((default_plugins(), UiPlugin::<NoData, NoData, MenuUi>::new()))
        //.add_plugins(UiDebugPlugin::<NoData, NoData, MenuUi>::new())
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
            UiLayout::Solid::new().size(Abs((2968.0, 1656.0))).cover(Cover::Full).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));

        // Spawn the board
        let board = root.add("Board");
        ui.spawn((
            MenuUi,
            board.clone(),
            UiLayout::Solid::new().size(Abs((896.0, 1656.0))).align_x(Align(-0.74)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));

        // Spawn the logo
        /* ui.spawn((
            MenuUi,
            board.add("Boundary"),
            UiLayout::Window::new().pos(Prc((0.0, 12.0))).size(Prc((105.0, 20.0))).pack(),
        ));
        ui.spawn((
            MenuUi,
            board.add("Boundary/Logo"),
            UiLayout::Solid::new().size(Abs((1240.0, 381.0))).pack(),
            UiImage2dBundle::from(assets.main_logo.clone())
        )); */

        ui.spawn((
            MenuUi,
            board.add("Boundary"),
            UiLayout::Window::new().pos(Prc((-10.0, 10.0))).size(Prc((117.0, 28.0))).pack(),
        ));
        ui.spawn((
            MenuUi,
            board.add("Boundary/Logo"),
            UiLayout::Solid::new().size(Abs((1120.0, 474.0))).pack(),
            UiImage2dBundle::from(assets.main_logo.clone())
        ));

        // Spawn the buttons
        ui.spawn((
            MenuUi,
            board.add("List"),
            UiLayout::Window::new().pos(Prc((23.0, 38.0))).size(Prc((54.0, 42.0))).pack(),
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Continue"),
            UiLayout::Window::new().pos(Prc(0.0)).size(Prc((100.0, 15.0))).pack(),
            UiImage2dBundle {
                texture: assets.button.clone(),
                sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(20.0), ..default() }),
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Continue/Text"),
            UiLayout::Window::FULL.pack(),
            UiText2dBundle {
                text: Text::from_section("CONTINUE",
                    TextStyle {
                        font: assets.font_regular.clone(),
                        font_size: 36.0,
                        color: Color::BEVYPUNK_RED,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Load game/Text"),
            UiLayout::Window::FULL.pack(),
            UiText2dBundle {
                text: Text::from_section("LOAD GAME",
                    TextStyle {
                        font: assets.font_regular.clone(),
                        font_size: 36.0,
                        color: Color::BEVYPUNK_YELLOW,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Settings/Text"),
            UiLayout::Window::FULL.pack(),
            UiText2dBundle {
                text: Text::from_section("SETTINGS",
                    TextStyle {
                        font: assets.font_regular.clone(),
                        font_size: 36.0,
                        color: Color::BEVYPUNK_RED,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Credits/Text"),
            UiLayout::Window::FULL.pack(),
            UiText2dBundle {
                text: Text::from_section("CREDITS",
                    TextStyle {
                        font: assets.font_medium.clone(),
                        font_size: 36.0,
                        color: Color::BEVYPUNK_RED,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Quit game/Text"),
            UiLayout::Window::FULL.pack(),
            UiText2dBundle {
                text: Text::from_section("QUIT GAME",
                    TextStyle {
                        font: assets.font_medium.clone(),
                        font_size: 36.0,
                        color: Color::BEVYPUNK_RED,
                    }),
                ..default()
            }
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Load game"),
            UiLayout::Window::new().pos(Prc((0.0, 17.0))).size(Prc((100.0, 15.0))).pack(),
            UiImage2dBundle {
                texture: assets.button.clone(),
                sprite: Sprite { color: Color::BEVYPUNK_YELLOW, ..default() },
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(20.0), ..default() }),
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Settings"),
            UiLayout::Window::new().pos(Prc((0.0, 34.0))).size(Prc((100.0, 15.0))).pack(),
            UiImage2dBundle {
                texture: assets.button.clone(),
                sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(20.0), ..default() }),
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Credits"),
            UiLayout::Window::new().pos(Prc((0.0, 51.0))).size(Prc((100.0, 15.0))).pack(),
            UiImage2dBundle {
                texture: assets.button.clone(),
                sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(20.0), ..default() }),
        ));

        ui.spawn((
            MenuUi,
            board.add("List/Quit game"),
            UiLayout::Window::new().pos(Prc((0.0, 68.0))).size(Prc((100.0, 15.0))).pack(),
            UiImage2dBundle {
                texture: assets.button.clone(),
                sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(20.0), ..default() }),
        ));

        /* ui.spawn((
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
        )); */




    });

}

