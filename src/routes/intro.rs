use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct IntroRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

#[derive(Component, Debug, Default, Clone, PartialEq)]
struct IntroGif;

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetCache>, _preloader: Res<PreLoader>, query: Query<Entity, Added<IntroRoute>>, mut event: EventWriter<actions::HideCursor2d>) {
    for route_entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        event.send(actions::HideCursor2d(true));

        // Spawn route
        commands.entity(route_entity).insert(
            SpatialBundle::default(),
        ).with_children(|route| {

            // Spawn the master ui tree
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new("Intro")),
                MovableByCamera,
            )).with_children(|ui| {

                // Spawn the root div
                let root = UiLink::<MainUi>::path("Root");  // Here we can define the name of the node
                ui.spawn((
                    root.clone(),                               // Here we add the link
                    UiLayout::window_full().pack::<Base>(),             // This is where we define layout
                ));

                // Spawn the background
                ui.spawn((
                    root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                    UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(assets.intro_background.clone()),  // We use this bundle to add background image to our node
                ));

                // Spawn the intro
                ui.spawn((
                    root.add("Intro"), // You can see here that we used existing "root" link to create chained link (same as "Root/Intro")
                    UiLayout::solid().size((1920.0, 1080.0)).pack::<Base>(),
                    UiDepthBias(1.0), // "background" and this node are on the same level, they will have same depth. Add this to avoid Z fighting.
                    
                    Element::default(),
                    Dimension::default(),

                    // Spawn the gif bundle
                    /* AnimatedGifImageBundle {
                        animated_gif: preloader.intro.clone(),
                        ..default()
                    }, */
                    IntroGif,
                ));

            });
        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/* /// Function that checks if our main intro has finished playing
fn despawn_intro_and_spawn_main_menu(
    mut commands: Commands,
    mut event: EventWriter<actions::HideCursor2d>,
    route: Query<Entity, With<IntroRoute>>,
    intro: Query<&AnimatedGifController, With<IntroGif>>,
) {
    for gif in &intro {
        if gif.current_frame() + 1 == gif.frame_count() {
            event.send(actions::HideCursor2d(false));
            commands.entity(route.single()).despawn_recursive();
            commands.spawn(MainMenuRoute);
        }
    }
} */


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct IntroRoutePlugin;
impl Plugin for IntroRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(AnimatedGifPlugin)

            //.add_systems(Update, despawn_intro_and_spawn_main_menu)
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

