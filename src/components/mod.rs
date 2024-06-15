pub mod button;
pub use button::*;

pub mod main_button;
pub use main_button::*;

pub mod spinner;
pub use spinner::*;

// #========================#
// #=== COMPONENT PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our component logic
pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ButtonPlugin)
            .add_plugins(MainButtonPlugin)
            .add_plugins(SpinnerPlugin);
    }
}