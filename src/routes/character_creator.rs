use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

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
    for route_entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Render 3D camera onto a texture
        let size = Extent3d { width: 1920, height: 1080, ..default() };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        image.resize(size);
        let render_image = asset_server.add(image);

        // Spawn the route
        commands.entity(route_entity).insert(
            SpatialBundle::default(),
        ).with_children(|route| {

            // Spawn 3D camera
            route.spawn(Camera3dBundle {
                camera: Camera {
                    order: -1,
                    target: render_image.clone().into(),
                    clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    hdr: true,
                    ..default()
                },
                ..default()
            });

            // Spawn 3D model in the scene
            route.spawn((
                SceneBundle {
                    scene: asset_server.load("models/female1.glb#Scene0"),
                    transform: Transform::from_xyz(-0.3, -1.5, -1.0),
                    ..default()
                },
                Showcase,
            ));
            
            // Spawn point light
            route.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 10000.0,
                    shadows_enabled: false,
                    color: Color::BEVYPUNK_RED.lerp(Color::WHITE, 0.6),
                    ..default()
                },
                ..default()
            });

            // Spawn the master ui tree
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new("CharacterCreator")),
                MovableByCamera,
            )).with_children(|ui| {
    
                // Spawn the root div
                let root = UiLink::<MainUi>::path("Root");  // Here we can define the name of the node
                ui.spawn((
                    root.clone(),                           // Here we add the link
                    UiLayout::window_full().pack::<Base>(), // This is where we define layout
                ));
    
                // Spawn the background
                ui.spawn((
                    root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                    UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(assets.settings_background.clone()),  // We use this bundle to add background image to our node
                    Pickable::IGNORE,
                ));
    
                // Spawn 3D camera view
                ui.spawn((
                    root.add("Background/Camera"),
                    UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(render_image),
                    Pickable::IGNORE,
                ));
                
                // Spawn return button
                ui.spawn((
                    root.add("Return"),
                    UiLayout::window().pos(Rl((2.0, 4.0))).size(Rl((16.0, 8.0))).pack::<Base>(),
                    Button { text: "<- Main menu".into() },

                    // If it detects UiClick event for this entity it will despawn route_entity and run a closure
                    OnUiClickDespawn::new(route_entity),
                    OnUiClickCommands::new(|commands| { commands.spawn((MainMenuRoute, MovableByCamera)); })
                ));
    
                // Spawn panel boundary
                let panel = root.add("Solid");
                ui.spawn((
                    panel.clone(),
                    UiLayout::solid().size((879.0, 1600.0)).align_x(0.74).pack::<Base>(), // Just different layout type that preserves aspect ratio
                ));
                
                // Spawn panel
                let panel = panel.add("Panel");
                ui.spawn((
                    panel.clone(),
                    UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack::<Base>(),
                    UiImage2dBundle::from(assets.character_creator_panel.clone())
                ));

                // Spawn text
               /*  ui.spawn((
                    panel.add("Text"),
                    UiLayout::window().pos(Rl((50., 5.))).anchor(Anchor::TopCenter).pack::<Base>(),
                    UiText2dBundle {
                        text: Text::from_section("Character creator",
                            TextStyle {
                                font: assets.font_medium.clone(),
                                font_size: 8.0,
                                color: Color::BEVYPUNK_RED,
                            }),
                        ..default()
                    },
                )); */
    
                // Spawn button boundary
                let list = panel.add("List");
                ui.spawn((
                    list.clone(),
                    UiLayout::window().pos(Rl((53.0, 15.0))).anchor(Anchor::TopCenter).size(Rl((60.0, 65.0))).pack::<Base>(),
                ));
    
                // Spawn buttons
                let gap = 5.0;
                let size = 14.0;
                let mut offset = 0.0;
                for array in [
                    ( "Gender", vec!["Female", "Male"]),
                    ( "Body", vec!["Body 1", "Body 2", "Body 3"]),
                    ( "Color", vec!["Red", "Blue"]),
                    ( "Hair", vec!["Short", "Bun", "Long", "Ponytail"]),
                    ( "Beard", vec!["None"]),
                ] {
                    let options: Vec<String> = array.1.iter().map(|&s| s.to_string()).collect();
    
                    ui.spawn((
                        list.add(array.0),
                        UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack::<Base>(),
                        Spinner { name: array.0.into(), index: 0, options },
                    ));
    
                    offset += gap + size;
                }
            });
        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

#[derive(Component)]
struct Showcase;
fn showcase_rotate_system(mut query: Query<&mut Transform, With<Showcase>>, mut local: Local<f32>, time: Res<Time>) {
    *local += time.delta_seconds();
    for mut transform in &mut query {
        *transform = transform.with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, (20.0 * local.sin()).to_radians(), 0.0));
    }
}
fn showcase_swap_system(mut events: EventReader<UiChangeEvent>, asset_server: Res<AssetServer>, mut query: Query<&mut Handle<Scene>, With<Showcase>>) {
    for event in events.read() {
        info!("{}", event.value);
        if event.value == "Male".to_string() {
            for mut mesh in &mut query {
                let new: Handle<Scene> = asset_server.load("models/male1.glb#Scene0");
                *mesh = new;
            }
        }
        if event.value == "Female".to_string() {
            for mut mesh in &mut query {
                let new: Handle<Scene> = asset_server.load("models/female1.glb#Scene0");
                *mesh = new;
            }
        }
    }
}


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct CharacterCreatorRoutePlugin;
impl Plugin for CharacterCreatorRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, showcase_rotate_system)
            .add_systems(Update, showcase_swap_system.run_if(on_event::<UiChangeEvent>()))

            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

