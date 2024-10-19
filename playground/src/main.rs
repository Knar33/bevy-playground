use bevy::prelude::*;

use crate::plugins::{bindings::BindingsPlugin, input::InputPlugin};

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((InputPlugin, BindingsPlugin))
        .run();
}
