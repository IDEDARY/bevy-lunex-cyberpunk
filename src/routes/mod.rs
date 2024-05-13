pub mod main_menu;
pub use main_menu::*;


// #====================#
// #=== ROUTE PLUGIN ===#

use bevy::prelude::*;
pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainMenuRoutePlugin);
    }
}