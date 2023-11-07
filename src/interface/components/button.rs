// MACRO import!()
use std::borrow::Borrow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;

use crate::MyData;
use crate::UiComponent;
use crate::logic as lg;


#[derive(Default)]
pub struct Button {
    display: String,
}
impl Button {
    pub fn new(text: impl Borrow<str>) -> Button {
        Button {
            display: text.borrow().into()
        }
    }
}
impl UiComponent for Button {
    fn construct<T: Component + Default>(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree<T>, path: impl Borrow<str>) -> Result<Widget, LunexError> {

        let text_style = TextStyle {
            font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
            font_size: 40.0,
            color: Color::rgba(255./255., 98./255., 81./255., 1.1),
        };

        let widget = WindowLayout::new().build(tree, path)?;

        commands.spawn((
            TextElementBundle::new(&widget, TextParams::centerleft().at(5.0, 50.0).with_scale(35.0).with_style(&text_style).with_height(Some(90.0)), &self.display),
            lg::Animate::new(),
            lg::AnimateColor::new(Color::rgba(255./255., 98./255., 81./255., 1.0), Color::rgba(252./255., 226./255., 8./255., 1.2).with_l(0.65)),
            InputActive
        ));
        commands.spawn((
            ImageElementBundle::new(&widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/main_menu/button.png"), Vec2::new(532.0, 75.0)),
            lg::Animate::new(),
            lg::AnimateColor::new(Color::rgba(255./255., 98./255., 81./255., 1.0), Color::rgba(252./255., 226./255., 8./255., 1.2).with_l(0.65)),
            InputActive
        ));

        Ok(widget)
    }
}



// MACRO logic![]
pub (super) struct ButtonPlugin<T:Component + Default>(pub std::marker::PhantomData<T>);
impl <T:Component + Default> Plugin for ButtonPlugin<T> {
    fn build(&self, app: &mut App) {
        #![allow(path_statements)]
        app.add_systems(Update, input_active_system);
    }
}


#[derive(Component)]
struct InputActive;
fn input_active_system(mut trees: Query<&UiTree<MyData>>, mut query: Query<(&mut lg::Animate, &Widget), With<InputActive>>) {
    for mut tree in &mut trees {
        for (mut destination, source) in &mut query {
            let data: &MyData = source.fetch_mut(&mut tree).unwrap().get_data();
            destination.trigger = data.animate;
        }
    }
}