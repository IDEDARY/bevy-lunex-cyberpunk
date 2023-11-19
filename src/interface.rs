use bevy_lunex::prelude::*;

pub mod logic;
pub mod routes;
pub mod components;
pub use logic as lg;
pub use routes as rt;
pub use components as ui;

// Bundle all interface logic to InterfacePlugin
script_plugin!(InterfacePlugin,
    add_plugins(lg::LogicPlugin::<T>::new()),
    add_plugins(rt::RoutePlugin::<T>::new()),
    add_plugins(ui::ComponentPlugin::<T>::new())
);