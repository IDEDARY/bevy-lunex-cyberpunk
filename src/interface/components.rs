use bevy_lunex::prelude::*;

import_use!(button);

// Bundle all component logic to ComponentPlugin
script_plugin!(ComponentPlugin, add_plugins(ButtonPlugin::<T>::new()));