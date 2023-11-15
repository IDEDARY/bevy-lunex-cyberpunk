pub mod animation;
pub use animation::*;
pub mod input;
pub use input::*;


use bevy::prelude::{Plugin, App, Update, IntoSystemConfigs, Component};
use std::marker::PhantomData;
#[derive(Debug, Clone, Default)]
pub struct LogicPlugin<T: Component + Default>(pub PhantomData<T>);
impl <T:Component + Default>LogicPlugin<T> {
    pub fn new() -> Self {
        LogicPlugin::<T>(PhantomData)
    }
}
impl <T: Component + Default> Plugin for LogicPlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_systems(Update, animate_system)
           .add_systems(Update, animate_window_position_system::<T>.after(animate_system))
           .add_systems(Update, animate_color_text_system.after(animate_system))
           .add_systems(Update, animate_color_image_system.after(animate_system))
           .add_systems(Update, input_mouse_hover_system::<T>)
           .add_systems(Update, input_mouse_click_system);
    }
}