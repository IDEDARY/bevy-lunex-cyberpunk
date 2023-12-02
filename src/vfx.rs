use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_lunex::LunexUiDebugSystemSet2D;
use bevy::core_pipeline::bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
use rand::Rng;

/// # Camera
/// Function that returns a [`Camera2dBundle`] with specific settings
pub fn camera() -> impl Bundle {
    (
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
        },
        VfxWiggleCamera {
            sinusoid: vec![
                Sine {
                    speed: 0.001,
                    amplitude: 0.003,
                    degree: 0.0,
                }
            ]
        }
    )
}


/// # VFX Bloom Animate
/// System that generates random values in specific pattern to get nice bloom threshold flickering
fn vfx_bloom_animate(mut query: Query<&mut BloomSettings>) {
    for mut bloom in &mut query {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) < 20 {
            bloom.intensity += (rng.gen_range(0.20..0.25)-bloom.intensity)/5.0;
            bloom.prefilter_settings.threshold += (rng.gen_range(0.25..0.30)-bloom.prefilter_settings.threshold)/5.0;
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

/// # VFX Plugin
/// Plugin adding visual effects systems to our app
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (vfx_bloom_animate, vfx_camera_wiggle.after(LunexUiDebugSystemSet2D)));
    }
}