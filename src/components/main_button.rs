use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{AssetCache, BevypunkColorPalette, LerpColor};


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// Control component for our ui-component.
/// This works as an abstraction over the logic to make things more simple.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainButton {
    pub text: String,
}


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct MainButtonUi;

/// Control struct for the button state
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct MainButtonControl {
    animation_direction: f32,    // -1.0 backwards, 1.0 forward
    animation_transition: f32,
}


/// System which builds the layout
fn build_system (mut commands: Commands, query: Query<(Entity, &MainButton), Added<MainButton>>, assets: Res<AssetCache>) {
    for (entity, button_source) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<NoData, NoData, MainButtonUi>::from(UiTree::new("MainButton")),
        ).with_children(|ui| {

            ui.spawn((
                MainButtonUi,
                MainButtonControl::default(),
                UiLink::<MainButtonUi>::path("Root"),
                UiLayout::window_full().pack(),
                UiImage2dBundle {
                    texture: assets.button.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.0), ..default() },
                    ..default()
                },
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),    // Here we make the sprite tillable

                // This is required to make this entity clickable
                PickableBundle::default(),
            ));

            // Spawn button text
            ui.spawn((
                MainButtonUi,
                UiLink::<MainButtonUi>::path("Root/Text"),

                // Here we can define where we want to position our text within the parent node,
                // don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
                UiLayout::window().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack(),

                // Here we define the text and style
                UiText2dBundle {
                    text: Text::from_section(&button_source.text,
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },
            ));
        });
    }
}


// #=================================#
// #=== MAIN BUTTON INTERACTIVITY ===#

/// System that triggers when a pointer enters a node
fn pointer_enter_system(mut events: EventReader<Pointer<Over>>, mut query: Query<&mut MainButtonControl, With<MainButtonUi>>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = 1.0;
        }
    }
}

/// System that triggers when a pointer leaves a node
fn pointer_leave_system(mut events: EventReader<Pointer<Out>>, mut query: Query<&mut MainButtonControl, With<MainButtonUi>>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = -1.0;
        }
    }
}

/// System that updates the state of the node over time
fn update_system(time: Res<Time>, mut query: Query<(&mut MainButtonControl, &mut Sprite, &mut UiLayout), With<MainButtonUi>>, mut cursor: Query<&mut Cursor2d>) {
    for (mut control, mut sprite, mut layout) in &mut query {

        // Animate the transition
        control.animation_transition += time.delta_seconds() * 10.0 * control.animation_direction;
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);

        // Get the color from transition
        sprite.color = Color::BEVYPUNK_RED.lerp(Color::BEVYPUNK_YELLOW.with_l(0.68), control.animation_transition);
        sprite.color.set_a(control.animation_transition);

        // Get the position from transition
        let window = layout.expect_window_mut();
        window.set_x(Rl(10.0 * control.animation_transition));

        // Request cursor
        if control.animation_direction == 1.0 {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(CursorIcon::Copy, 1.0);
        }

    }
}

fn sys(ui: Query<(&UiTree<NoData, NoData>, &Children), With<MainButtonUi>>, query: Query<(&mut Text, Entity), With<MainButtonUi>> ) {

}

// #==========================#
// #=== MAIN BUTTON PLUGIN ===#

pub struct MainButtonPlugin;
impl Plugin for MainButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiPlugin::<NoData, NoData, MainButtonUi>::new())

            // Add event systems
            .add_systems(Update, pointer_enter_system.before(update_system).run_if(on_event::<Pointer<Over>>()))
            .add_systems(Update, pointer_leave_system.before(update_system).run_if(on_event::<Pointer<Out>>()))

            // Add general systems
            .add_systems(Update, update_system)
            .add_systems(Update, build_system);
    }
}
