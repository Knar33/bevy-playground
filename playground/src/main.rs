use bevy::prelude::*;

use crate::plugins::hello_plugin::HelloPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
