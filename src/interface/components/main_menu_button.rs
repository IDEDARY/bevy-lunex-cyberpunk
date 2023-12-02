use crate::prelude::*;

/// # Main Menu Button
/// Main menu button
#[derive(Default)]
pub struct MainMenuButton {
    pub display: String,
    pub object_color: Color,
    pub text_color: Color,
    pub object_hover_color: Color,
    pub text_hover_color: Color,
}
impl MainMenuButton {
    pub fn new(text: impl Borrow<str>) -> Self {
        MainMenuButton {
            display: text.borrow().into(),
            object_color: COLOR_PRIMARY.with_a(0.0),
            text_color: COLOR_PRIMARY.with_a(1.0),
            object_hover_color: COLOR_SECONDARY.with_a(1.2).with_l(0.68),
            text_hover_color: COLOR_SECONDARY.with_a(1.2).with_l(0.68),
        }
    }
    pub fn construct<T: Component + Default>(self, commands: &mut Commands, assets: &MenuAssetCache, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {

        let text_style = TextStyle {
            font: assets.font.clone(),
            font_size: 40.0,
            color: Color::default(),
        };

        let widget = WindowLayout::new().build_as(tree, path)?;

        commands.spawn((
            TextElementBundle::new(&widget, TextParams::centerleft().at(5.0, 50.0).with_scale(35.0).with_style(&text_style).with_height(Some(90.0)), &self.display),
            lg::Animate::new(),
            lg::AnimateControl::new(0.25, 0.05),
            lg::AnimateColor::new(self.text_color, self.text_hover_color),
            lg::PipeAnimateInputFromTree,
            bundle.clone()
        ));
        commands.spawn((
            ImageElementBundle::new(&widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), assets.button.clone(), Vec2::new(532.0, 75.0)),
            lg::Animate::new(),
            lg::AnimateControl::new(0.25, 0.05),
            lg::AnimateColor::new(self.object_color, self.object_hover_color),
            lg::PipeAnimateInputFromTree,
            bundle
        ));

        Ok(widget)
    }
}
script_plugin!(MainMenuButtonPlugin);