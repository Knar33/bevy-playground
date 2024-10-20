use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::plugins::{
    input,
    input::{Input, Inputs},
};

pub struct BindingsPlugin;

impl Plugin for BindingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindings(HashMap::from([
            (
                InputCombination::Unmodified(Input::Keyboard(KeyCode::KeyW)),
                Action::MoveForward,
            ),
            (
                InputCombination::Unmodified(Input::Keyboard(KeyCode::KeyS)),
                Action::MoveBackwards,
            ),
            (
                InputCombination::Unmodified(Input::Keyboard(KeyCode::KeyA)),
                Action::MoveLeft,
            ),
            (
                InputCombination::Unmodified(Input::Keyboard(KeyCode::KeyD)),
                Action::MoveRight,
            ),
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
            Update,
            convert_inputs_to_actions
                .after(input::read_input_events)
                .after(input::read_mouse_movement_events),
        );
    }
}

/// Hashmap that contains the KeyCombination to Action bindings
#[derive(Resource, Debug, Eq, PartialEq)]
pub struct KeyBindings(pub HashMap<InputCombination, Action>);

/// resource that keeps track of which actions were initiated by keypressed in this frame
#[derive(Resource, Debug)]
pub struct Actions {
    pub pressed: HashSet<Action>,
    pub released: HashSet<Action>,
    pub holding: HashSet<Action>,
}

/// resource that keeps track of which actions were initiated by keypressed since the last FixedUpdate
#[derive(Resource, Debug)]
pub struct FixedActions {
    pub pressed: HashSet<Action>,
    pub released: HashSet<Action>,
    pub holding: HashSet<Action>,
}

/// Enum that defines all actions that can be done by the player
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackwards,
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum InputCombination {
    Unmodified(Input),
    ShiftModified(Input),
    CtrlModified(Input),
    SuperModified(Input),
    AltModified(Input),
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
fn convert_inputs_to_actions(
    mut inputs: ResMut<Inputs>,
    mut actions: ResMut<Actions>,
    mut fixed_actions: ResMut<FixedActions>,
) {
    for pressed in &inputs.pressed {
        let mut input_constructor: fn(Input) -> InputCombination = InputCombination::Unmodified;
        let modified = false;
        if let Input::Keyboard(key) = *pressed {
            if key == KeyCode::ShiftLeft || key == KeyCode::ShiftRight {
                input_constructor = InputCombination::ShiftModified;
            } else if key == KeyCode::ControlLeft || key == KeyCode::ControlRight {
                input_constructor = InputCombination::CtrlModified;
            } else if key == KeyCode::SuperLeft || key == KeyCode::SuperRight {
                input_constructor = InputCombination::SuperModified;
            } else if key == KeyCode::AltLeft || key == KeyCode::AltRight {
                input_constructor = InputCombination::AltModified;
            }
        }
    }
}
