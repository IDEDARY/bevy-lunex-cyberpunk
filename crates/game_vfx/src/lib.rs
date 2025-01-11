use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use rand::Rng;

#[derive(Component)]
pub struct VFXBloomFlicker;
impl VFXBloomFlicker {
    /// System for immitating flickering by randomly adjusting camera's bloom values
    fn system(mut query: Query<&mut Bloom, With<VFXBloomFlicker>>) {
        for mut bloom in &mut query {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..100) < 20 {
                // This formula will make the value jumping smooth and natural, like neon flicker
                bloom.intensity += (rng.gen_range(0.20..0.30)-bloom.intensity)/6.0;
                bloom.prefilter.threshold += (rng.gen_range(0.20..0.30)-bloom.prefilter.threshold)/4.0;
            }
        }
    }
}

/// Plugin with VFX systems for our menu
pub struct VFXPlugin;
impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, VFXBloomFlicker::system);
    }
}