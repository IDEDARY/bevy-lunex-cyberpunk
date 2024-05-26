pub mod animation;
pub use animation::*;


// #====================#
// #=== LOGIC PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AnimationPlugin);
    }
}