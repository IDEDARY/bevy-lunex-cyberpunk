pub(crate) use bevy::{prelude::*, sprite::Anchor};
pub(crate) use bevy_lunex::prelude::*;
//pub(crate) use vleue_kinetoscope::*;

mod boilerplate;
use boilerplate::*;

mod components;
use components::*;

mod routes;
use routes::*;


fn main() {
    // Our app
    let mut app = App::new();

    // Add plugins
    let app = app
        .insert_resource(bevy::asset::AssetMetaCheck::Never)
        .add_plugins((default_plugins(), UiPlugin))

        // General setup
        .add_plugins(VFXPlugin)
        .add_systems(PreStartup, cache_assets)
        .add_systems(Startup, setup)

        // Add our plugins
        .add_plugins(ComponentPlugin)
        .add_plugins(RoutePlugin);


    // Load gif before starting our app
    //let gif = AnimatedGifLoader::load_now("assets/images/intro/intro-lossy.gif".into(), app);

    // Insert the loaded handle and start our app
    app
    .insert_resource(PreLoader {
        //intro: gif
    })
    .run()
}


// #=====================#
// #=== GENERIC SETUP ===#

fn setup(mut commands: Commands, assets: Res<AssetCache>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>){
    // Spawn 2D camera
    commands.spawn(camera()).with_children(|camera| {

        // Spawn cursor
        camera.spawn ((

            // Here we can map different native cursor icons to texture atlas indexes and sprite offsets
            Cursor2d::new().native_cursor(false)
                .register_cursor(CursorIcon::Default, 0, (14.0, 14.0))
                .register_cursor(CursorIcon::Pointer, 1, (10.0, 12.0))
                .register_cursor(CursorIcon::Grab, 2, (40.0, 40.0)),

            // Add texture atlas to the cursor
            SpriteSheetBundle {
                texture: assets.cursor.clone(),
                atlas: TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(Vec2::splat(80.0), 3, 1, None, None)),
                    index: 0,
                },
                transform: Transform { scale: Vec3::new(0.45, 0.45, 1.0), ..default() },
                sprite: Sprite {
                    color: Color::BEVYPUNK_YELLOW.with_a(2.0).with_l(0.68),
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },

            // Make the raycaster ignore this entity, we don't want our cursor to block clicking
            Pickable::IGNORE,
        ));
    });

    // Spawn audio
    commands.spawn( AudioBundle { source: assets.music.clone(), settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.5)) } );

    // Spawn intro route
    commands.spawn(MainMenuRoute);
}
