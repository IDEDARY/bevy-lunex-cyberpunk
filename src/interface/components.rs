// MACRO add![button];

pub mod button;
pub use button::*;

use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
pub struct ComponentPlugin<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default> Plugin for ComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(ButtonPlugin::<T>(PhantomData));
    }
}