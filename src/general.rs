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
pub fn live_widget_text_update (mut systems: Query<&mut UiTree>, mut query: Query<(&Widget, &mut Text, &LiveWidgetText)>) {
    let mut system = systems.get_single_mut().unwrap();
    for (widget, mut text, _) in &mut query {
        let widget = widget.fetch_mut(&mut system).unwrap();
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
pub fn setup_profiler (commands: &mut Commands, asset_server: &Res<AssetServer>, system: &mut UiTree) {
    let profiler = Widget::create(system, "profiler", RelativeLayout {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 30.0 },
        ..Default::default()
    }).unwrap();

    profiler.fetch_mut(system).unwrap().set_visibility(false);

    let widget = Widget::create(system, &profiler.end(""), SolidLayout {
        width: 20.0,
        height: 10.0,
        scaling: SolidScale::Fit,
        horizontal_anchor: -1.0,
        ..Default::default()
    }).unwrap();

    let style = TextStyle {
        font: asset_server.load(GLOBAL_ITEM_BUTTON_FONT),
        font_size: 80.0,
        color: YELLOW_COLOR,
    };
    commands.spawn((
        TextElementBundle::new(widget.clone(), &TextParams::centerleft().with_style(&style).with_height(Some(10.0)).at(10.0, 30.0), "FPS: "),
        LiveWidgetText (),
        Profiler::new(),
    ));
}
pub fn profiler_update (mut systems: Query<&mut UiTree>, diagnostics: Res<bevy::diagnostic::DiagnosticsStore>, mut query: Query<(&mut Widget, &mut Profiler)> ) {
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

                widget.fetch_data_set_string(&mut system, "widget_text",  format!("FPS: {average:.0}")).unwrap();
            } else {
                profiler.timer -= 1.0;
            }
        }
    };
}
