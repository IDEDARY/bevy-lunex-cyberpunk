use bevy::{core_pipeline::{bloom::BloomSettings, contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings, experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin}, Skybox}, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, sprite::SpriteSource};

use crate::*;
use avian3d::prelude::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct GameRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<Entity, Added<GameRoute>>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, flicker: Query<Entity, With<VFXBloomFlicker>>) {
    for route_entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Disable flickering
        if let Ok(entity) = flicker.get_single() {
            commands.entity(entity).remove::<VFXBloomFlicker>();
        }
        
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

            // Spawn scene
            /* route.spawn(SceneBundle {
                scene: asset_server.load("scenes/bedroom.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, -10.0),
                ..default()
            }); */

            // Spawn balls
            /* for y in 5..6 {
                for x in -2..2 {
                    for z in -2..2 {
                        route.spawn((
                            RigidBody::Dynamic,
                            Velocity::linear(Vec3::new(0.0, 1.0, 0.0)),
                            Collider::ball(0.3 + 0.1 * x as f32),
                            Restitution::coefficient(0.7),
                            PbrBundle {
                                mesh: meshes.add(Sphere::new(0.3 + 0.1 * x as f32).mesh().ico(5).unwrap()),
                                material: materials.add(StandardMaterial {
                                    //emissive: LinearRgba::rgb(3.0, 23.0, 9.0) * 0.1,
                                    emissive: Color::BEVYPUNK_YELLOW.into(),
                                    ..default()
                                }),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                ..default()
                            }
                        )).with_children(|obj| {
                            obj.spawn(PointLightBundle {
                                point_light: PointLight {
                                    intensity: 1.0,
                                    shadows_enabled: false,
                                    //color: (LinearRgba::rgb(3.0, 23.0, 9.0) * 0.1).into(),
                                    color: Color::BEVYPUNK_YELLOW.into(),
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    }
                }
            } */

            // Spawn player
            route.spawn((
                /* PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
                    material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0),
                    ..default()
                },
                CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
                    30.0,
                    0.92,
                    7.0,
                    (30.0 as avian3d::math::Scalar).to_radians(),
                ),
                Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                GravityScale(2.0), */
                // THIS is XY PLANE (Around Z) rotation and XYZ movement
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 3.0, 0.0),
                    ..default()
                },
                //ControllerInput::default(),
                PlayerPlaneRotation::default(),
                PlayerState::default(),
                //ControllerGravity::default(),

                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Collider::capsule(0.5, 0.25),
            )).with_children(|obj| {

                obj.spawn(SpatialBundle {
                    transform: Transform::from_xyz(0.0, -1.0, 0.0),
                    ..default()
                }).with_children(|obj|{

                    // Spawn POV
                    obj.spawn((
                        SpatialBundle { transform: Transform::from_xyz(0.0, 1.7, 0.0), ..default() },
                    )).with_children(|obj| {

                        let light = 50.0;

                        // Spawn camera
                        let mut cam = obj.spawn((
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
                            PlayerTiltRotation::default(),
                        ));

                        #[cfg(not(target_family = "wasm"))]
                        cam.insert((
                            TemporalAntiAliasBundle::default(),
                            ContrastAdaptiveSharpeningSettings::default(),
                        ));

                    });
                });

            
            });

            let mat = materials.add(Color::srgb_u8(50, 50, 50));

            // Spawn floor
            route.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
                    material: mat.clone(),
                    transform: Transform::from_xyz(0.0, -1.0, 0.0),
                    ..default()
                },
                Collider::cuboid(50.0, 2.0, 50.0),
                RigidBody::Static,
            ));
            route.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(50.0, 2.0, 2.0)),
                    material: mat.clone(),
                    transform: Transform::from_xyz(0.0, 1.0, -25.0),
                    ..default()
                },
                Collider::cuboid(50.0, 2.0, 2.0),
                RigidBody::Static,
            ));
            route.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(50.0, 2.0, 2.0)),
                    material: mat.clone(),
                    transform: Transform::from_xyz(0.0, 1.0, 25.0),
                    ..default()
                },
                Collider::cuboid(50.0, 2.0, 2.0),
                RigidBody::Static,
            ));
            route.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 2.0, 50.0)),
                    material: mat.clone(),
                    transform: Transform::from_xyz(-25.0, 1.0, 0.0),
                    ..default()
                },
                Collider::cuboid(2.0, 2.0, 50.0),
                RigidBody::Static,
            ));
            route.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 2.0, 50.0)),
                    material: mat.clone(),
                    transform: Transform::from_xyz(25.0, 1.0, 0.0),
                    ..default()
                },
                Collider::cuboid(2.0, 2.0, 50.0),
                RigidBody::Static,
            ));

            route.spawn((
                UiTreeBundle::<Ui3d> {
                    transform: Transform::from_xyz(0.0, 2.0, -2.0),
                    tree: UiTree::new("Worldspace"),
                    ..default()
                },
            )).with_children(|ui|{
                ui.spawn((
                    UiLink::<Ui3d>::path("Display"),

                    UiLayout::boundary().pos2((1920.0/1000.0, 1080.0/1000.0)).pack::<Base>(),
                    UiLayout::boundary().pos1((-100.0/1000.0, 0.0)).pos2((2020.0/1000.0, 1080.0/1000.0)).pack::<Hover>(),
                    UiLayoutController::default(),

                    PickableBundle::default(),
                    SpriteSource::default(),
                    UiAnimator::<Hover>::new().forward_speed(6.0).backward_speed(6.0),
                    OnHoverSetCursor::new(CursorIcon::Pointer),

                    //UiMaterial3dBundle::from_image(&mut material, asset_server.load("images/hud/hud.png")),
                    UiMaterial3dBundle::from_transparent_image(&mut materials, asset_server.load("images/hud/hud.png")),
                ));
            });

            route.spawn((
                UiTreeBundle::<Ui3d> {
                    transform: Transform::from_xyz(0.0, 2.0, -2.5),
                    tree: UiTree::new("Worldspace"),
                    ..default()
                },
            )).with_children(|ui|{
                ui.spawn((
                    UiLink::<Ui3d>::path("Display"),

                    UiLayout::boundary().pos2((1920.0/1000.0, 1080.0/1000.0)).pack::<Base>(),
                    UiLayout::boundary().pos1((-100.0/1000.0, 0.0)).pos2((2020.0/1000.0, 1080.0/1000.0)).pack::<Hover>(),
                    UiLayoutController::default(),

                    PickableBundle::default(),
                    SpriteSource::default(),
                    UiAnimator::<Hover>::new().forward_speed(6.0).backward_speed(6.0),
                    OnHoverSetCursor::new(CursorIcon::Pointer),

                    //UiMaterial3dBundle::from_image(&mut material, asset_server.load("images/hud/hud.png")),
                    UiMaterial3dBundle::from_transparent_image(&mut materials, asset_server.load("images/hud/hud.png")),
                ));
            });

            // Spawn the master ui tree        
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new("HUD")),
                MovableByCamera,
            )).with_children(|ui| {

                // Spawn 3D camera view
                ui.spawn((
                    UiLink::<MainUi>::path("Camera"),
                    UiLayout::window_full().pack::<Base>(), // Make this resizable
                    MovableByCamera,                        // This will resize the texture on Dimension change
                    UiImage2dBundle::from(render_image),
                    PickingPortal,
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
        #[cfg(not(target_family = "wasm"))]
        app.add_plugins(TemporalAntiAliasPlugin);

        app
            .add_systems(PreUpdate, build_route.before(UiSystems::Compute));
    }
}

