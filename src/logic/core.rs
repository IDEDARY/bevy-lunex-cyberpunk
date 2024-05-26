use crate::*;


// #=================#
// #=== LISTENERS ===#

/// Event that gets triggered if we click on a button
#[derive(Event)]
pub struct UiClick {
    pub target: Entity,
}

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiClickEmitter {
    trigger: Entity,
}
impl UiClickEmitter {
    /// Creates new struct
    pub fn new(entity: Entity) -> Self {
        UiClickEmitter {
            trigger: entity
        }
    }
}

/// System that triggers when a pointer clicks a node and emmits an event
fn ui_click_listener_system(mut events: EventReader<Pointer<Down>>, mut write: EventWriter<UiClick>, query: Query<&UiClickEmitter>) {
    for event in events.read() {
        if let Ok(emitter) = query.get(event.target) {
            write.send(UiClick {
                target: emitter.trigger,
            });
        }
    }
}


// #====================#
// #=== HOVER PLUGIN ===#

/// Plugin adding all our logic
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add our event
            .add_event::<UiClick>()
            .add_systems(Update, ui_click_listener_system.run_if(on_event::<Pointer<Down>>()));
    }
}