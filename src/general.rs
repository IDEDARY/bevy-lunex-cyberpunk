use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_lunex::prelude::*;
use super::style::*;


// ===========================================================
// === GENERAL WIDGET FUNCTIONALY ===
//# Defines general functionaly of widgets shared accross the app.

/// ### Live widget text
/// Add this component to an text entity with [`Widget`] component.
/// It will make the text dynamic.
/// 
/// Text will be synchronized with `widget_text` data value stored in the widget.
#[derive(Component)]
pub struct LiveWidgetText ();
pub fn live_widget_text_update (mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Text, &LiveWidgetText)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut text, _) in &mut query {
        let widget = widget.fetch_mut(&mut system, "").unwrap();
        match widget.data_get_mut() {
            Option::Some ( data ) => {
                match data.strings.get_mut("widget_text") {
                    Option::Some(txt) => {
                        text.sections[0].value = txt.to_string();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}


/// ### General widget plugin
/// Plugin adding general shared functionality of widgets.
pub struct GeneralWidgetPlugin;
impl Plugin for GeneralWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, live_widget_text_update);
    }
}


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

                    transform.translation.z = branch.get_depth() + element.depth;

                    let pos = widget.fetch(&mut system, "").unwrap().container_get().position_get().invert_y();
                    let vec = pos.get_pos_y_inverted(element.relative);
                    transform.translation.x = vec.x + ui.offset.x;
                    transform.translation.y = vec.y + ui.offset.y;

                    match element.width {
                        Option::Some (w) => {
                            match element.height {
                                Option::Some (h) => {
                                    transform.scale.x = (pos.width/element.boundary.x)*(w/100.0) * element.scale/100.0;
                                    transform.scale.y = (pos.height/element.boundary.y)*(h/100.0) * element.scale/100.0;
                                },
                                Option::None => {
                                    let scale = (pos.width/element.boundary.x)*(w/100.0) * element.scale/100.0;
                                    transform.scale.x = scale;
                                    transform.scale.y = scale;
                                },
                            }
                        },
                        Option::None => {
                            match element.height {
                                Option::Some (h) => {
                                    let scale = (pos.height/element.boundary.y)*(h/100.0) * element.scale/100.0;
                                    transform.scale.x = scale;
                                    transform.scale.y = scale;
                                },
                                Option::None => {
                                    let scale = f32::min(pos.width/element.boundary.x, pos.height/element.boundary.y) * element.scale/100.0;
                                    transform.scale.x = scale;
                                    transform.scale.y = scale;
                                },
                            }
                        },
                    }

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


// ===========================================================
// === SETUP PROFILER ===

#[derive(Component)]
pub struct Profiler {
    timer: f32,
    fps_recording: VecDeque<f32>,
}
impl Profiler {
    pub fn new () -> Profiler {
        Profiler {
            timer: 0.0,
            fps_recording: VecDeque::new(),
        }
    }
}
pub fn setup_profiler (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut Hierarchy) {
    let profiler = Widget::create(system, "profiler", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 30.0 },
        ..Default::default()
    }.pack()).unwrap();

    let widget = Widget::create(system, &profiler.end(""), Layout::Solid {
        width: 20.0,
        height: 10.0,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }.pack()).unwrap();

    let style = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 80.0,
        color: YELLOW_COLOR,
    };
    text_element_spawn!(commands, widget.clone(), &TextParams::centerleft().styled(&style).with_height(10.0).at(10.0, 30.0), "FPS: ",
        LiveWidgetText (),
        Profiler::new()
    );
}
pub fn profiler_update (mut systems: Query<&mut Hierarchy>, diagnostics: Res<bevy::diagnostic::DiagnosticsStore>, mut query: Query<(&mut Widget, &mut Profiler)> ) {
    let mut system = systems.single_mut();
    let (widget, mut profiler) = query.single_mut();

    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {

            if profiler.timer <= 0.0 {
                profiler.timer = 60.0;

                if profiler.fps_recording.len() >= 10 { profiler.fps_recording.pop_front(); }
                profiler.fps_recording.push_back(raw as f32);

                let mut sum = 0.0;
                for n in profiler.fps_recording.iter() {
                    sum += n;
                }

                let average: f32 = sum / profiler.fps_recording.len() as f32;

                widget.fetch_data_set_string(&mut system, "", "widget_text",  format!("FPS: {average:.0}")).unwrap();
            } else {
                profiler.timer -= 1.0;
            }
        }
    };
}
