use bevy_lunex::prelude::*;
use bevy::prelude::{Update, IntoSystemConfigs};

import_use!(animation, input);

// Bundle all utility logic to LogicPlugin
script_plugin!(LogicPlugin,

    add_systems(Update, 
        (
            animate_cursor_input.after(InputSystemSet),
            animate_system,
            animate_window_position_system::<T>,
            animate_into_window_layout_system::<T>,
            animate_into_relative_layout_system::<T>,
            animate_into_solid_layout_system::<T>,
            animate_color_text_system,
            animate_color_image_system
        ).chain().in_set(AnimateSystemSet)
    ),

    add_systems(Update, 
        (input_mouse_hover_system::<T>.before(bevy_lunex::cursor_update), input_mouse_click_system).chain().in_set(InputSystemSet)
    ),

    add_systems(Update,
        (
            pipe_cursor_hover_as_animate_input,
            pipe_animate_input_from_tree,
            pipe_animate_to_tree,
            pipe_animate_from_tree
        ).chain().after(InputSystemSet).before(AnimateSystemSet)
    )

);