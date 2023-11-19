use bevy_lunex::prelude::*;

import_use!(menu);

// Bundle all route logic to RoutePlugin
script_plugin!(RoutePlugin,
    add_plugins(MenuPlugin::<T>::new())
);