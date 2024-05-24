use bevy::prelude::*;
use vleue_kinetoscope::*;
use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct IntroRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetCache>, query: Query<Entity, Added<IntroRoute>>) {
    for entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Spawn the master ui tree
        commands.entity(entity).insert((
            UiTreeBundle::<MenuUi>::from(UiTree::new("Intro")),
        )).with_children(|ui| {

            // Spawn the root div
            let root = UiLink::<MenuUi>::path("Root");  // Here we can define the name of the node
            ui.spawn((
                root.clone(),                               // Here we add the link
                UiLayout::window_full().pack(),             // This is where we define layout
            ));

            // Spawn the background
            ui.spawn((
                root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
                
                Element::default(),
                Dimension::default(),

                // Spawn the gif bundle
                AnimatedGifImageBundle {
                    animated_gif: assets.intro.clone(),
                    ..default()
                }
                
            ));

        });
    }
}


fn display_menu(
    mut commands: Commands,
    query: Query<Entity, With<Handle<AnimatedGif>>>,
    mut i: Local<f32>,
    delta: ResMut<Time>,
) {
    if *i > 11.0 {
        if !query.is_empty() {
            commands.entity(query.single()).despawn_recursive();
            commands.spawn((
                MainMenuRoute,
                MovableByCamera, // Marks this ui to receive Transform & Dimension updates from camera size
            ));
        }
    }
    *i += delta.delta_seconds();
}


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct IntroRoutePlugin;
impl Plugin for IntroRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AnimatedGifPlugin::default())

            .add_systems(Update, display_menu)
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

