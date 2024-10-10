use bevy::{
    input::{keyboard::KeyboardInput, ButtonState, InputSystem},
    prelude::*,
};
use std::collections::HashSet;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Keystrokes {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });
        app.add_systems(PreUpdate, read_keyboard_events.after(InputSystem));
        app.add_systems(FixedUpdate, read_keystrokes_fixed);
    }
}

/// resource that keeps track of which keys were pressed every frame and stores them until they can be read by Fixed Update
#[derive(Resource, Debug)]
struct Keystrokes {
    pressed: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
    holding: HashSet<KeyCode>,
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
/// edge cases to consider:
/// - modifier and key held down, then one released while the other is held down still
/// - key that was held down previous frame is quickly released then held down again (will have a released, pressed, and holding entry)
/// - key press + key press release, repeated twice before the next fixed update - will only show up as a single press and release
/// - Multiple modifiers are being held down and a third key is pressed (priority goes in this order and only applies the first one: shift, control, super, alt)
/// - multiple keybindings are pressed at the same time
/// - key being held down gets spammed, don't send a bunch of pressed events
/// modifier pressed and released, then key pressed, all before next fixedupdate - treat it like a modified key press?
fn read_keyboard_events(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<Keystrokes>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for event in keyboard_events.read() {
        match event.state {
            ButtonState::Pressed => {
                //this handles holding keys down and the OS spams the key
                if !keystrokes.holding.contains(&event.key_code) {
                    keystrokes.pressed.insert(event.key_code);
                }
                keystrokes.holding.insert(event.key_code);
            }
            ButtonState::Released => {
                keystrokes.released.insert(event.key_code);
                keystrokes.holding.remove(&event.key_code);
            }
        }
    }
}

/// system on fixedupdate that reads the resource and does something with them
/// resets all pressed and released keys but maintains the holding keys
fn read_keystrokes_fixed(mut keystrokes: ResMut<Keystrokes>) {
    for pressed in &keystrokes.pressed {
        println!("Pressed {:?}", pressed);
    }
    keystrokes.pressed.clear();

    for released in &keystrokes.released {
        println!("Released {:?}", released);
    }
    keystrokes.released.clear();

    for holding in &keystrokes.holding {
        println!("Holding {:?}", holding);
    }
}

//system to track window focus to reset keystroke state if the window changes https://bevy-cheatbook.github.io/input/keyboard.html#keyboard-focus release all held keys
