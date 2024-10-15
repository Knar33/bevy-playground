use bevy::{
    input::{
        keyboard::{KeyboardFocusLost, KeyboardInput},
        ButtonState, InputSystem,
    },
    prelude::*,
    utils::HashSet,
};

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Keystrokes {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });
        app.add_systems(PreUpdate, read_keyboard_events.after(InputSystem));
        app.add_systems(Update, detect_keyboard_focus_lost);
        app.add_systems(PostUpdate, cleanup_keystrokes);
    }
}

/// resource that keeps track of which keys were pressed every frame to be converted into bindings
#[derive(Resource, Debug)]
struct Keystrokes {
    pressed: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
    holding: HashSet<KeyCode>,
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
fn read_keyboard_events(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<Keystrokes>,
) {
    for event in keyboard_events.read() {
        match event.state {
            ButtonState::Pressed => {
                keystrokes.pressed.insert(event.key_code);
                keystrokes.holding.insert(event.key_code);
            }
            ButtonState::Released => {
                keystrokes.released.insert(event.key_code);
                keystrokes.holding.remove(&event.key_code);
            }
        }
    }
}

/// system that resets all pressed and released keys but maintains the holding keys
fn cleanup_keystrokes(mut keystrokes: ResMut<Keystrokes>) {
    keystrokes.pressed.clear();
    keystrokes.released.clear();
}

/// system to track window focus to reset keystroke state if the window changes release all held keys
fn detect_keyboard_focus_lost(
    mut keyboard_focus_lost: EventReader<KeyboardFocusLost>,
    mut keystrokes: ResMut<Keystrokes>,
) {
    if !keyboard_focus_lost.is_empty() {
        keystrokes.holding.clear();
        keyboard_focus_lost.clear();
    }
}
