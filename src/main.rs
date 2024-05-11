use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

mod boilerplate;
use boilerplate::*;

mod components;
use components::*;

fn main() {
    App::new()
        .add_plugins((default_plugins(), DefaultPickingPlugins, UiGeneralPlugin, UiPlugin::<MenuUi>::new()))
        //.add_plugins(UiDebugPlugin::<MenuUi>::new())
        .add_plugins(MainButtonPlugin)

        .add_plugins(VFXPlugin)
        .add_systems(PreStartup, cache_assets)
        .add_systems(Startup, startup)

        // React to button press
        .add_systems(Update, main_menu_button_action_system.run_if(on_event::<MainButtonClick>()))

        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetCache>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>) {


    // #=====================#
    // #=== GENERIC SETUP ===#

    // Spawn camera
    commands.spawn(camera()).with_children(|camera| {

        // Spawn cursor
        camera.spawn ((

            // Here we can map different native cursor icons to texture atlas indexes and sprite offsets
            Cursor2d::new().native_cursor(false)
                .register_cursor(CursorIcon::Default, 0, (14.0, 14.0))
                .register_cursor(CursorIcon::Copy, 1, (10.0, 12.0))
                .register_cursor(CursorIcon::Grab, 2, (40.0, 40.0)),

            // Add texture atlas to the cursor
            SpriteSheetBundle {
                texture: assets.cursor.clone(),
                atlas: TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(Vec2::splat(80.0), 3, 1, None, None)),
                    index: 0,
                },
                transform: Transform { scale: Vec3::new(0.45, 0.45, 1.0), ..default() },
                sprite: Sprite {
                    color: Color::BEVYPUNK_YELLOW.with_a(2.0).with_l(0.68),
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },

            // Make the raycaster ignore this entity, we don't want our cursor to block clicking
            Pickable::IGNORE,
        ));
    });

    // Spawn audio
    commands.spawn( AudioBundle { source: assets.music.clone(), settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.5)) } );


    // #======================#
    // #=== USER INTERFACE ===#

    // Spawn the master ui tree
    commands.spawn((
        UiTreeBundle::<MenuUi>::from(UiTree::new("Bevypunk")),
        MovableByCamera,    // Marks this entity to receive Transform & Dimension updates from camera size
    )).with_children(|ui| {

        // Spawn the root div
        let root = UiLink::<MenuUi>::path("Root");  // Here we can define the name of the node
        ui.spawn((
            root.clone(),                           // Here we add the link
            UiLayout::window_full().pack(),         // This is where we define layout
        ));

        // Spawn the background
        ui.spawn((
            root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
            UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack(),
            UiImage2dBundle::from(assets.main_background.clone()),  // We use this bundle to add background image to our node
        ));


        // Spawn the board
        let board = root.add("Solid");
        ui.spawn((
            board.clone(),
            UiLayout::solid().size((879.0, 1600.0)).align_x(-0.74).pack(), // Just different layout type that preserves aspect ratio
        ));

        let board = board.add("Board");
        ui.spawn((
            board.clone(),
            UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));


        // Spawn the logo
        ui.spawn((
            board.add("Boundary"),
            UiLayout::window().y(Rl(13.0)).size(Rl((105.0, 20.0))).pack(),
        ));
        ui.spawn((
            board.add("Boundary/Logo"),
            UiLayout::solid().size((1240.0, 381.0)).pack(),
            UiImage2dBundle::from(assets.main_logo.clone())
        ));


        // #=========================#
        // #=== MAIN MENU BUTTONS ===#

        // Spawn button boundary
        let list = board.add("List");
        ui.spawn((
            list.clone(),
            UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack(),
        ));

        // Spawn buttons
        let gap = 3.0;
        let size = 14.0;
        let mut offset = 0.0;
        for button in [
            MainMenuButton::Continue,
            MainMenuButton::NewGame,
            MainMenuButton::LoadGame,
            MainMenuButton::Settings,
            MainMenuButton::AdditionalContent,
            MainMenuButton::Credits,
            MainMenuButton::QuitGame,
        ] {

            ui.spawn((
                list.add(button.str()),
                UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                MainButton { text: button.str().into() },
                button.clone(),
            ));

            offset += gap + size;
        }
    });

}


// #======================================#
// #=== MAIN MENU BUTTON INTERACTIVITY ===#

#[derive(Component, Clone)]
enum MainMenuButton {
    Continue,
    NewGame,
    LoadGame,
    Settings,
    AdditionalContent,
    Credits,
    QuitGame,
}
impl MainMenuButton {
    fn str(&self) -> String {
        match self {
            MainMenuButton::Continue => "CONTINUE".into(),
            MainMenuButton::NewGame => "NEW GAME".into(),
            MainMenuButton::LoadGame => "LOAD GAME".into(),
            MainMenuButton::Settings => "SETTINGS".into(),
            MainMenuButton::AdditionalContent => "ADDITIONAL CONTENT".into(),
            MainMenuButton::Credits => "CREDITS".into(),
            MainMenuButton::QuitGame => "QUIT GAME".into(),
        }
    }
}

// System that will resolve our event
fn main_menu_button_action_system(mut events: EventReader<MainButtonClick>, query: Query<&MainMenuButton, With<MainButton>>, mut exit: EventWriter<bevy::app::AppExit>) {
    for event in events.read() {
        if let Ok(button) = query.get(event.target) {

            info!("Pressed: {}", button.str());

            // Here we can do our logic for each button
            match button {
                MainMenuButton::Continue => {},
                MainMenuButton::NewGame => {},
                MainMenuButton::LoadGame => {},
                MainMenuButton::Settings => {},
                MainMenuButton::AdditionalContent => {},
                MainMenuButton::Credits => {},
                MainMenuButton::QuitGame => {
                    exit.send(bevy::app::AppExit);
                },
            }
        }
    }
}