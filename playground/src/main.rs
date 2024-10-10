use bevy::prelude::*;

use crate::plugins::hello_plugin::HelloPlugin;
use crate::plugins::keyboard::KeyboardPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(HelloPlugin)
        .add_plugins(KeyboardPlugin)
        .run();
}
