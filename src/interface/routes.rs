pub mod menu;
pub use menu::*;


use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
pub struct RoutePlugin<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T: Component + Default>Plugin for RoutePlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(MenuPlugin::<T>(PhantomData));
    }
}