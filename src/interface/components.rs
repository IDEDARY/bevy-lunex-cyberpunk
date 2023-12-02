use bevy_lunex::prelude::*;

import_use!(button);
import_use!(main_menu_button);
import_use!(switch);

// Bundle all component logic to ComponentPlugin
script_plugin!(ComponentPlugin,
    add_plugins(ButtonPlugin::<T>::new()),
    add_plugins(MainMenuButtonPlugin::<T>::new()),
    add_plugins(SwitchPlugin::<T>::new())
);