use bevy::prelude::*;

use crate::plugins::keyboard::KeyboardPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeyboardPlugin)
        .run();
}
