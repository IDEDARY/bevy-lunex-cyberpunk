use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainMenuRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetServer>, query: Query<Entity, Added<MainMenuRoute>>) {
    for route_entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Spawn the route
        commands.entity(route_entity).insert(
            SpatialBundle::default(),
        ).with_children(|route| {


            // Spawn the master ui tree        
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                MovableByCamera,
            )).with_children(|ui| {

                // Spawn the root div
                let root = UiLink::<MainUi>::path("Root");  // Here we can define the name of the node
                ui.spawn((
                    root.clone(),                           // Here we add the link
                    UiLayout::window_full().pack::<Base>(),         // This is where we define layout
                ));

                // Spawn the background
                ui.spawn((
                    root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                    UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(assets.load(PreLoader::MAIN_BACKGROUND)),  // We use this bundle to add background image to our node
                ));


                // Spawn the board
                let board = root.add("Solid");
                ui.spawn((
                    board.clone(),
                    UiLayout::solid().size((881.0, 1600.0)).align_x(-0.74).pack::<Base>(), // Just different layout type that preserves aspect ratio
                ));

                let board = board.add("Board");
                ui.spawn((
                    board.clone(),
                    UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack::<Base>(),
                    UiImage2dBundle::from(assets.load(PreLoader::MAIN_BOARD)),
                ));


                // Spawn the logo
                ui.spawn((
                    board.add("Boundary"),
                    UiLayout::window().y(Rl(11.0)).size(Rl((105.0, 20.0))).pack::<Base>(),
                ));
                ui.spawn((
                    board.add("Boundary/Logo"),
                    UiLayout::solid().size((1240.0, 381.0)).pack::<Base>(),
                    UiImage2dBundle::from(assets.load(PreLoader::MAIN_LOGO)),
                ));


                // #=========================#
                // #=== MAIN MENU BUTTONS ===#

                // Spawn button boundary
                let list = board.add("List");
                ui.spawn((
                    list.clone(),
                    UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack::<Base>(),
                ));

                // Spawn buttons
                let gap = 3.0;
                let size = 14.0;
                let mut offset = 0.0;
                for button in [MainMenuButton::Continue, MainMenuButton::NewGame, MainMenuButton::LoadGame, MainMenuButton::Settings, MainMenuButton::AdditionalContent, MainMenuButton::Credits, MainMenuButton::QuitGame] {

                    let mut btn = ui.spawn((
                        // Link the entity
                        list.add(button.str()),

                        // Add the button type
                        button.clone(),

                        // Add layout
                        UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack::<Base>(),

                        // Add the button component
                        MainButton { text: button.str().into() },
                    ));

                    // Insert specific components if the condition is true
                    if button == MainMenuButton::NewGame {
                        btn.insert((
                            // Despawn this entity on UiClick
                            OnUiClickDespawn::new(route_entity),

                            // Run this command on UiClick
                            OnUiClickCommands::new(|commands| { commands.spawn(CharacterCreatorRoute); })
                        ));
                    }
                    if button == MainMenuButton::Continue {
                        btn.insert((
                            // Despawn this entity on UiClick
                            OnUiClickDespawn::new(route_entity),

                            // Run this command on UiClick
                            OnUiClickCommands::new(|commands| { commands.spawn(GameRoute); })
                        ));
                    }

                    offset += gap + size;
                }
            });
        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// Good practice is to use custom component for buttons, so we can easily know what type of button was pressed
#[derive(Component, Clone, PartialEq)]
enum MainMenuButton {
    Continue,
    NewGame,
    LoadGame,
    Settings,
    AdditionalContent,
    Credits,
    QuitGame,
}
impl MainMenuButton {
    fn str(&self) -> String {
        match self {
            MainMenuButton::Continue => "CONTINUE".into(),
            MainMenuButton::NewGame => "NEW GAME".into(),
            MainMenuButton::LoadGame => "LOAD GAME".into(),
            MainMenuButton::Settings => "FULLSCREEN".into(),
            MainMenuButton::AdditionalContent => "ADDITIONAL CONTENT".into(),
            MainMenuButton::Credits => "CREDITS".into(),
            MainMenuButton::QuitGame => "QUIT GAME".into(),
        }
    }
}

/// In this system we run our button click logic
fn main_menu_button_clicked_system(mut events: EventReader<UiClickEvent>, query: Query<&MainMenuButton, With<MainButton>>, mut exit: EventWriter<bevy::app::AppExit>,
    mut event1: EventWriter<actions::SetWindowMode>,
    mut event2: EventWriter<actions::SetWindowResolution>
) {
    for event in events.read() {
        if let Ok(button) = query.get(event.target) {
            info!("Pressed: {}", button.str());

            // Here we can run code on button click
            match button {
                MainMenuButton::QuitGame => {
                    exit.send(bevy::app::AppExit::Success);
                },
                MainMenuButton::Settings => {
                    event1.send(actions::SetWindowMode(bevy::window::WindowMode::BorderlessFullscreen));
                    event2.send(actions::SetWindowResolution(Vec2::new(1920.0, 1080.0)));
                },
                _ => {},
            }
        }
    }
}


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct MainMenuRoutePlugin;
impl Plugin for MainMenuRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, build_route.before(UiSystems::Compute))
            .add_systems(Update, main_menu_button_clicked_system.run_if(on_event::<UiClickEvent>()));
    }
}

