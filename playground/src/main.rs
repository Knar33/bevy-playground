use bevy::prelude::*;

use crate::plugins::{bindings::BindingsPlugin, keyboard::KeyboardPlugin};

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((KeyboardPlugin, BindingsPlugin))
        .run();
}
