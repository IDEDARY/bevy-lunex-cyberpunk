pub mod menu;
pub use menu::*;


use bevy::prelude::{Plugin, App};
pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(Menu);
    }
}