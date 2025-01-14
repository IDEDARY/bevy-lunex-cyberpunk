pub(crate) use bevy::{prelude::*, sprite::Anchor};
pub(crate) use bevy_lunex::prelude::*;
pub(crate) use bevy_kira_audio::prelude::*;
#[cfg(not(target_family = "wasm"))]
pub(crate) use vleue_kinetoscope::*;

mod boilerplate;
use boilerplate::*;

mod components;
use components::*;

mod routes;
use routes::*;


fn main() {
    // Our app
    let mut app = App::new();

    #[cfg(not(target_family = "wasm"))]
    app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin { mode: bevy_embedded_assets::PluginMode::ReplaceDefault});

    // Add plugins
    let app = app
        .add_plugins((default_plugins(), UiDefaultPlugins))
        //.add_plugins(UiDebugPlugin::<MainUi>::new())

        // General setup
        .add_plugins(VFXPlugin)
        .add_systems(Startup, setup)

        // Add our plugins
        .add_plugins(ComponentPlugin)
        .add_plugins(RoutePlugin);


    #[cfg(not(target_family = "wasm"))]
    if let Ok(intro) = AnimatedImageLoader::load_now_from_bytes(include_bytes!("../assets/images/intro/intro.gif"), "gif", app){
        app.insert_resource(PreLoader { intro });
    }

    app.run();
}


// #=====================#
// #=== GENERIC SETUP ===#

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>, _audio: Res<Audio>){
    // Spawn 2D camera
    commands.spawn(camera()).with_children(|camera| {

        // Spawn cursor
        camera.spawn ((
            StyledCursorBundle {
                // Here we can map different native cursor icons to texture atlas indexes and sprite offsets
                cursor: Cursor2d::new()
                    .set_index(CursorIcon::Default, 0, (14.0, 14.0))
                    .set_index(CursorIcon::Pointer, 1, (10.0, 12.0))
                    .set_index(CursorIcon::Grab, 2, (40.0, 40.0)),
                // Add texture atlas to the cursor
                atlas: TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                },
                // Add sprite strip to the cursor
                sprite: SpriteBundle {
                    texture: assets.load(PreLoader::CURSOR),
                    transform: Transform { scale: Vec3::new(0.45, 0.45, 1.0), ..default() },
                    sprite: Sprite {
                        color: Color::BEVYPUNK_YELLOW.with_alpha(2.0),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },

            // Here we specify that the cursor should be controlled by gamepad 0
            //GamepadCursor::new(0),
        ));
    });

    #[cfg(not(target_family = "wasm"))]
    {
        // Spawn intro route
        commands.spawn(IntroRoute);
    }


    #[cfg(target_family = "wasm")]
    {
        // Skip intro on wasm
        commands.spawn(MainMenuRoute);

        // Play audio
        _audio.play(assets.load(PreLoader::MUSIC)).looped();
    }
}
