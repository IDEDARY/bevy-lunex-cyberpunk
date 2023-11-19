use crate::prelude::*;

#[derive(Default)]
pub struct Button {
    pub display: String,
    pub object_color: Color,
    pub text_color: Color,
    pub object_hover_color: Color,
    pub text_hover_color: Color,
}
impl Button {
    pub fn new(text: impl Borrow<str>) -> Button {
        Button {
            display: text.borrow().into(),
            object_color: Color::rgba(255./255., 98./255., 81./255., 0.0),
            text_color: Color::rgba(255./255., 98./255., 81./255., 1.0),
            object_hover_color: Color::rgba(252./255., 226./255., 8./255., 1.2).with_l(0.68),
            text_hover_color: Color::rgba(252./255., 226./255., 8./255., 1.2).with_l(0.68),
        }
    }
}
impl UiComponent for Button {
    fn construct<T: Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {

        let text_style = TextStyle {
            font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
            font_size: 40.0,
            color: Color::rgba(255./255., 98./255., 81./255., 1.1),
        };

        let widget = WindowLayout::new().build_as(tree, path)?;

        commands.spawn((
            TextElementBundle::new(&widget, TextParams::centerleft().at(5.0, 50.0).with_scale(35.0).with_style(&text_style).with_height(Some(90.0)), &self.display),
            lg::Animate::new(),
            lg::AnimateColor::new(self.text_color, self.text_hover_color),
            bundle.clone()
        ));
        commands.spawn((
            ImageElementBundle::new(&widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/main_menu/button.png"), Vec2::new(532.0, 75.0)),
            lg::Animate::new(),
            lg::AnimateColor::new(self.object_color, self.object_hover_color),
            bundle
        ));

        Ok(widget)
    }
}

script_plugin!(ButtonPlugin);