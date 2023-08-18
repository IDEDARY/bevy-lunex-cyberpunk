use bevy::audio::Volume;
use bevy_lunex::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

//# Mod and Import
mod general;
use general::*;
mod style;
use style::*;
mod ui_settings;
use ui_settings::*;
mod ui_mainmenu;
use ui_mainmenu::*;

//# For visual effects only
use bevy::core_pipeline::bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use rand::Rng;



fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Lunex Cyberpunk".into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..Default::default()
                }),
                ..Default::default()
            }
        ), bevy::diagnostic::FrameTimeDiagnosticsPlugin ))

        //Actual setup
        .add_systems(Startup, (setup, apply_deferred).chain())

        //Debug
        //.add_plugins(LunexDebugPlugin)


        .add_systems(Update, (hierarchy_update, cursor_update).chain().before(image_update))

        //UI_Settings plugins
        .add_plugins(UISettingsPlugin)
        .add_plugins(UIMainMenuPlugin)

        // UI widgets update
        .add_plugins(AlignPlugin)
        .add_plugins(GeneralWidgetPlugin)
        .add_plugins(HoverEffectPlugin)
        

        //GLOBAL VFX update
        .add_systems(Update, vfx_bloom_update)

        // App functionality
        .add_systems(Update, profiler_update)


        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //# Start playing the main menu music
    commands.spawn(
        AudioBundle {
            //source: asset_server.load("sounds/main_menu.ogg"),
            source: asset_server.load("sounds/AffectEffect_VThemeCover.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.5)),
        }
    );
    

    //# Spawn the camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1000. },
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings {
            intensity: 0.20,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 0.9,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.25,
                threshold_softness: 0.1,
            },
            composite_mode: BloomCompositeMode::Additive,
        }
    ));

    //# Spawn cursor
    commands.spawn ((
        Cursor::new(10.0),
        SpriteBundle {
            texture: asset_server.load("cursor_mouse.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 800. } , scale: Vec3 { x: 0.4, y: 0.4, z: 1. }, ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 2.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
        FastFlickerEffect::new(0.02, 1.5, 2.5)
    ));


    let mut system = Hierarchy::new();
    setup_main_menu(&mut commands, &asset_server, &mut system);
    setup_menu_settings(&mut commands, &asset_server, &mut system);
    setup_profiler(&mut commands, &asset_server, &mut system);
    
    //################################################################################
    //# == Hierarchy Debug ==
    //# This will print out both "normal" and "debug" maps (It's like "ls" command on Linux). The difference is that "debug" will also print out "nameless" widgets.
    //# "Nameless" widgets are hidden because they are NOT IMPORTANT to the main functionality of the system, but are there only for layout purposes.
    //# Displaying them would be considered overwhelming.
    println!("{}", system.get_map_debug());
    println!("{}", system.get_map());

    //# spawn the finished system
    commands.spawn ((
        system,
        UserInterface { offset: Vec2::default()}
    ));

}


//################################################################################
//# == Bloom Update ==
//# Just a quick system to randomly change bloom threshold (smoothly)
//# It adds another dynamic layer to static camera
fn vfx_bloom_update (mut query: Query<&mut BloomSettings>) {
    for mut bloom in &mut query {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) > 20 {break;}

        bloom.intensity += (rng.gen_range(0.20..0.25)-bloom.intensity)/5.;
        bloom.prefilter_settings.threshold += (rng.gen_range(0.25..0.30)-bloom.prefilter_settings.threshold)/5.;
    }
}