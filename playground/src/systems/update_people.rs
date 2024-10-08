use crate::components::person::*;
use bevy::prelude::*;

pub fn update_people(mut query: Query<&mut crate::components::name::Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
