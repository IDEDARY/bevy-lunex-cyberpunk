use bevy_lunex::prelude::*;
use bevy::prelude::{Update, IntoSystemConfigs};

import_use!(animation, input);

// Bundle all utility logic to LogicPlugin
script_plugin!(LogicPlugin,
    add_systems(Update, animate_system),
    add_systems(Update, animate_window_position_system::<T>.after(animate_system)),
    add_systems(Update, animate_color_text_system.after(animate_system)),
    add_systems(Update, animate_color_image_system.after(animate_system)),
    add_systems(Update, input_mouse_hover_system::<T>),
    add_systems(Update, input_mouse_click_system)
);