use bevy::prelude::*;
use bevy_lunex::prelude::*;  

pub const RED_COLOR: Color = Color::rgba(255./255., 98./255., 81./255., 1.1);
pub const RED_COLOR_DIM: Color = Color::rgba(204./255., 56./255., 51./255., 1.0);
pub const BLUE_COLOR: Color = Color::rgba(42./255., 237./255., 247./255., 1.3);
pub const PURPLE_COLOR: Color = Color::rgba(255./255., 34./255., 245./255., 1.3);
pub const YELLOW_COLOR: Color = Color::rgba(255./255., 245./255., 34./255., 1.3);
pub const GREY_COLOR: Color = Color::rgba(199./255., 186./255., 174./255., 1.0);


pub const GLOBAL_COLOR_STANDBY: Color = RED_COLOR;
pub const GLOBAL_COLOR_HOVER: Color = BLUE_COLOR;
pub const MAIN_MENU_COLOR_STANDBY: Color = GLOBAL_COLOR_STANDBY;
pub const SETTINGS_COLOR_CATEGORY: Color = GREY_COLOR;

// ===========================================================
// === LUNEX SYNC TO ENTITIES ===
//# This function is the main system that is behind aligning text and images. It querries through entities with widgets

#[derive(Component)]
pub struct UserInterface {
    pub offset: Vec2,
}

//OUTDATED, NEEDS TO RUN AFTER ELEMENT_UPDATE TO FIX ALL IMAGES DUE TO 0 FOR THER BOUNDARY
pub fn image_update(mut systems: Query<(&mut Hierarchy, &mut UserInterface)>, mut query: Query<(&mut Widget, &Handle<Image>, &mut Transform)>, assets: Res<Assets<Image>>) {

    let (mut system, mut ui) = systems.get_single_mut().unwrap();     //get the single hiearchy struct
    for (widget, image_handle, mut transform) in &mut query {
        match widget.fetch(&system, "") {
            Result::Err(..) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            },
            Result::Ok(branch) => {
                if !branch.is_visible() {
                    transform.translation.x = -10000.0;
                    transform.translation.y = -10000.0;
                } else {
                    ui.offset.x = -system.width/2.0;
                    ui.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch(&mut system, "").unwrap().container_get().position_get().invert_y();      //The widget will locate itself inside the hierarchy
                    transform.translation.x = pos.point_1.x + ui.offset.x;
                    transform.translation.y = pos.point_1.y + ui.offset.y;

                    match assets.get(image_handle) {
                        Option::Some(image) => {
                            let image_dimensions = image.size();
                            transform.scale.x = pos.width/image_dimensions.x;
                            transform.scale.y = pos.height/image_dimensions.y;
                        },
                        Option::None => {},
                    }
                }
            }
        };
    }
}

pub fn element_update(mut systems: Query<(&mut Hierarchy, &mut UserInterface)>, mut query: Query<(&mut Widget, &Element, &mut Transform)>) {

    let (mut system, mut ui) = systems.get_single_mut().unwrap();
    for (widget, element, mut transform) in &mut query {
        match widget.fetch(&system, "") {
            Result::Err(..) => {
                transform.translation.x = -10000.0;
                transform.translation.y = -10000.0;
            },
            Result::Ok(branch) => {
                if !branch.is_visible() {
                    transform.translation.x = -10000.0;
                    transform.translation.y = -10000.0;
                } else {
                    ui.offset.x = -system.width/2.0;
                    ui.offset.y = system.height/2.0;

                    transform.translation.z = branch.get_depth();

                    let pos = widget.fetch(&mut system, "").unwrap().container_get().position_get().invert_y();
                    let vec = pos.get_pos_y_inverted(element.relative);
                    transform.translation.x = vec.x + ui.offset.x;
                    transform.translation.y = vec.y + ui.offset.y;

                    let scale = f32::min(pos.width/element.boundary.x, pos.height/element.boundary.y) * element.scale/100.0;
                    transform.scale.x = scale;
                    transform.scale.y = scale;

                }
            }
        };
    }
}

pub struct AlignPlugin;
impl Plugin for AlignPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (element_update, image_update).chain());
    }
}