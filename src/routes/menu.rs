use bevy_lunex::prelude::*;
use bevy::prelude::*;

pub trait UiComponent {
    fn construct(tree: &mut UiTree) -> Result<(), LunexError>;
}

pub struct UiMenu;
impl UiComponent for UiMenu {
    fn construct(tree: &mut UiTree) -> Result<(), LunexError> {

        let menu = RelativeLayout::new().build(tree, "Menu")?;
        println!("{}", tree.tree());

        Ok(())
    }
}