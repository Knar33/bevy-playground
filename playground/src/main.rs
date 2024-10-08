use std::collections::HashMap;

use bevy::prelude::*;

use bevy::tasks::futures_lite::future;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};

use crate::plugins::hello_plugin::HelloPlugin;

mod components;
mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
