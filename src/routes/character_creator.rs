use bevy::{prelude::*, sprite::Anchor};
use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct CharacterCreatorRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetCache>, query: Query<Entity, Added<CharacterCreatorRoute>>, asset_server: Res<AssetServer>) {
    for entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Spawn the master ui tree
        commands.entity(entity).insert((
            UiTreeBundle::<MenuUi>::from(UiTree::new("CharacterCreator")),
        )).with_children(|ui| {

            // Spawn the root div
            let root = UiLink::<MenuUi>::path("Root");  // Here we can define the name of the node
            ui.spawn((
                root.clone(),                           // Here we add the link
                UiLayout::window_full().pack(),         // This is where we define layout
            ));

            // Spawn the background
            /* ui.spawn((
                root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack(),
                UiImage2dBundle::from(assets.settings_background.clone()),  // We use this bundle to add background image to our node
            )); */

            let board = root.add("Solid");
            ui.spawn((
                board.clone(),
                UiLayout::solid().size((879.0, 1600.0)).align_x(0.74).pack(), // Just different layout type that preserves aspect ratio
            ));

            let board = board.add("Board");
            ui.spawn((
                board.clone(),
                UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack(),
                UiImage2dBundle::from(assets.main_board.clone())
            ));

            // Spawn button boundary
            let list = board.add("List");
            ui.spawn((
                list.clone(),
                UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack(),
            ));

            // Spawn buttons
            let gap = 3.0;
            let size = 14.0;
            let mut offset = 0.0;
            for button in [
                "GENDER",
                "HAIR",
                "COLOR",
                "CLOTHES",
                "NAME",
            ] {

                ui.spawn((
                    list.add(button),
                    UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                    MainButton { text: button.into() },
                ));

                offset += gap + size;
            }


        });

        commands.spawn(SceneBundle {
            scene: asset_server.load("models/female4.glb#Scene0"),
            transform: Transform::from_xyz(-0.3, -1.5, -1.0),
            ..default()
        });

        commands.spawn(PointLightBundle {
			point_light: PointLight {
				intensity: 10000.0,
				shadows_enabled: false,
                color: Color::BEVYPUNK_RED.lerp(Color::WHITE, 0.5),
				//color: Color::rgb_linear(3000.0, 3000.0, 3000.0),
				..default()
			},
			..default()
		});
    }
}


// #=====================#
// #=== INTERACTIVITY ===#



// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct CharacterCreatorRoutePlugin;
impl Plugin for CharacterCreatorRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

