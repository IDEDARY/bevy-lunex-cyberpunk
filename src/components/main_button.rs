use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

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
                UiLayout::window_full().pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.button_symetric_sliced.clone(),
                    sprite: Sprite { color: Color::BEVYPUNK_RED.with_a(0.15), ..default() },
                    ..default()
                },

                // Make the background scalable
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().receiver(true),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED.with_a(0.15)),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),

                // Hover layout
                UiLayout::window_full().x(Rl(10.0)).pack::<Hover>(),
                UiLayoutController::default(),
            )).id();

            // Spawn button text
            let text = ui.spawn((
                // Link this widget
                UiLink::<MainButtonUi>::path("Control/Image/Text"),

                // Here we can define where we want to position our text within the parent node,
                // don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
                UiLayout::window().pos(Rl((6., 50.))).anchor(Anchor::CenterLeft).pack::<Base>(),

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

                // Make it non-obsructable for hit checking (mouse detection)
                Pickable::IGNORE,

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().receiver(true),

                // This will set the color to red
                UiColor::<Base>::new(Color::BEVYPUNK_RED),

                // This will set hover color to yellow
                UiColor::<Hover>::new(Color::BEVYPUNK_YELLOW.with_l(0.68)),
            )).id();

            // Spawn button hover-zone
            ui.spawn((
                // Link this widget
                UiLink::<MainButtonUi>::path("Control"),

                // Add layout
                UiLayout::window_full().pack::<Base>(),

                // Make this spacial & clickable entity
                UiZoneBundle::default(),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0),

                // This will pipe this hover data to the specified entities
                UiAnimatorPipe::<Hover>::new(vec![text, image]),

                // This will change cursor icon on mouse hover
                OnHoverSetCursor::new(CursorIcon::Pointer),

                OnHoverPlaySound::new(assets.ui_ping.clone()),

                // If we click on this hover zone, it will emmit UiClick event from parent entity
                UiClickEmitter::new(entity),
            ));
            
        });
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
            .add_plugins(UiGenericPlugin::<MainButtonUi>::new())

            // Add general systems
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}
