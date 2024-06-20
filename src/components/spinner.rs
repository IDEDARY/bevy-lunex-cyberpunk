use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Spinner {
    pub name: String,
    pub index: usize,
    pub options: Vec<String>,
}

// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct SpinnerUi;

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
struct SpinnerControl {
    chevron_left: Entity,
    chevron_right: Entity
}

/// System that builds the component UI
fn build_component (mut commands: Commands, query: Query<(Entity, &Spinner), Added<Spinner>>, assets: Res<AssetCache>) {
    for (entity, spinner) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert(
            UiTreeBundle::<SpinnerUi>::from(UiTree::new("Spinner")),
        ).with_children(|ui| {

            // Spawn chevron left
            let chevron_left = ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Left/Chevron"),

                // Add layout
                UiLayout::window().pos(Rl((50.0, 50.0))).anchor(Anchor::Center).size((Rh(45.0), Rl(60.0))).pack::<Base>(),

                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // Give it a background image
                UiImage2dBundle::from(assets.chevron_left.clone()),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED.with_alpha(1.0)),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().receiver(true),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW),
            )).id();

            // Spawn chevron right
            let chevron_right = ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Right/Chevron"),

                // Add layout
                UiLayout::window().pos(Rl((50.0, 50.0))).anchor(Anchor::Center).size((Rh(45.0), Rl(60.0))).pack::<Base>(),
                
                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // Give it a background image
                UiImage2dBundle::from(assets.chevron_right.clone()),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED.with_alpha(1.0)),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().receiver(true),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW),
            )).id();

            // Spawn spinner text
            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Image/Text"),

                // Add layout
                UiLayout::window().pos(Rl((95., 50.))).anchor(Anchor::CenterRight).pack::<Base>(),

                // Add text
                UiText2dBundle {
                    text: Text::from_section(spinner.options[spinner.index].clone(),
                        TextStyle {
                            font: assets.font_medium.clone(),
                            font_size: 60.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },

                // Spinner control
                SpinnerControl { chevron_left, chevron_right }
            ));

            // Spawn spinner name
            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Image/Name"),

                // Add layout
                UiLayout::window().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack::<Base>(),

                // Add text
                UiText2dBundle {
                    text: Text::from_section(spinner.name.clone(),
                        TextStyle {
                            font: assets.font_semibold.clone(),
                            font_size: 60.0,
                            color: Color::BEVYPUNK_RED,
                        }),
                    ..default()
                },
            ));

            // Spawn spinner image
            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Image"),

                // Add layout
                UiLayout::window().size(Rl((100.0, 50.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_symetric_sliced.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_alpha(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
            ));

            // Spawn left
            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Left"),

                // Add layout
                UiLayout::window().pos(Rl((0.0, 55.0))).size((Rl(50.0) - Rh(2.5), Rl(45.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle::from(assets.button_sliced_bottom_left.clone()),

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                PickableBundle::default(),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0),

                // This will pipe this hover data to the specified entities
                UiAnimatorPipe::<Hover>::new(vec![chevron_left]),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED.with_alpha(0.15)),

                // This will change cursor icon on mouse hover
                OnHoverSetCursor::new(CursorIcon::Pointer),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW),

                // If we click on this, it will emmit UiClick event
                UiClickEmitter::new(chevron_left),
            ));

            // Spawn right
            ui.spawn((
                // Link this widget
                UiLink::<SpinnerUi>::path("Right"),

                // Add layout
                UiLayout::window().pos((Rl(50.0) + Rh(2.5), Rl(55.0))).size((Rl(50.0) - Rh(2.5), Rl(45.0))).pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_sliced_bottom_right.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_alpha(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                PickableBundle::default(),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0),

                // This will pipe this hover data to the specified entities
                UiAnimatorPipe::<Hover>::new(vec![chevron_right]),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED.with_alpha(0.15)),

                // This will change cursor icon on mouse hover
                OnHoverSetCursor::new(CursorIcon::Pointer),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW),

                // If we click on this, it will emmit UiClick event
                UiClickEmitter::new(chevron_right),
            ));

        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// System that will react to chevron presses
fn spinner_change_system(mut events: EventReader<UiClickEvent>, mut change: EventWriter<UiChangeEvent>, mut query: Query<(&mut Spinner, &Children, Entity)>, mut text: Query<(&SpinnerControl, &mut Text)>) {
    for event in events.read() {
        for (mut spinner, children, entity) in &mut query {
            for child in children {
                if let Ok((spinner_control, mut text)) = text.get_mut(*child) {
                    if spinner_control.chevron_left == event.target  {
                        if spinner.index == 0 { spinner.index = spinner.options.len() - 1 } else { spinner.index -= 1 }
                        text.sections[0].value = spinner.options[spinner.index].clone();
                        change.send(UiChangeEvent { target: entity, value: spinner.options[spinner.index].clone() });
                    }
                    if spinner_control.chevron_right == event.target {
                        if spinner.index + 1 == spinner.options.len() { spinner.index = 0 } else { spinner.index += 1 }
                        text.sections[0].value = spinner.options[spinner.index].clone();
                        change.send(UiChangeEvent { target: entity, value: spinner.options[spinner.index].clone() });
                    }
                }
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
            .add_plugins(UiGenericPlugin::<SpinnerUi>::new())

            // Add general systems
            .add_systems(Update, spinner_change_system.run_if(on_event::<UiClickEvent>()))
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}
