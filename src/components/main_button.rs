use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{AssetCache, BevypunkColorPalette};


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// Control component for our ui-component.
/// This works as an abstraction over the logic to make things more simple.
#[derive(Component, Clone, Default)]
pub struct MainButton {
    pub text: String,
    pub pressed: bool,
}


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct MainButtonUi;


/// System which builds the layout
fn build_system (mut commands: Commands, query: Query<(Entity, &MainButton), Added<MainButton>>, assets: Res<AssetCache>) {
    for (entity, button_source) in &query {

        info!("SPAWNED UI SYSTEM");

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<NoData, NoData, MainButtonUi>::from(UiTree::new("MainButton")),
        ).with_children(|ui| {

            let root = UiLink::path("Root");
            ui.spawn((
                MainButtonUi,
                root.clone(),
                UiLayout::Window::full().pack(),
                UiImage2dBundle {
                    texture: assets.button.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.0), ..default() },
                    ..default()
                },
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),    // Here we make the sprite tillable

                // This is required to make this entity clickable
                PickableBundle::default(),

                // Here we can define what event should be triggered on click
                On::<Pointer<Down>>::send_event::<MainButtonPressed>(),

                // Here we can define what happens on hover
                On::<Pointer<Over>>::target_component_mut::<Sprite>(|_, sprite| {
                    //sprite.color = Color::BEVYPUNK_YELLOW.with_l(0.68);
                    sprite.color.set_a(1.0);
                }),
                On::<Pointer<Out>>::target_component_mut::<Sprite>(|_, sprite| {
                    sprite.color.set_a(0.0);
                }),
            ));

            // Spawn button text
            ui.spawn((
                MainButtonUi,
                root.add("Text"),

                // Here we can define where we want to position our text within the parent node,
                // don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
                UiLayout::Window::new().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack(),

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

// Our event that will happen if we click on the button
#[derive(Event)]
struct MainButtonPressed {
    enitity: Entity,
}

// Implement constructor for our event
impl From<ListenerInput<Pointer<Down>>> for MainButtonPressed {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        MainButtonPressed {
            enitity: value.target(),
        }
    }
}

// System that will resolve our event
fn main_button_pressed_event_system(mut events: EventReader<MainButtonPressed>, mut query: Query<&mut MainButton>) {
    for event in events.read() {
        if let Ok(mut button) = query.get_mut(event.enitity) {
            button.pressed = true;
        }
    }
}

// System that will update our button
fn main_button_update_system(mut query: Query<&mut MainButton>) {
    for mut button in &mut query {
        button.pressed = false;
    }
}


// #==========================#
// #=== MAIN BUTTON PLUGIN ===#

pub struct MainButtonPlugin;
impl Plugin for MainButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiPlugin::<NoData, NoData, MainButtonUi>::new())

            // Add events
            .add_event::<MainButtonPressed>()
            .add_systems(Update, main_button_pressed_event_system.after(main_button_update_system).run_if(on_event::<MainButtonPressed>()))

            // Add general systems
            .add_systems(Update, build_system)
            .add_systems(Update, main_button_update_system);
    }
}
