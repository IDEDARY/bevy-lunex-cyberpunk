pub mod prelude {
    pub use std::borrow::Borrow;
    pub use bevy_lunex::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;

    pub use crate::UiComponent;
    pub use crate::MyData;
    pub use crate::interface::*;
}
use prelude::*;
import_use!(vfx, interface);

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
        .add_systems(Startup, (setup, apply_deferred).chain())

        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {

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
        Cursor::new().with_os_cursor(false).add_sprite_offset(Vec2::splat(14.0)).add_sprite_offset(Vec2::new(10.0, 12.0)).add_sprite_offset(Vec2::splat(40.0)).with_cursor_index(2),
        SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(asset_server.load("cursor.png"), Vec2::splat(80.0), 3, 1, None, None)),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 800.0), scale: Vec3::new(0.4, 0.4, 1.0), ..default() },
            sprite: TextureAtlasSprite {
                color: Color::rgba(252./255., 226./255., 8./255., 2.0).with_l(0.68),
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    let mut tree: UiTree<MyData> = UiTree::new("Interface");

    rt::Menu.construct(&mut commands, &asset_server, &mut tree, ".", ()).unwrap();

    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}

/// # My Data
/// This struct is used to define which data my widgets will need to access and share across the UiTree
#[derive(Debug, Clone, Component, Default)]
pub struct MyData {
    pub animate: bool,
}


/// # My Data
/// This struct is used to define which data my widgets will need to access and share across the UiTree
pub trait UiComponent: {
    fn construct<T:Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError>;
}