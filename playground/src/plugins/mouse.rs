use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState, InputSystem,
    },
    prelude::*,
    utils::HashSet,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseClicks {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });
        app.insert_resource(MouseMovement { x: 0., y: 0. });
        app.add_systems(PreUpdate, mouse_button_events.after(InputSystem));
        app.add_systems(FixedPostUpdate, cleanup_mouse_movement);
    }
}

/// resource that keeps track of which buttons were pressed every frame and stores them until they can be read by Fixed Update
#[derive(Resource, Debug)]
struct MouseClicks {
    pressed: HashSet<MouseButton>,
    released: HashSet<MouseButton>,
    holding: HashSet<MouseButton>,
}

/// resource that keeps a running total of mouse movement between each update until FixedUpdate is able to process it
#[derive(Resource, Debug)]
struct MouseMovement {
    x: f32,
    y: f32,
}

//system that reads mouse clicks and cursor movement and stores them in the appropriate resources
fn mouse_button_events(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_clicks: ResMut<MouseClicks>,
    mut mouse_movement: ResMut<MouseMovement>,
) {
    for event in mouse_button_events.read() {
        match event.state {
            ButtonState::Pressed => {
                if !mouse_clicks.holding.contains(&event.button) {
                    mouse_clicks.pressed.insert(event.button);
                }
                mouse_clicks.holding.insert(event.button);
            }
            ButtonState::Released => {
                mouse_clicks.released.insert(event.button);
                mouse_clicks.holding.remove(&event.button);
            }
        }
    }

    for event in mouse_motion_events.read() {
        mouse_movement.x += event.delta.x;
        mouse_movement.y += event.delta.y;
    }
}

/// system that resets all mouse movement at the end of fixed update
fn cleanup_mouse_movement(mut mouse_movement: ResMut<MouseMovement>) {
    mouse_movement.x = 0.;
    mouse_movement.y = 0.;
}
