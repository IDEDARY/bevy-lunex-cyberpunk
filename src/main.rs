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
            UiLayout::Solid::new().size((2968.0, 1656.0)).cover(Cover::Fill).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));


        // Spawn the board
        let board = root.add("Board");
        ui.spawn((
            MenuUi,
            board.clone(),
            UiLayout::Solid::new().size((896.0, 1656.0)).align_x(-0.74).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));

        // Spawn the logo
        ui.spawn((
            MenuUi,
            board.add("Boundary"),
            UiLayout::Window::new().pos(Prc((0.0, 13.0))).size(Prc((105.0, 20.0))).pack(),
        ));
        ui.spawn((
            MenuUi,
            board.add("Boundary/Logo"),
            UiLayout::Solid::new().size((1240.0, 381.0)).pack(),
            UiImage2dBundle::from(assets.main_logo.clone())
        ));

        // Spawn button boundary
        let list = board.add("List");
        ui.spawn((
            MenuUi,
            list.clone(),
            UiLayout::Window::new().pos(Prc((22.0, 41.0))).size(Prc((55.0, 35.0))).pack(),
        ));

        // Spawn buttons
        let gap = 3.0;
        let size = 14.0;
        let mut offset = 0.0;
        for button in ["CONTINUE", "LOAD GAME", "SETTINGS", "CREDITS", "QUIT GAME"] {

            // Spawn button image
            ui.spawn((
                MenuUi,
                list.add(button),
                UiLayout::Window::new().pos(Prc((0.0, offset))).size(Prc((100.0, size))).pack(),
                UiImage2dBundle {
                    texture: assets.button.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED_DIM, ..default() },
                    ..default()
                },
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
            ));
    
            // Spawn button text
            ui.spawn((
                MenuUi,
                list.add(format!("{button}/Text")),
                UiLayout::Solid::new().align_x(-0.85).pack(),
                //UiLayout::Div::new().margin_l(Prc(5.0)).br().pack(),
                //UiContent::default(),
                UiText2dBundle {
                    text: Text::from_section(button,
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 50.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                }
            ));

            offset += gap + size;
        }
    });

}
