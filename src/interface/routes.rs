use bevy_lunex::prelude::*;

import_use!(menu);
import_use!(settings);

// Bundle all route logic to RoutePlugin
script_plugin!(RoutePlugin,
    add_plugins(MenuPlugin::<T>::new()),
    add_plugins(SettingsPlugin::<T>::new())
);