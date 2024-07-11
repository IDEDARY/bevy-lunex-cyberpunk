use crate::{Plugin, App};
use avian3d::prelude::*;

pub mod controller;
pub use controller::*;

pub mod route;
pub use route::*;

pub mod input;
pub use input::*;


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputPlugin)
            .add_plugins(EntryPlugin)
            .add_plugins(ControllerPlugin)
            .add_plugins(PhysicsPlugins::default());
    }
}

