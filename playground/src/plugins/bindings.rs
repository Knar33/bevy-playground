use bevy::prelude::*;

pub struct BindingsPlugin;

impl Plugin for BindingsPlugin {
    fn build(&self, app: &mut App) {}
}

//enum for keys + modifiers
//hashmap of binds -> actions
