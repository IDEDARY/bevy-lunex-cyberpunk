pub mod intro;
pub use intro::*;

pub mod main_menu;
pub use main_menu::*;


// #====================#
// #=== ROUTE PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IntroRoutePlugin)
            .add_plugins(MainMenuRoutePlugin);
    }
}