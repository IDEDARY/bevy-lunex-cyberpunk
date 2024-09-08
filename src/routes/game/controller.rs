use avian3d::prelude::*;

use crate::*;


// #====================#
// #=== PLAYER INPUT ===#

/// Input event that is emmited when player should move.
#[derive(Event, Clone, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct PlayerMove(pub Vec2);

/// Input event that is emmited when player should rotate.
#[derive(Event, Clone, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct PlayerLook(pub Vec2);

/// Input event that is emmited when player should do an action.
#[derive(Event, Clone, Debug, PartialEq)]
pub enum PlayerAct {
    /// Start running
    Run,
    /// Start crouching
    Crouch,
    /// Go prone and lie down
    Prone,
}

/// This function will create all Move and Look events
fn player_input(
    // Input
    mouse_input: Res<MouseCapture>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    gamepad_input: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    // Output
    mut move_player: EventWriter<PlayerMove>,
    mut look_player: EventWriter<PlayerLook>,
    mut act_player: EventWriter<PlayerAct>,
) {
    // Pull gamepad axis values
    let gamepad_move = Vec2::new(
        gamepad_input.get(GamepadAxis { gamepad: Gamepad::new(0), axis_type: GamepadAxisType::LeftStickY }).unwrap_or(0.0),
        gamepad_input.get(GamepadAxis { gamepad: Gamepad::new(0), axis_type: GamepadAxisType::LeftStickX }).unwrap_or(0.0),
    );

    // Pull gamepad axis values
    let gamepad_look = Vec2::new(
        gamepad_input.get(GamepadAxis { gamepad: Gamepad::new(0), axis_type: GamepadAxisType::RightStickX }).unwrap_or(0.0),
        gamepad_input.get(GamepadAxis { gamepad: Gamepad::new(0), axis_type: GamepadAxisType::RightStickY }).unwrap_or(0.0) * -1.0,
    );
    
    // Pull keyboard values
    let forward = keyboard_buttons.pressed(KeyCode::KeyW) as i8;
    let leftward = keyboard_buttons.pressed(KeyCode::KeyA) as i8;
    let backward = keyboard_buttons.pressed(KeyCode::KeyS) as i8;
    let rightward = keyboard_buttons.pressed(KeyCode::KeyD) as i8;
    let keyboard_move = Vec2::new((forward - backward) as f32, (rightward - leftward) as f32).normalize_or_zero();

    // Pull mouse values
    let mouse_look = mouse_input.delta;

    // Send out input events
    if gamepad_move != Vec2::ZERO { move_player.send(PlayerMove(gamepad_move)); }
    if keyboard_move != Vec2::ZERO { move_player.send(PlayerMove(keyboard_move)); }
    if gamepad_look != Vec2::ZERO { look_player.send(PlayerLook(gamepad_look * 15.0)); }
    if mouse_look != Vec2::ZERO { look_player.send(PlayerLook(mouse_look)); }

    // Send out actions
    if keyboard_buttons.just_pressed(KeyCode::KeyX) || gamepad_buttons.just_pressed(GamepadButton::new(Gamepad::new(0), GamepadButtonType::West)) {
        act_player.send(PlayerAct::Prone);
    }
    if keyboard_buttons.just_pressed(KeyCode::KeyC) || gamepad_buttons.just_pressed(GamepadButton::new(Gamepad::new(0), GamepadButtonType::East)) {
        act_player.send(PlayerAct::Crouch);
    }
    if keyboard_buttons.just_pressed(KeyCode::ShiftLeft) || gamepad_buttons.just_pressed(GamepadButton::new(Gamepad::new(0), GamepadButtonType::LeftThumb)) {
        act_player.send(PlayerAct::Run);
    }
}


// #=======================#
// #=== PLAYER MOVEMENT ===#

/// Add this component for Y player rotation
#[derive(Component, Default)]
pub struct PlayerPlaneRotation {
    y: f32,
}
// This function will react to look events
fn player_plane_rotation(
    time: Res<Time>,
    mut look: EventReader<PlayerLook>,
    mut query: Query<(&mut PlayerPlaneRotation, &mut Transform)>,
) {
    for event in look.read() {
        for (mut rotation, mut transform) in &mut query {
            rotation.y -= (event.x * 6.0).to_radians() * time.delta_seconds();
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, rotation.y, 0.0);
        }
    }
}

/// Add this component for X player rotation
#[derive(Component, Default)]
pub struct PlayerTiltRotation {
    x: f32,
}
// This function will react to look events
fn player_tilt_rotation(
    time: Res<Time>,
    mut look: EventReader<PlayerLook>,
    mut query: Query<(&mut PlayerTiltRotation, &mut Transform)>,
) {
    for event in look.read() {
        for (mut rotation, mut transform) in &mut query {
            rotation.x -= (event.y * 6.0).to_radians() * time.delta_seconds();
            rotation.x = rotation.x.clamp(-HALF_PI, HALF_PI);
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, rotation.x);
        }
    }
}


#[derive(Component, Default, Debug, PartialEq)]
pub enum PlayerState {
    Sprinting,
    Running,
    #[default] Base,
    ADS,
    Crouch,
    Prone,
}
fn player_state(
    mut player_act: EventReader<PlayerAct>,
    mut query: Query<&mut PlayerState>,
) {
    for action in player_act.read() {
        for mut state in &mut query {
        
            // Change to "CROUCH" state
            if *action == PlayerAct::Crouch {
                if *state != PlayerState::Crouch { *state = PlayerState::Crouch } else {
                    *state = PlayerState::Base;
                }
            }
    
            // Change to "PRONE" state
            if *action == PlayerAct::Prone {
                if *state != PlayerState::Prone { *state = PlayerState::Prone } else {
                    *state = PlayerState::Base;
                }
            }
    
            // Change to "RUN" or "SPRINT" state
            if *action == PlayerAct::Run {
                if *state == PlayerState::Running || *state == PlayerState::Sprinting { *state = PlayerState::Sprinting } else {
                    *state = PlayerState::Running;
                }
            }
    
            // Stop "RUN" or "SPRINT" if player is not moving forward
            /* if *state == PlayerState::Running || *state == PlayerState::Sprinting {
                if input.raw.y.abs() > 0.8 || input.raw.x < 0.1 {
                    *state = PlayerState::Base;
                } else {
                    input.altered.x = 1.0;
                    input.altered.y *= 0.45;
                }
            } */
        }
    }

}

fn player_movement(
    time: Res<Time>,
    mut player_move: EventReader<PlayerMove>,
    mut query: Query<(&PlayerState, &PlayerPlaneRotation, &mut LinearVelocity)>,
) {
    for event in player_move.read() {
        for (state, rotation, mut physics) in &mut query {

            // Get the proper movement speed
            let movement_speed = match *state {
                PlayerState::Sprinting => SPRINTING_MOVEMENT_SPEED,
                PlayerState::Running => RUNNING_MOVEMENT_SPEED,
                PlayerState::Base => BASE_MOVEMENT_SPEED,
                PlayerState::ADS => ADS_MOVEMENT_SPEED,
                PlayerState::Crouch => CROUCH_MOVEMENT_SPEED,
                PlayerState::Prone => PRONE_MOVEMENT_SPEED,
            };
    
            // Compute the direction offsets
            let local = Vec2 {
                x: event.x * movement_speed * 1.0.lerp(BACKWARDS_MOVEMEMENT_SPEED_MULTIPLIER, event.x.abs() * event.x.is_sign_negative() as i8 as f32 ),
                y: event.y * movement_speed * 1.0.lerp(STRAFING_MOVEMEMENT_SPEED_MULTIPLIER, event.y.abs())
            };
    
            // Compute two perpendicular vectors for global transformation
            let front_vector =  Vec2::new(-rotation.y.sin(), -rotation.y.cos());
            let right_vector =  Vec2::new(-front_vector.y, front_vector.x);
    
            // Translate the local directions into a global directions
            let mut global = Vec2::ZERO;
            global += front_vector * local.x;
            global += right_vector * local.y;
    
            physics.x += global.x * time.delta_seconds() * 4.0;
            physics.z += global.y * time.delta_seconds() * 4.0;
        }
    }
}

// #=====================#
// #=== PLAYER PLUGIN ===#

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerMove>()
            .add_event::<PlayerLook>()
            .add_event::<PlayerAct>()
            .add_systems(Update, (
                player_input,
                player_plane_rotation.run_if(on_event::<PlayerLook>()),
                player_tilt_rotation.run_if(on_event::<PlayerLook>()),
                player_state.run_if(on_event::<PlayerAct>()),
                player_movement.run_if(on_event::<PlayerMove>()),
            ).chain());
    }
}


///////////////////////////////////////////////////////////////////////////////////////

pub const PRONE_MOVEMENT_SPEED: f32 = 1.0;      // ~1 m/s
pub const CROUCH_MOVEMENT_SPEED: f32 = 2.0;     // 1.5-2.5 m/s
pub const ADS_MOVEMENT_SPEED: f32 = 2.5;        // 2-3 m/s
pub const BASE_MOVEMENT_SPEED: f32 = 3.5;       // 3-4 m/s
pub const RUNNING_MOVEMENT_SPEED: f32 = 7.0;    // 5-8 m/s
pub const SPRINTING_MOVEMENT_SPEED: f32 = 8.0;  // ~8m/s
pub const STRAFING_MOVEMEMENT_SPEED_MULTIPLIER: f32 = 0.85;    // 70-90 %
pub const BACKWARDS_MOVEMEMENT_SPEED_MULTIPLIER: f32 = 0.75;  // 50-80 %
pub const HALF_PI: f32 = std::f32::consts::PI / 2.0;

