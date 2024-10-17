use bevy::{
    input::keyboard::NativeKeyCode,
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::plugins::{keyboard::Keystrokes, mouse::MouseClicks};

pub struct BindingsPlugin;

impl Plugin for BindingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindings(HashMap::from([
            (
                KeyCombination::UnmodifiedKey(KeyCode::KeyW),
                Action::MoveForward,
            ),
            (
                KeyCombination::UnmodifiedKey(KeyCode::KeyS),
                Action::MoveBackwards,
            ),
            (
                KeyCombination::UnmodifiedKey(KeyCode::KeyA),
                Action::MoveLeft,
            ),
            (
                KeyCombination::UnmodifiedKey(KeyCode::KeyD),
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
    }
}

/// Hashmap that contains the KeyCombination to Action bindings
#[derive(Resource, Debug, Eq, PartialEq)]
pub struct KeyBindings(pub HashMap<KeyCombination, Action>);

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
pub enum KeyCombination {
    UnmodifiedKey(KeyCode),
    ShiftModifiedKey(KeyCode),
    CtrlModifiedKey(KeyCode),
    SuperModifiedKey(KeyCode),
    AltModifiedKey(KeyCode),
    UnmodifiedButton(MouseButton),
    ShiftModifiedButton(MouseButton),
    CtrlModifiedButton(MouseButton),
    SuperModifiedButton(MouseButton),
    AltModifiedButton(MouseButton),
}

/// system that reads keystrokes every frame and records them to a HashSet resource Keystrokes
fn convert_inputs_to_actions(
    mut keystrokes: ResMut<Keystrokes>,
    mut mouse_clicks: ResMut<MouseClicks>,
    mut actions: ResMut<Actions>,
    mut fixed_actions: ResMut<FixedActions>,
) {
    for pressed in &keystrokes.pressed {
        if keystrokes.holding.contains(&KeyCode::ShiftLeft)
            || keystrokes.holding.contains(&KeyCode::ShiftRight)
        {}
    }
}
