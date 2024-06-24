use bevy::{core_pipeline::{bloom::BloomSettings, contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings, experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin}, Skybox}, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};

use crate::*;
use bevy_rapier3d::prelude::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct GameRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<Entity, Added<GameRoute>>) {
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

            route.spawn(SceneBundle {
                scene: asset_server.load("scenes/apartment_nomat.glb#Scene0"),
                //transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });


            // Spawn player
            route.spawn((
                // THIS is XY PLANE (Around Z) rotation and XYZ movement
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 3.0, 0.0),
                    ..default()
                },
                ControllerInput::default(),
                ControllerPlaneRotation::default(),
                ControllerState::default(),
                ControllerGravity::default(),

                RigidBody::KinematicPositionBased,
                Collider::capsule_y(0.5, 0.25),
                KinematicCharacterController {
                    translation: Some(Vec3::new(5.0, 0.0, 5.0)),
                    ..default()
                }

            )).with_children(|obj| {

                obj.spawn(SpatialBundle {
                    transform: Transform::from_xyz(0.0, -1.0, 0.0),
                    ..default()
                }).with_children(|obj|{

                    // Body
                    /* obj.spawn(SceneBundle {
                        scene: asset_server.load("objects/male/thin.gltf#Scene0"),
                        transform: Transform::from_xyz(0.0, 0.0, 0.4),
                        ..default()
                    }); */

                    // Spawn POV
                    obj.spawn((
                        SpatialBundle { transform: Transform::from_xyz(0.0, 1.7, 0.0), ..default() },
                    )).with_children(|obj| {

                        let light = 300.0;

                        // Spawn camera
                        obj.spawn((
                            TemporalAntiAliasBundle::default(),
                            ContrastAdaptiveSharpeningSettings::default(),
                            BloomSettings::NATURAL,
                            Skybox {
                                image: asset_server.load("scenes/skybox/skybox.ktx2"),
                                brightness: light,
                            },
                            EnvironmentMapLight {
                                diffuse_map: asset_server.load("scenes/skybox/diffuse_map.ktx2"),
                                specular_map: asset_server.load("scenes/skybox/specular_map.ktx2"),
                                intensity: light,
                            },
                            Camera3dBundle {
                                camera: Camera {
                                    order: -1,
                                    target: render_image.clone().into(),
                                    //clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                    hdr: true,
                                    ..default()
                                },
                                projection: Projection::Perspective(PerspectiveProjection {
                                    fov: 60.0_f32.to_radians(),
                                    ..default()
                                }),
                                ..default()
                            },
                            VisibilityBundle::default(),
                            ControllerTiltRotation::default(),
                        ));

                    });
                });

            
            });

            // Spawn floor
            route.spawn((
                Collider::cuboid(25.0, 1.0, 25.0),
                SpatialBundle::default(),
            ));

            // Spawn the master ui tree        
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new("HUD")),
                MovableByCamera,
            )).with_children(|ui| {

                // Spawn 3D camera view
                ui.spawn((
                    UiLink::<MainUi>::path("Camera"),
                    UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fit).pack::<Base>(),
                    UiImage2dBundle::from(render_image),
                    Pickable::IGNORE,
                ));

                ui.spawn((
                    UiLink::<MainUi>::path("Camera/HUD"),
                    UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fit).pack::<Base>(),
                    UiImage2dBundle::from(asset_server.load("images/hud/hud.png")),
                    Pickable::IGNORE,
                ));

                
            });
        });
    }
}


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct EntryPlugin;
impl Plugin for EntryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TemporalAntiAliasPlugin)
            .add_systems(PreUpdate, build_route.before(UiSystems::Compute));
    }
}

