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
            base_color_off: Color::GRAY.with_a(0.2),
            base_color_on: COLOR_PRIMARY.with_a(1.0).with_l(0.68),
            base_head_color_off: COLOR_PRIMARY.with_a(1.0).with_s(0.2).with_l(0.2),
            base_head_color_on: COLOR_PRIMARY.with_a(1.0).with_l(0.68),
        }
    }
    pub fn construct<T: Component + Default>(self, commands: &mut Commands, assets: &MenuAssetCache, tree: &mut UiTree<T>, path: impl Borrow<str>, bundle: impl Bundle + Clone) -> Result<Widget, LunexError> {
        let widget = RelativeLayout::new().build_as(tree, path)?;
        let head = SolidLayout::new().with_horizontal_anchor(-1.0).build_as(tree, widget.end("Head"))?;
        let head_icon = RelativeLayout::new().with_rel_1((15.0, 15.0).into()).with_rel_2((85.0, 85.0).into()).build_as(tree, head.end("Head_Icon"))?;

        // Add hover logic (enlarge)
        commands.spawn((
            widget.clone(),
            lg::CursorHoverAsAnimateInput::new(),
            lg::InputCursorHover::new().request_cursor(1),
            
            lg::Animate::new(),
            lg::AnimateControl::new(0.1, 0.03).ease(1),
            lg::AnimateIntoRelativeLayout::new(RelativeLayout::new(), RelativeLayout::new().with_rel_1((-2.0, -2.0).into()).with_rel_2((102.0, 102.0).into())),
        ));

        // Add click logic (image + color change + pipe animation)
        commands.spawn((
            ImageElementBundle::new(&widget, ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), assets.switch_base.clone(), Vec2::new(160.0, 80.0)),
            SwitchState { state: self.state},
            lg::InputCursorHover::new(),
            lg::InputMouseClick::new(),

            lg::Animate::new(),
            lg::AnimateControl::new(0.02, 0.02).ease(4),
            lg::AnimateColor::new(self.base_color_off, self.base_color_on),
            lg::PipeAnimateToTree("Head".into()),
            bundle.clone()
        ));

        // Add move logic (from piped animation)
        commands.spawn((
            head.clone(),
            lg::Animate::new(),
            lg::AnimateIntoSolidLayout::new(SolidLayout::new().with_horizontal_anchor(-1.0), SolidLayout::new().with_horizontal_anchor(1.0)),
            lg::PipeAnimateFromTree,
            lg::PipeAnimateToTree("Head_Icon".into()),
        ));

        // Add color change logic (image + pipe animation)
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

/// All of custom switch logic
mod script {
    use crate::prelude::*;

    #[derive(Component, Clone, Copy)]
    pub struct SwitchState {
        pub state: bool,
    }
    /// What to do when the button is pressed
    pub(super) fn switch_update(mut query: Query<(&mut SwitchState, &mut lg::AnimateControl, &lg::InputMouseClick)>) {
        for (mut switch, mut control, clicked) in &mut query {
            if clicked.left {
                switch.state = !switch.state;
                control.trigger = switch.state;
            }
        }
    }
}
use script::*;
script_plugin!(SwitchPlugin,
    add_systems(Update, switch_update.after(lg::InputSystemSet).before(LunexUiSystemSet2D))
);