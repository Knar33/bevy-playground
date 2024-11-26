use bevy::prelude::*;

use crate::plugins::{
    bindings::BindingsPlugin, camera::CameraPlugin, character::CharacterPlugin, input::InputPlugin,
    physics::PhysicsPlugin,
};

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            InputPlugin,
            BindingsPlugin,
            PhysicsPlugin,
            CharacterPlugin,
            CameraPlugin,
        ))
        .run();
}
