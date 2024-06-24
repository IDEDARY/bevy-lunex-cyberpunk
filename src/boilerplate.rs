use bevy::core_pipeline::bloom::BloomSettings;
use bevy::app::PluginGroupBuilder;
use crate::*;


// #=========================================#
// #=== BOILERPLATE REQUIRED FOR BEVYPUNK ===#

/// Custom color palette for Bevypunk
pub trait BevypunkColorPalette {
    const BEVYPUNK_RED: Color;
    const BEVYPUNK_RED_DIM: Color;
    const BEVYPUNK_YELLOW: Color;
    const BEVYPUNK_BLUE: Color;
}
impl BevypunkColorPalette for Color {
    const BEVYPUNK_RED: Color = Color::srgba(255./255., 98./255., 81./255., 1.0);
    const BEVYPUNK_RED_DIM: Color = Color::srgba(172./255., 64./255., 63./255., 1.0);
    const BEVYPUNK_YELLOW: Color = Color::linear_rgba(252./255., 226./255., 8./255., 1.0);
    const BEVYPUNK_BLUE: Color = Color::srgba(8./255., 226./255., 252./255., 1.0);
}

// #======================================#
// #=== ASSET CACHE FOR SMOOTH LOADING ===#

// Load the gif before the app is run
#[derive(Resource)]
pub struct PreLoader {
    //pub intro: Handle<AnimatedGif>,
}

#[allow(dead_code)]
// Load all assets at startup for faster loading during runtime
#[derive(Resource)]
pub struct AssetCache {
    // Music
    pub music: Handle<AudioSource>,
    pub ui_ping: Handle<AudioSource>,

    // Fonts
    pub font_light: Handle<Font>,
    pub font_regular: Handle<Font>,
    pub font_medium: Handle<Font>,
    pub font_semibold: Handle<Font>,
    pub font_bold: Handle<Font>,

    // Cursor
    pub cursor: Handle<Image>,

    // Symbols
    pub button_symetric: Handle<Image>,
    pub button_symetric_sliced: Handle<Image>,
    pub button_sliced_bottom_left: Handle<Image>,
    pub button_sliced_bottom_right: Handle<Image>,
    pub button_sliced_top_left: Handle<Image>,
    pub button_sliced_top_right: Handle<Image>,
    pub chevron_left: Handle<Image>,
    pub chevron_right: Handle<Image>,
    pub switch_base: Handle<Image>,
    pub switch_head: Handle<Image>,

    // Routes
    pub intro_background: Handle<Image>,

    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,

    pub settings_background: Handle<Image>,

    pub character_creator_panel: Handle<Image>,
}
pub fn cache_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetCache {
        // Music
        music: asset_server.load("sounds/main_menu.ogg"),
        ui_ping: asset_server.load("sounds/ui_ping.ogg"),

        // Fonts
        font_light: asset_server.load("fonts/rajdhani/Rajdhani-Light.ttf"),
        font_regular: asset_server.load("fonts/rajdhani/Rajdhani-Regular.ttf"),
        font_medium: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_semibold: asset_server.load("fonts/rajdhani/Rajdhani-SemiBold.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),

        // Cursor
        cursor: asset_server.load("images/cursor.png"),

        // Symbols
        button_symetric: asset_server.load("images/button_symetric.png"),
        button_symetric_sliced: asset_server.load("images/button_symetric_sliced.png"),
        button_sliced_bottom_left: asset_server.load("images/button_sliced_bottom_left.png"),
        button_sliced_bottom_right: asset_server.load("images/button_sliced_bottom_right.png"),
        button_sliced_top_left: asset_server.load("images/button_sliced_top_left.png"),
        button_sliced_top_right: asset_server.load("images/button_sliced_top_right.png"),
        chevron_left: asset_server.load("images/chevron_left.png"),
        chevron_right: asset_server.load("images/chevron_right.png"),
        switch_base: asset_server.load("images/switch_base.png"),
        switch_head: asset_server.load("images/switch_head.png"),

        // Routes
        intro_background: asset_server.load("images/intro/frame0.png"),

        main_background: asset_server.load("images/settings/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),

        settings_background: asset_server.load("images/settings/background.png"),

        character_creator_panel: asset_server.load("images/character_creator/panel.png"),
    });
}


// #======================================#
// #=== JUST SPAWN PRESETS FOR CLARITY ===#

/// Function to return default plugins with correct settings
pub fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set (
        WindowPlugin {
            primary_window: Some(Window {
                title: "Bevypunk".into(),
                mode: bevy::window::WindowMode::Windowed,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }
    ).set (
        bevy::render::RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(
                bevy::render::settings::WgpuSettings {
                    power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                    ..default()
                }
            ),
            ..default()
        }
    ).set(AssetPlugin {
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    })
}

/// Function to return camera will all appropriate settings
pub fn camera() -> impl Bundle {
    (
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera {
                hdr: true,
                ..default()
            },
            //tonemapping: Tonemapping::None,
            ..default()
        },
        VFXBloomFlicker,
        BloomSettings::OLD_SCHOOL,
        InheritedVisibility::default(),
        /*VfxWiggleCamera {
            sinusoid: vec![
                Sine {
                    speed: 0.005,
                    amplitude: 0.003,
                    degree: 0.0,
                }
            ]
        }*/
    )
}


// #===============================#
// #=== VFX LOGIC AND ANIMATION ===#

use std::f32::consts::TAU;
use rand::Rng;

#[derive(Component)]
pub struct VFXBloomFlicker;

/// System for immitating flickering by randomly adjusting cameras bloom values
fn vfx_bloom_flicker(mut query: Query<&mut BloomSettings, With<VFXBloomFlicker>>) {
    for mut bloom in &mut query {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) < 20 {
            bloom.intensity += (rng.gen_range(0.20..0.30)-bloom.intensity)/6.0;
            bloom.prefilter_settings.threshold += (rng.gen_range(0.20..0.30)-bloom.prefilter_settings.threshold)/4.0;
        }
    }
}

#[derive(Clone, Default)]
pub struct Sine {
    pub speed: f32,
    pub amplitude: f32,
    pub degree: f32,
}
impl Sine {
    fn tick(&mut self) {
        self.degree += self.speed; 
        if self.degree >= TAU { self.degree -= TAU; }
        if self.degree < 0.0 { self.degree += TAU; }
    }
    fn get_pure(&self) -> f32 {
        self.degree.sin()*self.amplitude
    }
}

#[derive(Component, Clone, Default)]
pub(super) struct VfxWiggleCamera {
    pub sinusoid: Vec<Sine>
}
fn vfx_camera_wiggle(mut query: Query<(&mut VfxWiggleCamera, &mut Transform)>) {
    for (mut animation, mut transform) in &mut query {
        for sine in &mut animation.sinusoid {
            sine.tick()
        }
        transform.rotation.z = animation.sinusoid[0].get_pure();
    }
}

/// Plugin with VFX systems for our menu
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (vfx_bloom_flicker, vfx_camera_wiggle));
    }
}