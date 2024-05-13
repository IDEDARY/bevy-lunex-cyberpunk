pub mod main_button;
pub use main_button::*;


// #========================#
// #=== COMPONENT PLUGIN ===#

use bevy::prelude::*;
pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainButtonPlugin);
    }
}