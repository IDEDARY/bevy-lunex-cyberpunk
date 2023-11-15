pub mod logic;
pub mod routes;
pub mod components;
pub use logic as lg;
pub use routes as rt;
pub use components as ui;


use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
#[derive(Debug, Clone, Default)]
pub struct InterfacePlugin<T:Component + Default>(pub PhantomData<T>);
impl <T:Component + Default>InterfacePlugin<T> {
    pub fn new() -> Self {
        InterfacePlugin::<T>(PhantomData)
    }
}
impl <T:Component + Default> Plugin for InterfacePlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(lg::LogicPlugin::<T>::new())
           .add_plugins(rt::RoutePlugin::<T>::new())
           .add_plugins(ui::ComponentPlugin::<T>::new());
    }
}