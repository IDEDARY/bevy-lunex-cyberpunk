pub mod button;
pub use button::*;



use bevy::prelude::{Plugin, App};
pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(Button::default());
    }
}