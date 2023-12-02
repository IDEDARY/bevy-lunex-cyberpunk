use crate::prelude::*;

/// # Button
/// simple button
#[derive(Default)]
pub struct Button {
    pub display: String,
    pub text_color: Color,
    pub text_hover_color: Color,
}
impl Button {
    pub fn new(text: impl Borrow<str>) -> Button {
        Button {
            display: text.borrow().into(),
            text_color: COLOR_PRIMARY.with_a(1.0),
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
            TextElementBundle::new(&widget, TextParams::center().with_scale(55.0).with_style(&text_style).with_height(Some(90.0)), &self.display),
            lg::Animate::new(),
            lg::AnimateColor::new(self.text_color, self.text_hover_color),
            bundle.clone()
        ));

        Ok(widget)
    }
}
script_plugin!(ButtonPlugin);