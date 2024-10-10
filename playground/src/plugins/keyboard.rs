use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
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
        app.add_systems(Update, read_keystrokes);
    }
}

#[derive(Eq, Hash, PartialEq)]
enum KeyboardCombination {
    None(KeyCode),
    Shift(KeyCode),
    Control(KeyCode),
    Alt(KeyCode),
    Command(KeyCode),
}

/// resource that keeps track of which keys were pressed every frame and stores them until they can be read by Fixed Update
#[derive(Resource)]
struct Keystrokes {
    pressed: HashSet<KeyboardCombination>,
    released: HashSet<KeyboardCombination>,
    holding: HashSet<KeyboardCombination>,
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
/// edge cases to consider:
/// - modifier and key held down, then one released while the other is held down still
/// - key that was held down previous frame is quickly released then held down again (will have a released, pressed, and holding entry)
/// - key press + key press release, repeated twice before the next fixed update - will only show up as a single press and release
fn read_keystrokes(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<Keystrokes>,
) {
    for event in keyboard_events.read() {
        match event.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", event.key_code, event.logical_key);
                keystrokes
                    .pressed
                    .insert(KeyboardCombination::None(event.key_code));
                keystrokes
                    .holding
                    .insert(KeyboardCombination::None(event.key_code));
            }
            ButtonState::Released => {
                println!(
                    "Key release: {:?} ({:?})",
                    event.key_code, event.logical_key
                );
                keystrokes
                    .released
                    .insert(KeyboardCombination::None(event.key_code));
                keystrokes
                    .holding
                    .remove(&KeyboardCombination::None(event.key_code));
            }
        }
    }
}

//system on fixedupdate that reads the resource and does something with them
//system on fixedupdate that resets all keys (after the one that reads them)
//system to track window focus to reset keystroke state if the window changes https://bevy-cheatbook.github.io/input/keyboard.html#keyboard-focus
