use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{AssetCache, BevypunkColorPalette, LerpColor};


// #=========================#
// #=== EXPOSED COMPONENT ===#


/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Spinner {
    pub options: Vec<String>,
}


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct SpinnerUi;

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
struct ChevronControl {
    animation_direction: f32,    // -1.0 backwards, 1.0 forward
    animation_transition: f32,
}

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
struct SpinnerControl {
    index: usize,
    len: usize,
    options: Vec<String>,
    chevron_left: Entity,
    chevron_right: Entity
}


#[derive(Event)] struct ChevronClick { pub target: Entity }
impl From<ListenerInput<Pointer<Click>>> for ChevronClick {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self { ChevronClick { target: value.target() } }
}

#[derive(Event)] struct ChevronEnter { pub target: Entity }
impl From<ListenerInput<Pointer<Over>>> for ChevronEnter {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self { ChevronEnter { target: value.target() } }
}

#[derive(Event)] struct ChevronLeave { pub target: Entity }
impl From<ListenerInput<Pointer<Out>>> for ChevronLeave {
    fn from(value: ListenerInput<Pointer<Out>>) -> Self { ChevronLeave { target: value.target() } }
}

/// System that builds the component UI
fn build_component (mut commands: Commands, query: Query<(Entity, &Spinner), Added<Spinner>>, assets: Res<AssetCache>) {
    for (entity, spinner) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<SpinnerUi>::from(UiTree::new("Spinner")),
        ).with_children(|ui| {

            // Spawn chevron left
            let left = ui.spawn((
                UiLink::<SpinnerUi>::path("ChevronLeft"),
                UiLayout::window().size((Rh(60.0), Rl(100.0))).pack(),

                PickableBundle::default(),
                On::<Pointer<Click>>::send_event::<ChevronClick>(),
                On::<Pointer<Over>>::send_event::<ChevronEnter>(),
                On::<Pointer<Out>>::send_event::<ChevronLeave>(),

                UiImage2dBundle {
                    texture: assets.chevron_left.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                    ..default()
                },

                ChevronControl {
                    animation_direction: -1.0,
                    animation_transition: 0.0,
                },
            )).id();

            // Spawn chevron right
            let right = ui.spawn((
                UiLink::<SpinnerUi>::path("ChevronRight"),
                UiLayout::window().x(Rl(100.0) - Rh(60.0)).size((Rh(60.0), Rl(100.0))).pack(),
                
                PickableBundle::default(),
                On::<Pointer<Click>>::send_event::<ChevronClick>(),
                On::<Pointer<Over>>::send_event::<ChevronEnter>(),
                On::<Pointer<Out>>::send_event::<ChevronLeave>(),

                UiImage2dBundle {
                    texture: assets.chevron_right.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED, ..default() },
                    ..default()
                },

                ChevronControl {
                    animation_direction: -1.0,
                    animation_transition: 0.0,
                },
            )).id();

            // Spawn button text
            ui.spawn((
                UiLink::<SpinnerUi>::path("Text"),
                UiLayout::window().pos(Rl((50., 50.))).anchor(Anchor::Center).pack(),
                Pickable::IGNORE,
                UiText2dBundle {
                    text: Text::from_section(spinner.options[0].clone(),
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,    // Currently hardcoded as Relative height (Rh) - so 60% of the node height
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },
                SpinnerControl {
                    index: 0,
                    len: spinner.options.len(),
                    options: spinner.options.clone(),
                    chevron_left: left,
                    chevron_right: right,
                }
            ));

        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that triggers when a pointer enters a node
fn chevron_pointer_enter_system(mut events: EventReader<ChevronEnter>, mut query: Query<&mut ChevronControl>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = 1.0;
        }
    }
}

/// System that triggers when a pointer leaves a node
fn chevron_pointer_leave_system(mut events: EventReader<ChevronLeave>, mut query: Query<&mut ChevronControl>) {
    for event in events.read() {
        if let Ok(mut control) = query.get_mut(event.target) {
            control.animation_direction = -1.0;
        }
    }
}

/// System that updates the state of the node over time
fn chevron_update_system(
    time: Res<Time>,
    mut set_color: EventWriter<SetColor>,
    mut query: Query<(&mut ChevronControl, Entity)>,
    mut cursor: Query<&mut Cursor2d>,
) {
    for (mut control, entity) in &mut query {

        let previous = control.animation_transition;

        // Animate the transition
        control.animation_transition += time.delta_seconds() * 10.0 * control.animation_direction * if control.animation_direction == 1.0 { 1.0 } else { 0.2 };
        control.animation_transition = control.animation_transition.clamp(0.0, 1.0);

        // If animation progress call instruction events
        if previous != control.animation_transition {

            // Set the color from transition
            let color = Color::BEVYPUNK_RED.lerp(Color::BEVYPUNK_YELLOW.with_l(0.68), control.animation_transition);
            set_color.send(SetColor {
                target: entity,
                color,
            });
        }

        // Request cursor
        if control.animation_direction == 1.0 {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(CursorIcon::Pointer, 1.0);
        }
    }
}


fn spinner_update_system(mut query: Query<(&SpinnerControl, &mut Text), Changed<SpinnerControl>>) {
    for (spinner, mut text) in &mut query {
        text.sections[0].value = spinner.options[spinner.index].clone();
    }
}

fn spinner_change_system(mut events: EventReader<ChevronClick>, mut query: Query<&mut SpinnerControl>) {
    for event in events.read() {
        for mut spinner in &mut query {
            if spinner.chevron_left == event.target  {
                if spinner.index == 0 { spinner.index = spinner.len - 1 } else { spinner.index -= 1 }
            }
            if spinner.chevron_right == event.target {
                if spinner.index + 1 == spinner.len { spinner.index = 0 } else { spinner.index += 1 }
            }
        }
    }
}

// #========================#
// #=== COMPONENT PLUGIN ===#

/// Plugin adding all our logic
pub struct SpinnerPlugin;
impl Plugin for SpinnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiPlugin::<SpinnerUi>::new())
            //.add_plugins(UiDebugPlugin::<SpinnerUi>::new())

            .add_event::<ChevronClick>()
            .add_event::<ChevronEnter>()
            .add_event::<ChevronLeave>()
            .add_systems(Update, spinner_change_system.run_if(on_event::<ChevronClick>()))
            .add_systems(Update, chevron_pointer_enter_system.before(chevron_update_system).run_if(on_event::<ChevronEnter>()))
            .add_systems(Update, chevron_pointer_leave_system.before(chevron_update_system).run_if(on_event::<ChevronLeave>()))
            .add_systems(Update, chevron_update_system)

            // Add general systems
            .add_systems(Update, spinner_update_system)
            .add_systems(Update, build_component);
    }
}
