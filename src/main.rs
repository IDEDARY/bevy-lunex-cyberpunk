use std::{borrow::Borrow, marker::PhantomData};
use bevy_lunex::prelude::*;
use bevy::prelude::*;

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
                    mode: bevy::window::WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        
        // Lunex boilerplate
        .add_plugins(LunexUiPlugin2D::<MyData>(PhantomData))
        //.add_plugins(LunexUiDebugPlugin2D::<MyData>(PhantomData))

        // Lunex logic
        .add_plugins(InterfacePlugin::<MyData>(PhantomData))

        // Game logic
        .add_plugins(VFXPlugin)
        .add_systems(Startup, (setup, apply_deferred).chain())

        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Query<(&mut Window, Entity)>) {

    // Start playing the main menu music
    commands.spawn(
        AudioBundle {
            source: asset_server.load("sounds/main_menu.ogg"),
            //source: asset_server.load("sounds/AffectEffect_VThemeCover.ogg"),
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
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 800. }, scale: Vec3 { x: 0.4, y: 0.4, z: 1. }, ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 2.0),
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        }
    ));

    let mut tree: UiTree<MyData> = UiTree::new("Interface");

    rt::Menu.construct(&mut commands, &asset_server, &mut tree, ".").unwrap();

    let _window = window.get_single_mut().unwrap();
    commands.entity(_window.1).insert((tree, Transform::default(), Size::default()));
}

#[derive(Component, Default)]
pub struct MyData {
    pub animate: bool,
}
pub trait UiComponent: {
    fn construct<T:Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>) -> Result<Widget, LunexError>;
}