use avian3d::prelude::*;

use crate::*;

pub const PRONE_MOVEMENT_SPEED: f32 = 1.0;      // ~1 m/s
pub const CROUCH_MOVEMENT_SPEED: f32 = 2.0;     // 1.5-2.5 m/s
//pub const ADS_MOVEMENT_SPEED: f32 = 2.5;        // 2-3 m/s
pub const BASE_MOVEMENT_SPEED: f32 = 3.5;       // 3-4 m/s
pub const RUNNING_MOVEMENT_SPEED: f32 = 7.0;    // 5-8 m/s
pub const SPRINTING_MOVEMENT_SPEED: f32 = 8.0;  // ~8m/s
pub const STRAFING_MOVEMEMENT_SPEED_MULTIPLIER: f32 = 0.85;    // 70-90 %
pub const BACKWARDS_MOVEMEMENT_SPEED_MULTIPLIER: f32 = 0.75;  // 50-80 %
pub const HALF_PI: f32 = std::f32::consts::PI / 2.0;


#[derive(Component, Default)]
pub struct ControllerInput {
    /// Raw input: `[x: +front -back, y: +right -left]` range: `(-1.0 - 1.0)`
    pub raw: Vec2,
    /// Same as Raw, but when `PlayerState:Running` input `y` is scaled down by `0.45` and `x` is set to `1.0`
    pub altered: Vec2,

    pub jump: bool,
}
fn controller_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut ControllerInput>) {
    for mut input in &mut query {
        // Keyboard input
        let forward = keyboard_input.pressed(KeyCode::KeyW) as i8;
        let leftward = keyboard_input.pressed(KeyCode::KeyA) as i8;
        let backward = keyboard_input.pressed(KeyCode::KeyS) as i8;
        let rightward = keyboard_input.pressed(KeyCode::KeyD) as i8;

        // Normalized input vector (+x: front, +y: right) from keyboard (can get data from controller too)
        input.raw = Vec2::new((forward - backward) as f32, (rightward - leftward) as f32).normalize_or_zero();
        input.altered = input.raw;

        input.jump = keyboard_input.just_pressed(KeyCode::Space);
    }
}


#[derive(Component, Default)]
pub struct ControllerPlaneRotation {
    y: f32,
}
fn controller_plane_rotation(capture: Res<MouseCapture>,mut query: Query<(&mut ControllerPlaneRotation, &mut Transform)>) {
    for (mut rotation, mut transform) in &mut query {
        rotation.y -= (capture.delta.x * 0.035 * 0.6).to_radians();
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, rotation.y, 0.0);
    }
}


#[derive(Component, Default)]
pub struct ControllerTiltRotation {
    x: f32,
}
fn controller_tilt_rotation(capture: Res<MouseCapture>,mut query: Query<(&mut ControllerTiltRotation, &mut Transform)>) {
    for (mut rotation, mut transform) in &mut query {
        rotation.x -= (capture.delta.y * 0.035 * 0.6).to_radians();
        rotation.x = rotation.x.clamp(-HALF_PI, HALF_PI);
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, rotation.x);
    }
}


#[derive(Component, Default, Debug, PartialEq)]
pub enum ControllerState {
    Sprinting,
    Running,
    #[default] Base,
    //ADS,
    Crouch,
    Prone,
}
fn controller_state(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut ControllerInput, &mut ControllerState)>) {
    for (mut input, mut state) in &mut query {
        
        // Change to "CROUCH" state
        if keyboard_input.just_pressed(KeyCode::KeyC) {
            if *state != ControllerState::Crouch { *state = ControllerState::Crouch } else {
                *state = ControllerState::Base;
            }
        }

        // Change to "PRONE" state
        if keyboard_input.just_pressed(KeyCode::KeyX) {
            if *state != ControllerState::Prone { *state = ControllerState::Prone } else {
                *state = ControllerState::Base;
            }
        }

        // Change to "RUN" or "SPRINT" state
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
            if *state == ControllerState::Running || *state == ControllerState::Sprinting { *state = ControllerState::Sprinting } else {
                *state = ControllerState::Running;
            }
        }

        // Stop "RUN" or "SPRINT" if player is not moving forward
        if *state == ControllerState::Running || *state == ControllerState::Sprinting {
            if input.raw.y.abs() > 0.8 || input.raw.x < 0.1 {
                *state = ControllerState::Base;
            } else {
                input.altered.x = 1.0;
                input.altered.y *= 0.45;
            }
        }
    }
}
fn controller_movement(mut query: Query<(&ControllerInput, &ControllerState, &ControllerPlaneRotation, &mut LinearVelocity)>, time: Res<Time>) {
    for (input, state, rotation, mut physics) in &mut query {

        // Get the proper movement speed
        let movement_speed = match *state {
            ControllerState::Sprinting => SPRINTING_MOVEMENT_SPEED,
            ControllerState::Running => RUNNING_MOVEMENT_SPEED,
            ControllerState::Base => BASE_MOVEMENT_SPEED,
            //ControllerState::ADS => ADS_MOVEMENT_SPEED,
            ControllerState::Crouch => CROUCH_MOVEMENT_SPEED,
            ControllerState::Prone => PRONE_MOVEMENT_SPEED,
        };

        // Compute the direction offsets
        let local = Vec2 {
            x: input.altered.x * movement_speed * 1.0.lerp(BACKWARDS_MOVEMEMENT_SPEED_MULTIPLIER, input.raw.x.abs() * input.raw.x.is_sign_negative() as i8 as f32 ),
            y: input.altered.y * movement_speed * 1.0.lerp(STRAFING_MOVEMEMENT_SPEED_MULTIPLIER, input.raw.y.abs())
        };

        // Compute two perpendicular vectors for global transformation
        let front_vector =  Vec2::new(-rotation.y.sin(), -rotation.y.cos());
        let right_vector =  Vec2::new(-front_vector.y, front_vector.x);

        // Translate the local directions into a global directions
        let mut global = Vec2::ZERO;
        global += front_vector * local.x;
        global += right_vector * local.y;

        physics.x = global.x;//* time.delta_seconds();
        physics.z = global.y;//* time.delta_seconds();
    }
}


#[derive(Component, Default)]
pub struct ControllerGravity {
    z: f32,
}
fn controller_gravity(mut query: Query<(&mut ControllerGravity, &ControllerInput, &mut LinearVelocity, &CollidingEntities)>, time: Res<Time>) {
    for (mut gravity, input, mut physics, collisions) in &mut query {

        //gravity.z += -10.0 * time.delta_seconds();
        //if collisions.is_empty() { gravity.z = 0.0 }

        if input.jump { physics.z = 60.0 };

        //if !collisions.is_empty() {
        //    physics.y += gravity.z * time.delta_seconds();
        //}
        //info!("{:?}", physics);
    }
}

// #=======================#
// #=== MOVEMENT PLUGIN ===#

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            controller_input,
            controller_plane_rotation,
            controller_tilt_rotation,
            controller_state,
            controller_movement,
            controller_gravity,
        ).chain());
    }
}
