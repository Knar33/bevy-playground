use bevy::{
    input::{
        keyboard::{KeyboardFocusLost, KeyboardInput},
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState, InputSystem,
    },
    prelude::*,
    utils::HashSet,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inputs {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });
        app.insert_resource(MouseMovement { x: 0., y: 0. });
        app.insert_resource(FixedMouseMovement { x: 0., y: 0. });
        app.add_systems(
            PreUpdate,
            (read_input_events, read_mouse_movement_events).after(InputSystem),
        );
        app.add_systems(FixedPostUpdate, cleanup_fixed_mouse_movement);
        app.add_systems(Update, detect_keyboard_focus_lost);
        app.add_systems(PostUpdate, cleanup_inputs);
    }
}

/// resource that keeps track of which keys were pressed every frame to be converted into bindings
#[derive(Resource, Debug)]
pub struct Inputs {
    pub pressed: HashSet<Input>,
    pub released: HashSet<Input>,
    pub holding: HashSet<Input>,
}

/// resource that records mouse movement every frame
#[derive(Resource, Debug)]
pub struct MouseMovement {
    pub x: f32,
    pub y: f32,
}

/// resource that keeps a running total of mouse movement between each frame until FixedUpdate is able to process it
#[derive(Resource, Debug)]
pub struct FixedMouseMovement {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Input {
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
pub fn read_input_events(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut inputs: ResMut<Inputs>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
) {
    for event in keyboard_events.read() {
        match event.state {
            ButtonState::Pressed => {
                inputs.pressed.insert(Input::Keyboard(event.key_code));
                inputs.holding.insert(Input::Keyboard(event.key_code));
            }
            ButtonState::Released => {
                inputs.released.insert(Input::Keyboard(event.key_code));
                inputs.holding.remove(&Input::Keyboard(event.key_code));
            }
        }
    }

    for event in mouse_button_events.read() {
        match event.state {
            ButtonState::Pressed => {
                inputs.pressed.insert(Input::Mouse(event.button));
                inputs.holding.insert(Input::Mouse(event.button));
            }
            ButtonState::Released => {
                inputs.released.insert(Input::Mouse(event.button));
                inputs.holding.remove(&Input::Mouse(event.button));
            }
        }
    }
}

//system that reads mouse clicks and cursor movement and stores them in the appropriate resources
pub fn read_mouse_movement_events(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_movement: ResMut<MouseMovement>,
    mut fixed_mouse_movement: ResMut<FixedMouseMovement>,
) {
    for event in mouse_motion_events.read() {
        mouse_movement.x = event.delta.x;
        mouse_movement.y = event.delta.y;
        fixed_mouse_movement.x += event.delta.x;
        fixed_mouse_movement.y += event.delta.y;
    }
}

/// system to track window focus to reset keystroke state if the window changes release all held keys
pub fn detect_keyboard_focus_lost(
    mut keyboard_focus_lost: EventReader<KeyboardFocusLost>,
    mut inputs: ResMut<Inputs>,
) {
    if !keyboard_focus_lost.is_empty() {
        inputs.holding.clear();
        keyboard_focus_lost.clear();
    }
}

/// system that resets all pressed and released keys but maintains the holding keys
pub fn cleanup_inputs(mut inputs: ResMut<Inputs>) {
    inputs.pressed.clear();
    inputs.released.clear();
}

/// system that resets all mouse movement at the end of fixed update
pub fn cleanup_fixed_mouse_movement(mut mouse_movement: ResMut<FixedMouseMovement>) {
    mouse_movement.x = 0.;
    mouse_movement.y = 0.;
}
