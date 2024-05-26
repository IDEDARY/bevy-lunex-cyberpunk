use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{BevypunkColorPalette, LerpColor};


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// Simple button that triggers [`ButtonClick`], [`ButtonEnter`] and [`ButtonLeave`] events.
#[derive(Component, Debug, Default)]
pub struct Button {
    pub text: Text,
    pub texture: Option<Handle<Image>>,
    pub sprite: Option<Sprite>,
}

/// Event that is triggered if [`Button`] is clicked on.
#[derive(Event)] pub struct ButtonClick { pub target: Entity }
impl From<ListenerInput<Pointer<Click>>> for ButtonClick {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self { ButtonClick { target: value.target() } }
}

/// Event that is triggered if cursor enters [`Button`] boundary.
#[derive(Event)] pub struct ButtonEnter { pub target: Entity }
impl From<ListenerInput<Pointer<Over>>> for ButtonEnter {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self { ButtonEnter { target: value.target() } }
}

/// Event that is triggered if cursor leaves [`Button`] boundary.
#[derive(Event)] pub struct ButtonLeave { pub target: Entity }
impl From<ListenerInput<Pointer<Out>>> for ButtonLeave {
    fn from(value: ListenerInput<Pointer<Out>>) -> Self { ButtonLeave { target: value.target() } }
}


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// Marker struct for the sandboxed UI
#[derive(Component, Debug, Default, Clone, PartialEq)]
struct ButtonUi;




/// System that builds the component UI
fn build_component (mut commands: Commands, query: Query<(Entity, &Button), Added<Button>>) {
    for (entity, button) in &query {

        // This will create a private sandboxed UiTree within the entity just for the button
        commands.entity(entity).insert((
            PickableBundle::default(),
            On::<Pointer<Click>>::send_event::<ButtonClick>(),
            On::<Pointer<Over>>::send_event::<ButtonEnter>(),
            On::<Pointer<Out>>::send_event::<ButtonLeave>(),
            UiTreeBundle::<ButtonUi>::from(UiTree::new("Button")),
        )).with_children(|ui| {

            // Spawn background image
            if let Some(texture) = &button.texture {
                let mut image = ui.spawn((
                    UiLink::<ButtonUi>::path("Background"),
                    UiLayout::window_full().pack(),
                    UiImage2dBundle {
                        texture: texture.clone(),
                        ..default()
                    },
                ));

                if let Some(sprite) = &button.sprite {
                    image.insert(sprite.clone());
                }
            }


            // Spawn button text
            /* ui.spawn((
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
            )); */

        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#


// #========================#
// #=== COMPONENT PLUGIN ===#

/// Plugin adding all our logic
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonClick>()
            .add_event::<ButtonEnter>()
            .add_event::<ButtonLeave>()

            // Add general systems
            .add_systems(Update, build_component);
    }
}