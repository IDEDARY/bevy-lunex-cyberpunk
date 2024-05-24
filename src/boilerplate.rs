use bevy::render::settings::WgpuSettings;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::{app::PluginGroupBuilder, prelude::*};
use vleue_kinetoscope::AnimatedGif;


// #=========================================#
// #=== BOILERPLATE REQUIRED FOR BEVYPUNK ===#

/// Marker struct for UI framework entities
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MenuUi;

/// Custom color palette for Bevypunk
pub trait BevypunkColorPalette {
    const BEVYPUNK_RED: Color;
    const BEVYPUNK_RED_DIM: Color;
    const BEVYPUNK_YELLOW: Color;
}
impl BevypunkColorPalette for Color {
    const BEVYPUNK_RED: Color = Color::rgba(255./255., 98./255., 81./255., 1.0);
    const BEVYPUNK_RED_DIM: Color = Color::rgba(172./255., 64./255., 63./255., 1.0);
    const BEVYPUNK_YELLOW: Color = Color::rgba(252./255., 226./255., 8./255., 1.0);
}

/// Color lerping functionality
pub trait LerpColor {
    fn lerp(&self, color: Color, value: f32) -> Color;
}
impl LerpColor for Color {
    fn lerp(&self, color: Color, value: f32) -> Color {
        let c1 = self.hsl_to_vec3();
        let c2 = color.hsl_to_vec3();
        Color::hsl(c1.x.lerp(c2.x, value), c1.y.lerp(c2.y, value), c1.z.lerp(c2.z, value))
    }
}

// #======================================#
// #=== ASSET CACHE FOR SMOOTH LOADING ===#

// Load all assets at startup for faster loading during runtime
#[derive(Resource)]
pub struct AssetCache {
    pub music: Handle<AudioSource>,

    pub font_light: Handle<Font>,
    pub font_regular: Handle<Font>,
    pub font_medium: Handle<Font>,
    pub font_semibold: Handle<Font>,
    pub font_bold: Handle<Font>,

    pub cursor: Handle<Image>,

    pub intro: Handle<AnimatedGif>,

    pub button: Handle<Image>,
    pub switch_base: Handle<Image>,
    pub switch_head: Handle<Image>,

    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,
    pub settings_background: Handle<Image>,
}
pub fn cache_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetCache {
        music: asset_server.load("sounds/main_menu.ogg"),
        font_light: asset_server.load("fonts/rajdhani/Rajdhani-Light.ttf"),
        font_regular: asset_server.load("fonts/rajdhani/Rajdhani-Regular.ttf"),
        font_medium: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_semibold: asset_server.load("fonts/rajdhani/Rajdhani-SemiBold.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),
        cursor: asset_server.load("images/cursor.png"),
        intro: asset_server.load("images/intro/intro-lossy.gif"),
        button: asset_server.load("images/main_menu/button.png"),
        switch_base: asset_server.load("images/settings/switch_base.png"),
        switch_head: asset_server.load("images/settings/switch_head.png"),
        main_background: asset_server.load("images/settings/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),
        settings_background: asset_server.load("images/settings/background.png"),
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
                WgpuSettings {
                    power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                    ..default()
                }
            ),
            ..default()
        }
    )
}

/// Function to return camera will all appropriate settings
pub fn camera() -> impl Bundle {
    (
        MenuUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera {
                hdr: true,
                ..default()
            },
            //tonemapping: Tonemapping::None,
            ..default()
        },
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

/// System for immitating flickering by randomly adjusting cameras bloom values
fn vfx_bloom_flicker(mut query: Query<&mut BloomSettings>) {
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