use bevy::core_pipeline::bloom::BloomSettings;
use bevy::app::PluginGroupBuilder;
use crate::*;

#[derive(Component, Default)]
pub struct Ui3d;

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
    const BEVYPUNK_RED: Color = Color::srgba(1., 98./255., 81./255., 1.0);
    const BEVYPUNK_RED_DIM: Color = Color::srgba(172./255., 64./255., 63./255., 1.0);
    const BEVYPUNK_YELLOW: Color = Color::linear_rgba(252./255., 226./255., 8./255., 1.0);
    const BEVYPUNK_BLUE: Color = Color::srgba(8./255., 226./255., 252./255., 1.0);
}

// #======================================#
// #=== ASSET CACHE FOR SMOOTH LOADING ===#

// Load the gif before the app is run
#[derive(Resource)]
pub struct PreLoader {
    #[cfg(not(target_family = "wasm"))]
    pub intro: Handle<AnimatedImage>,
}

#[allow(dead_code)]
impl PreLoader {
    // Music
    pub const MUSIC: &'static str = "sounds/main_menu.ogg";
    pub const MUSIC_INTRO: &'static str = "sounds/intro.ogg";
    pub const SFX_UI: &'static str = "sounds/ui_ping.ogg";

    // Fonts
    pub const FONT_LIGHT: &'static str = "fonts/rajdhani/Rajdhani-Light.ttf";
    pub const FONT_REGULAR: &'static str = "fonts/rajdhani/Rajdhani-Regular.ttf";
    pub const FONT_MEDIUM: &'static str = "fonts/rajdhani/Rajdhani-Medium.ttf";
    pub const FONT_SEMIBOLD: &'static str = "fonts/rajdhani/Rajdhani-SemiBold.ttf";
    pub const FONT_BOLD: &'static str = "fonts/rajdhani/Rajdhani-Bold.ttf";

    // Cursor
    pub const CURSOR: &'static str = "images/cursor.png";

    // Symbols
    pub const BUTTON_SYMETRIC: &'static str = "images/button_symetric.png";
    pub const BUTTON_SYMETRIC_SLICED: &'static str = "images/button_symetric_sliced.png";
    pub const BUTTON_SLICED_BOTTOM_LEFT: &'static str = "images/button_sliced_bottom_left.png";
    pub const BUTTON_SLICED_BOTTOM_RIGHT: &'static str = "images/button_sliced_bottom_right.png";
    pub const BUTTON_SLICED_TOP_LEFT: &'static str = "images/button_sliced_top_left.png";
    pub const BUTTON_SLICED_TOP_RIGHT: &'static str = "images/button_sliced_top_right.png";
    pub const CHEVRON_LEFT: &'static str = "images/chevron_left.png";
    pub const CHEVRON_RIGHT: &'static str = "images/chevron_right.png";
    pub const SWITCH_BASE: &'static str = "images/switch_base.png";
    pub const SWITCH_HEAD: &'static str = "images/switch_head.png";

    // Routes
    pub const INTRO_BACKGROUND: &'static str = "images/intro/frame_blank.png";

    pub const MAIN_BACKGROUND: &'static str = "images/settings/background.png";
    pub const MAIN_BOARD: &'static str = "images/main_menu/board.png";
    pub const MAIN_LOGO: &'static str = "images/main_menu/bevypunk.png";

    pub const SETTINGS_BACKGROUND: &'static str = "images/settings/background.png";

    pub const CHARACTER_CREATOR_PANEL: &'static str = "images/character_creator/panel.png";
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
    .add_group(UiGenericPlugins::<Ui3d>::new())
    //.add(UiDebugPlugin::<Ui3d>::new())
    .add(AudioPlugin)
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