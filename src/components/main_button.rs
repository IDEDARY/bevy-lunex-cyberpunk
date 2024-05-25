use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{AssetCache, BevypunkColorPalette, LerpColor};


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// Event that will trigger if we click the button.
#[derive(Event)]
pub struct MainButtonClick {
    pub target: Entity,
}

#[derive(Event)] struct MainButtonEnter { pub target: Entity }
impl From<ListenerInput<Pointer<Over>>> for MainButtonEnter {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self { MainButtonEnter { target: value.target() } }
}

#[derive(Event)] struct MainButtonLeave { pub target: Entity }
impl From<ListenerInput<Pointer<Out>>> for MainButtonLeave {
    fn from(value: ListenerInput<Pointer<Out>>) -> Self { MainButtonLeave { target: value.target() } }
}



/// When this component is added, a UI system is built
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
#[derive(Component, Debug, Clone, PartialEq)]
struct MainButtonControl {
    animation_direction: f32,    // -1.0 backwards, 1.0 forward
    animation_transition: f32,
    image_entity: Entity,
    text_entity: Entity,
}


/// System that builds the component UI
fn build_component (mut commands: Commands, query: Query<(Entity, &MainButton), Added<MainButton>>, assets: Res<AssetCache>) {
    for (entity, button_source) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<MainButtonUi>::from(UiTree::new("MainButton")),
        ).with_children(|ui| {

            // Spawn button image
            let image = ui.spawn((
                // Link this widget
                UiLink::<MainButtonUi>::path("Control/Image"),

                // Add layout
                UiLayout::window_full().pack(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.0), ..default() },
                    ..default()
                },

                Pickable::IGNORE,

                // Make the sprite tile
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
            )).id();

            // Spawn button text
            let text = ui.spawn((
                // Link this widget
                UiLink::<MainButtonUi>::path("Control/Image/Text"),

                // Here we can define where we want to position our text within the parent node,
                // don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
                UiLayout::window().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack(),

                Pickable::IGNORE,

                // Add text
                UiText2dBundle {
                    text: Text::from_section(&button_source.text,
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,    // Currently hardcoded as Relative height (Rh) - so 60% of the node height
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },
            )).id();

            // Spawn button control/hover-zone
            ui.spawn((
                // Link this widget
                UiLink::<MainButtonUi>::path("Control"),

                // Add layout
                UiLayout::window_full().pack(),

                // This is required to make this entity clickable
                PickableBundle::default(),
                On::<Pointer<Over>>::send_event::<MainButtonEnter>(),
                On::<Pointer<Out>>::send_event::<MainButtonLeave>(),
                UiSpatialBundle::default(),

                // This is our state machine
                MainButtonControl {
                    animation_direction: 0.0,
                    animation_transition: 0.0,
                    image_entity: image,
                    text_entity: text,
                },
            ));
        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that triggers when a pointer click a node
fn pointer_click_system(mut events: EventReader<Pointer<Down>>, mut write: EventWriter<MainButtonClick>, query: Query<&Parent, (With<MainButtonControl>, With<UiLink<MainButtonUi>>)>) {
    for event in events.read() {
        if let Ok(parent) = query.get(event.target) {
            write.send(MainButtonClick {
                target: **parent,
            });
        }
    }
}

/// System that triggers when a pointer enters a node
fn pointer_enter_system(mut events: EventReader<MainButtonEnter>, mut query: Query<&mut MainButtonControl, With<UiLink<MainButtonUi>>>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = 1.0;
        }
    }
}

/// System that triggers when a pointer leaves a node
fn pointer_leave_system(mut events: EventReader<MainButtonLeave>, mut query: Query<&mut MainButtonControl, With<UiLink<MainButtonUi>>>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = -1.0;
        }
    }
}

/// System that updates the state of the node over time
fn update_system(
    time: Res<Time>,
    mut set_color: EventWriter<SetColor>,
    mut set_layout: EventWriter<SetUiLayout>,
    mut query: Query<&mut MainButtonControl, With<UiLink<MainButtonUi>>>,
    mut cursor: Query<&mut Cursor2d>,
) {
    for mut control in &mut query {

        let previous = control.animation_transition;

        // Animate the transition
        control.animation_transition += time.delta_seconds() * 10.0 * control.animation_direction;
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);

        // If animation progress call instruction events
        if previous != control.animation_transition {

            // Set the color from transition
            let color = Color::BEVYPUNK_RED.lerp(Color::BEVYPUNK_YELLOW.with_l(0.68), control.animation_transition);
            set_color.send(SetColor {
                target: control.image_entity,
                color: color.with_a(control.animation_transition),
            });
            set_color.send(SetColor {
                target: control.text_entity,
                color,
            });

            // Set the layout from transition
            set_layout.send(SetUiLayout {
                target: control.image_entity,
                layout: UiLayout::window_full().x(Rl(10.0 * control.animation_transition)).pack(),
            });
        }

        // Request cursor
        if control.animation_direction == 1.0 {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(CursorIcon::Pointer, 1.0);
        }

    }
}


// #========================#
// #=== COMPONENT PLUGIN ===#

/// Plugin adding all our logic
pub struct MainButtonPlugin;
impl Plugin for MainButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiPlugin::<MainButtonUi>::new())
            //.add_plugins(UiDebugPlugin::<MainButtonUi>::new())

            // Add out event
            .add_event::<MainButtonClick>()
            .add_event::<MainButtonEnter>()
            .add_event::<MainButtonLeave>()

            // Add event systems
            .add_systems(Update, pointer_click_system.run_if(on_event::<Pointer<Down>>()))
            .add_systems(Update, pointer_enter_system.before(update_system).run_if(on_event::<MainButtonEnter>()))
            .add_systems(Update, pointer_leave_system.before(update_system).run_if(on_event::<MainButtonLeave>()))

            // Add general systems
            .add_systems(Update, update_system)
            .add_systems(Update, build_component);
    }
}
