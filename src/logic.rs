pub mod animation;
pub use animation::*;
pub mod input;
pub use input::*;


use bevy::prelude::{Plugin, App, Update};
pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_systems(Update, animate_window_position_system)
           .add_systems(Update, animate_window_system)
           .add_systems(Update, input_mouse_hover_system)
           .add_systems(Update, input_mouse_click_system);
    }
}