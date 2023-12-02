use crate::prelude::*;

/// # Switch
/// Spawned with following animation components:
/// * [`lg::AnimateControl`]
/// * [`lg::CursorHoverAsAnimateInput`]
/// * [`lg::AnimateColor`]
/// 
/// Spawned with following input components:
/// * [`lg::InputCursorHover`]
#[derive(Default)]
pub struct Switch {
    pub state: bool,
    pub base_color_off: Color,
    pub base_color_on: Color,
    pub base_head_color_off: Color,
    pub base_head_color_on: Color,
}
impl Switch {
    pub fn new(state: bool) -> Self {
        Switch {
            state,
            base_color_off: COLOR_PRIMARY.with_a(1.0),
            base_color_on: COLOR_SECONDARY.with_a(1.2).with_l(0.68),
            base_head_color_off: Color::ANTIQUE_WHITE,
            base_head_color_on: COLOR_PRIMARY.with_a(1.0),
        }
    }
    pub fn construct<T: Component + Default>(self, commands: &mut Commands, assets: &MenuAssetCache, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {

        let widget = WindowLayout::new().build_as(tree, path)?;

        let head = SolidLayout::new().with_horizontal_anchor(-1.0).build_as(tree, widget.end("Head"))?;
        let head_icon = WindowLayout::new().rel((10.0, 10.0)).size_rel((80.0, 80.0)).build_as(tree, head.end("Head_Icon"))?;

        commands.spawn((
            ImageElementBundle::new(&widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), assets.switch_base.clone(), Vec2::new(230.0, 80.0)),
            
            lg::Animate::new(),
            lg::AnimateControl::new(0.05, 0.05),
            lg::AnimateColor::new(self.base_color_off, self.base_color_on),
            lg::CursorHoverAsAnimateInput::new(),
            lg::PipeAnimateToTree("Head".into()),

            lg::InputCursorHover::new().request_cursor(1),
            lg::InputMouseClick::new(),
            bundle.clone()
        ));

        commands.spawn((
            head.clone(),
            lg::Animate::new(),
            lg::AnimateIntoSolidLayout::new(SolidLayout::new().with_horizontal_anchor(-1.0), SolidLayout::new().with_horizontal_anchor(1.0)),
            lg::PipeAnimateFromTree,
            lg::PipeAnimateToTree("Head_Icon".into()),
        ));

        commands.spawn((
            ImageElementBundle::new(&head_icon, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), assets.switch_head.clone(), Vec2::new(64.0, 64.0)),
            lg::Animate::new(),
            lg::AnimateColor::new(self.base_head_color_off, self.base_head_color_on),
            lg::PipeAnimateFromTree,
            bundle
        ));

        Ok(widget)
    }
}
script_plugin!(SwitchPlugin);