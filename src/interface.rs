pub mod logic;
pub mod routes;
pub mod components;
pub use logic as lg;
pub use routes as rt;
pub use components as ui;


use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
pub struct InterfacePlugin<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default> Plugin for InterfacePlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(lg::LogicPlugin::<T>(PhantomData))
           .add_plugins(rt::RoutePlugin::<T>(PhantomData))
           .add_plugins(ui::ComponentPlugin::<T>(PhantomData));
    }
}