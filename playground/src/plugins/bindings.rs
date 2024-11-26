use bevy::{
    input::{keyboard::KeyCode, mouse::MouseButtonInput},
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::plugins::{
    input,
    input::{Input, Inputs, ReadInputSet},
};

/// Plugin that handles everything related to keybindings -> actions conversion
pub struct BindingsPlugin;

impl Plugin for BindingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindings(HashMap::from([
            (Action::MoveForward, vec![Input::Keyboard(KeyCode::KeyW)]),
            (Action::MoveLeft, vec![Input::Keyboard(KeyCode::KeyA)]),
            (Action::MoveRight, vec![Input::Keyboard(KeyCode::KeyD)]),
            (Action::MoveBackward, vec![Input::Keyboard(KeyCode::KeyS)]),
            (Action::Jump, vec![Input::Keyboard(KeyCode::Space)]),
        ])));
        app.insert_resource(Actions {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });
        app.insert_resource(FixedActions {
            pressed: HashSet::new(),
            released: HashSet::new(),
            holding: HashSet::new(),
        });

        app.add_systems(
            PreUpdate,
            convert_inputs_to_actions.after(input::ReadInputSet),
        );
        app.add_systems(FixedPostUpdate, clear_fixed_actions);
    }
}

/// Hashmap that contains Actions and their corresponding key combinations
/// If an Action has no binding, it should be removed from the hashmap
#[derive(Resource, Debug, Eq, PartialEq)]
pub struct KeyBindings(pub HashMap<Action, Vec<Input>>);

/// resource that keeps track of which actions were initiated by keypresses in this frame
#[derive(Resource, Debug)]
pub struct Actions {
    pub pressed: HashSet<Action>,
    pub released: HashSet<Action>,
    pub holding: HashSet<Action>,
}

/// resource that keeps track of which actions were initiated by keypresses since the last FixedUpdate
#[derive(Resource, Debug)]
pub struct FixedActions {
    pub pressed: HashSet<Action>,
    pub released: HashSet<Action>,
    pub holding: HashSet<Action>,
}

/// Enum that defines all actions that can be done by the player
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
}

/// system that reads the input hashsets every frame and converts them into actions which are stored in two hashsets (one for update and one for fixedupdate)
fn convert_inputs_to_actions(
    inputs: Res<Inputs>,
    mut actions: ResMut<Actions>,
    mut fixed_actions: ResMut<FixedActions>,
    key_bindings: Res<KeyBindings>,
) {
    let mut new_actions = Actions {
        pressed: HashSet::new(),
        released: HashSet::new(),
        holding: HashSet::new(),
    };

    //find all matching bindings being held
    for (action, keys) in key_bindings.0.iter() {
        if keys.iter().all(|key| inputs.holding.contains(key)) {
            new_actions.holding.insert(*action);
        }
    }
    //filter out clashes
    new_actions.holding = new_actions
        .holding
        .iter()
        .copied()
        .filter(|binding| {
            let key_combination = key_bindings.0.get(binding).unwrap();
            for other_binding in new_actions.holding.iter() {
                let other_key_combination = key_bindings.0.get(other_binding).unwrap();
                for key in key_combination {
                    for other_key in other_key_combination {
                        //TODO: This is only checking for 1 overlapping key - if we want to allow 3-key bindings then this has to check for a full subset
                        if key == other_key && key_combination.len() < other_key_combination.len() {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect();

    //detect released keys
    for action in actions.holding.iter() {
        if !new_actions.holding.contains(action) {
            new_actions.released.insert(*action);
        }
    }

    //detect newly pressed keys
    for action in new_actions.holding.iter() {
        if !actions.holding.contains(action) {
            new_actions.pressed.insert(*action);
        }
    }

    //update action hashsets with new values
    actions.holding.clone_from(&new_actions.holding);
    actions.pressed.clone_from(&new_actions.pressed);
    actions.released.clone_from(&new_actions.released);
    fixed_actions.holding = new_actions.holding;
    for pressed in new_actions.pressed {
        fixed_actions.pressed.insert(pressed);
    }
    for released in new_actions.released {
        fixed_actions.released.insert(released);
    }
}

/// empty the hashsets at the end of the FixedUpdate (not necessary for actions because they are overwritten every frame already)
fn clear_fixed_actions(mut fixed_actions: ResMut<FixedActions>) {
    fixed_actions.pressed.clear();
    fixed_actions.released.clear();
}
