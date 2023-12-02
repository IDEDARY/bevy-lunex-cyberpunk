use crate::prelude::*;

/// # Settings
/// Builder struct for 
#[derive(Default)]
pub struct Settings;
impl Settings {
    pub fn construct<T:Component + Default>(tree: &mut UiTree<T>, commands: &mut Commands, assets: &MenuAssetCache) -> Result<(), LunexError> {

        let settings = RelativeLayout::new().with_rel_1(Vec2::splat(-1.0)).with_rel_2(Vec2::splat(101.0)).build_as(tree, "Settings")?;
        
        let image = SolidLayout::new()
            .with_scaling(SolidScale::Fill)
            .with_size(1920.0, 1080.0)
            .build_in(tree, &settings)?;

        image.fetch_mut(tree)?.get_container_mut().set_render_depth(Modifier::Set(90.0));
        commands.spawn(ImageElementBundle::new(image, ImageParams::default().with_depth(-0.5), assets.settings_background.clone(), Vec2::new(1920.0, 1080.0)));
        
        let return_button = WindowLayout::empty().rel((5., 5.)).size_rel((10.0, 5.0)).build_as(tree, settings.end("Return"))?;

        ui::Button::new("Return").construct(commands, assets, tree, return_button.end("g"), ())?;

        Ok(())
    }
}


/// All of custom Main Menu logic
mod script {

}
use script::*;
script_plugin!(SettingsPlugin
);