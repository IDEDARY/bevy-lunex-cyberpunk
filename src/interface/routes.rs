pub mod menu;
pub use menu::*;


use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
#[derive(Debug, Clone, Default)]
pub struct RoutePlugin<T:Component + Default>(pub PhantomData<T>);
impl <T:Component + Default>RoutePlugin<T> {
    pub fn new() -> Self {
        RoutePlugin::<T>(PhantomData)
    }
}
impl <T: Component + Default>Plugin for RoutePlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(MenuPlugin::<T>(PhantomData));
    }
}