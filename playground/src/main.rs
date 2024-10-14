use bevy::prelude::*;

use crate::plugins::{bindings::BindingsPlugin, keyboard::KeyboardPlugin, mouse::MousePlugin};

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((KeyboardPlugin, MousePlugin, BindingsPlugin))
        .run();
}
