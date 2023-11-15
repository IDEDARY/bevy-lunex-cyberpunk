// MACRO add![button];

pub mod button;
pub use button::*;


use bevy::prelude::{Plugin, App, Component};
use std::marker::PhantomData;
#[derive(Debug, Clone, Default)]
pub struct ComponentPlugin<T:Component + Default>(pub PhantomData<T>);
impl <T:Component + Default>ComponentPlugin<T> {
    pub fn new() -> Self {
        ComponentPlugin::<T>(PhantomData)
    }
}
impl <T:Component + Default> Plugin for ComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_plugins(ButtonPlugin::<T>::new());
    }
}