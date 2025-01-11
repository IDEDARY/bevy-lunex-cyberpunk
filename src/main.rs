use avian3d::parry::utils::hashmap::HashMap;
pub(crate) use bevy::prelude::*;
pub(crate) use vleue_kinetoscope::*;

use bevy_embedded_assets::*;


fn main() -> AppExit {
    // ____________________________________
    // ----- NEW APPLICATION INSTANCE -----
    let mut app = App::new();

    // Bundle all game assets into the binary
    app.add_plugins(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault});

    // Add game plugins
    app.add_plugins((DefaultPlugins, AnimatedImagePlugin));

    // ___________________________________
    // ----- PRIORITY ASSET LOADING  -----

    let mut priority_assets = PriorityAssets::default();

    // Load the game intro
    let intro = AnimatedImageLoader::load_now("assets/images/movies/intro.webp".into(), &mut app).expect("Priority load failed");
    priority_assets.video.insert("intro".to_string(), intro);

    // _________________________________
    // ----- START THE APPLICATION -----

    app.run()
}

#[derive(Resource, Default)]
pub struct PriorityAssets {
    video: HashMap<String, Handle<AnimatedImage>>,
}

