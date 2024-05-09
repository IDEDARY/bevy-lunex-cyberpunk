use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() {
    App::new()
        .add_plugins((default_plugins(), DefaultPickingPlugins, UiPlugin::<NoData, NoData, MenuUi>::new()))
        //.add_plugins(UiDebugPlugin::<NoData, NoData, MenuUi>::new())

        .add_plugins(VFXPlugin)
        .add_systems(PreStartup, cache_assets)
        .add_systems(Startup, startup)

        // Register our button event
        .add_event::<MainMenuButtonAction>()
        .add_systems(Update, main_menu_button_action_system.run_if(on_event::<MainMenuButtonAction>()))

        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetCache>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>) {

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

    // Spawn the master ui tree
    commands.spawn((
        UiTreeBundle::<NoData, NoData, MenuUi>::from(UiTree::new("Bevypunk")),
        MovableByCamera,    // Marks this entity to receive Transform & Dimension updates from camera size
    )).with_children(|ui| {

        // Spawn the root div
        let root = UiLink::path("Root");        // Here we can define the name of the node
        ui.spawn((
            MenuUi,                             // Required marker component
            root.clone(),                       // Here we add the link
            UiLayout::Window::full().pack(),    // This is where we define layout
        ));

        // Spawn the background
        ui.spawn((
            MenuUi,
            root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
            UiLayout::Solid::new().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack(),
            UiImage2dBundle::from(assets.main_background.clone()),  // We use this bundle to add background image to our node
        ));


        // Spawn the board
        let board = root.add("Solid");
        ui.spawn((
            MenuUi,
            board.clone(),
            UiLayout::Solid::new().size((879.0, 1600.0)).align_x(-0.74).pack(), // Just different layout type that preserves aspect ratio
        ));

        let board = board.add("Board");
        ui.spawn((
            MenuUi,
            board.clone(),
            UiLayout::Window::new().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));

        // Spawn the logo
        ui.spawn((
            MenuUi,
            board.add("Boundary"),
            UiLayout::Window::new().y(Rl(13.0)).size(Rl((105.0, 20.0))).pack(),
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
            UiLayout::Window::new().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack(),
        ));

        // Spawn buttons
        let gap = 3.0;
        let size = 14.0;
        let mut offset = 0.0;
        for button in ["CONTINUE", "NEW GAME", "LOAD GAME", "SETTINGS", "ADDITIONAL CONTENT", "CREDITS", "QUIT GAME"] {

            // Spawn button image
            ui.spawn((
                MenuUi,
                list.add(button),
                UiLayout::Window::new().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                UiImage2dBundle {
                    texture: assets.button.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.0), ..default() },
                    ..default()
                },
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),    // Here we make the sprite tillable

                // This is required to make this entity clickable
                PickableBundle::default(),

                // Here we can define what event should be triggered on click
                On::<Pointer<Down>>::send_event::<MainMenuButtonAction>(),

                // Here we can define what happens on hover
                On::<Pointer<Over>>::target_component_mut::<Sprite>(|_, sprite| {
                    sprite.color.set_a(1.0);
                }),
                On::<Pointer<Out>>::target_component_mut::<Sprite>(|_, sprite| {
                    sprite.color.set_a(0.0);
                }),
            ));

            // Spawn button text
            ui.spawn((
                MenuUi,
                list.add(format!("{button}/Text")),

                // Here we can define where we want to position our text within the parent node,
                // don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
                UiLayout::Window::new().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack(),

                // Here we define the text and style
                UiText2dBundle {
                    text: Text::from_section(button,
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },
            ));

            offset += gap + size;
        }
    });

}

// Our event that will happen if we click one of the main menu buttons
#[derive(Event)]
struct MainMenuButtonAction {
    important_data: usize,
}

// Implement constructor for our event
impl From<ListenerInput<Pointer<Down>>> for MainMenuButtonAction {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        let _target = value.target();
        let _listener = value.listener();
        MainMenuButtonAction {
            important_data: value.event.button as usize,
        }
    }
}

// System that will resolve our event
fn main_menu_button_action_system(mut events: EventReader<MainMenuButtonAction>) {
    for event in events.read() {
        info!("Doing complex things with data: {}", event.important_data)
    }
}