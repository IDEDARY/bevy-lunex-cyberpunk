use std::borrow::Borrow;
use bevy::window::PrimaryWindow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod vfx;
use vfx::*;

mod interface;
use interface::*;


fn main() {
    App::new()
        // Game boilerplate
        .add_plugins(DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Lunex Cyberpunk".into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        
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
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {

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
        Cursor::new(10.0).with_hide_os_cursor(true),
        SpriteBundle {
            texture: asset_server.load("cursor_mouse.png"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 800.0), scale: Vec3::new(0.4, 0.4, 1.0), ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 2.0),
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


#[derive(Debug, Clone, Component, Default)]
pub struct MyData {
    pub animate: bool,
}
pub trait UiComponent: {
    fn construct<T:Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError>;
}