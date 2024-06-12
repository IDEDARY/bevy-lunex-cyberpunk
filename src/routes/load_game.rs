use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct LoadGameRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetCache>, query: Query<Entity, Added<LoadGameRoute>>) {
    for entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Spawn the master ui tree
        commands.entity(entity).insert((
            UiTreeBundle::<MenuUi>::from(UiTree::new("LoadGame")),
        )).with_children(|ui| {

            // Spawn the root div
            let root = UiLink::<MenuUi>::path("Root");  // Here we can define the name of the node
            ui.spawn((
                root.clone(),                           // Here we add the link
                UiLayout::window_full().pack::<Base>(),         // This is where we define layout
            ));

            // Spawn the background
            ui.spawn((
                root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack::<Base>(),
                UiImage2dBundle::from(assets.settings_background.clone()),  // We use this bundle to add background image to our node
            ));

        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#



// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct LoadGameRoutePlugin;
impl Plugin for LoadGameRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

