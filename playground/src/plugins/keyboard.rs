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
    Super(KeyCode),
    Alt(KeyCode),
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
/// - Multiple modifiers are being held down and a third key is pressed (priority goes in this order and only applies the first one: shift, control, super, alt)
fn read_keystrokes(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<Keystrokes>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let shift_down = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let control_down = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let super_down = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);
    let alt_down = keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    for event in keyboard_events.read() {
        let modifier = if shift_down {
            KeyboardCombination::Shift
        } else if control_down {
            KeyboardCombination::Control
        } else if super_down {
            KeyboardCombination::Alt
        } else if alt_down {
            KeyboardCombination::Super
        } else {
            KeyboardCombination::None
        };

        match event.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", event.key_code, event.logical_key);
                //TODO: Don't add to pressed if it's already being held down
                keystrokes.pressed.insert(modifier(event.key_code));
                keystrokes.holding.insert(modifier(event.key_code));
            }
            ButtonState::Released => {
                println!(
                    "Key release: {:?} ({:?})",
                    event.key_code, event.logical_key
                );
                keystrokes.released.insert(modifier(event.key_code));
                keystrokes.holding.remove(&modifier(event.key_code));
            }
        }
    }
}

//system on fixedupdate that reads the resource and does something with them
//system on fixedupdate that resets all keys (after the one that reads them)
//system to track window focus to reset keystroke state if the window changes https://bevy-cheatbook.github.io/input/keyboard.html#keyboard-focus
