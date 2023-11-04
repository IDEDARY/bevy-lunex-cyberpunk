use bevy_lunex::prelude::*;
use bevy::prelude::*;


mod vfx;
use vfx::*;

mod routes;

fn main() {
    App::new()
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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin )
        .add_plugins(LunexUiPlugin)
        .add_plugins(VFXPlugin)

        .add_systems(Startup, (setup, apply_deferred).chain())

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

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
        Cursor::new(10.0),
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
}