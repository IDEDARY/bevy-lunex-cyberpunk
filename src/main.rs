pub mod prelude {
    // Bevy + Bevy_Lunex
    pub use bevy::prelude::*;
    pub use bevy_lunex::prelude::*;
    
    // STD + Usefull stuff
    pub use std::borrow::Borrow;
    pub use bevy::window::PrimaryWindow;

    // Global access to this data
    pub use crate::MyData;
    pub use crate::MenuAssetCache;
    pub use crate::interface::*;
    pub use crate::{COLOR_PRIMARY, COLOR_SECONDARY};
}
use prelude::*;
import_use!(vfx, interface);

pub const BEVYPUNK_RED: Color = Color::rgba(255./255., 98./255., 81./255., 1.0);
pub const BEVYPUNK_YELLOW: Color = Color::rgba(252./255., 226./255., 8./255., 1.0);

pub const COLOR_PRIMARY: Color = BEVYPUNK_RED;
pub const COLOR_SECONDARY: Color = BEVYPUNK_YELLOW;

fn main() {
    App::new()
        // Game boilerplate
        .add_plugins((DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevypunk".into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }
        ), bevy::diagnostic::FrameTimeDiagnosticsPlugin))
        
        // Lunex boilerplate
        .add_plugins(LunexUiPlugin2D::<MyData>::new())
        //.add_plugins(LunexUiDebugPlugin2D::<MyData>::new())

        // Lunex logic
        .add_plugins(InterfacePlugin::<MyData>::new())

        // Game logic
        .add_plugins(VFXPlugin)
        .add_systems(PreStartup, presetup)
        .add_systems(Startup, setup)

        .run();
}

/// This function loads all big assets into memory to awoid waiting for async load times
fn presetup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // LOAD ALL ASSETS INTO CACHE
    commands.insert_resource(MenuAssetCache {
        font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),
        button: asset_server.load("images/main_menu/button.png"),
        main_background: asset_server.load("images/main_menu/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),
        settings_background: asset_server.load("images/settings/background.png"),
    });
}

/// This function is RUN on start of the app
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, assets: Res<MenuAssetCache>, mut textures: ResMut<Assets<TextureAtlas>>, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {

    // Start playing the main menu music
    commands.spawn(
        AudioBundle {
            source: asset_server.load("sounds/main_menu.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new_relative(0.5)),
        }
    );
    
    // Spawn the camera
    commands.spawn(camera());

    // Spawn cursor
    commands.spawn ((
        Cursor::new().with_os_cursor(false).add_sprite_offset(Vec2::splat(14.0)).add_sprite_offset(Vec2::new(10.0, 12.0)).add_sprite_offset(Vec2::splat(40.0)),
        SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(asset_server.load("images/cursor.png"), Vec2::splat(80.0), 3, 1, None, None)),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 800.0), scale: Vec3::new(0.5, 0.5, 1.0), ..default() },
            sprite: TextureAtlasSprite {
                color: COLOR_SECONDARY.with_a(2.0).with_l(0.68),
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    // Create new UiTree (a UI context / DOM)
    let mut tree: UiTree<MyData> = UiTree::new("Interface");

    // Construct the route Menu first
    rt::Menu::construct(&mut commands, &assets, &mut tree).unwrap();

    // Print nice debug tree in console
    println!("{}", tree.tree());

    // Insert the UI into the window
    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}

/// # My Data
/// This struct is used to define which data my widgets will need to access and share across the UiTree
#[derive(Debug, Clone, Component, Default)]
pub struct MyData {
    pub animate: bool,
}

/// # Menu Asset Cache
/// On PreStartup, load all UI assets ahead of time.
/// 
/// Makes it more smooth when dynamically building UI.
#[derive(Resource)]
pub struct MenuAssetCache {
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub button: Handle<Image>,
    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,
    pub settings_background: Handle<Image>,
}