use crate::{Plugin, App};

pub mod controller;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
pub use controller::*;

pub mod entry;
pub use entry::*;

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
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(ControllerPlugin);
    }
}

