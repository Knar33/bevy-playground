use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use std::collections::HashSet;

//system that reads keystrokes every frame and records them
//resource that keeps track of which keys were pressed every frame and stores them until they can be read by Fixed Update
//system on fixedupdate that reads the resource and does something with them
//system on fixedupdate that resets all keys (after the one that reads them)

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

#[derive(Resource)]
struct Keystrokes {
    pressed: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
    holding: HashSet<KeyCode>,
}

fn read_keystrokes(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<Keystrokes>,
) {
    for event in keyboard_events.read() {
        match event.state {
            //TODO: factor in modifiers
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", event.key_code, event.logical_key);
                keystrokes.pressed.insert(event.key_code);
                keystrokes.holding.insert(event.key_code);
            }
            ButtonState::Released => {
                println!(
                    "Key release: {:?} ({:?})",
                    event.key_code, event.logical_key
                );
                keystrokes.released.insert(event.key_code);
                keystrokes.holding.remove(&event.key_code);
            }
        }
    }
}
