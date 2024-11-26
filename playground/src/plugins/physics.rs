use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{control_helpers::TnuaCrouchEnforcerPlugin, prelude::TnuaControllerPlugin};
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

//Plugin that adds and configures the physics system
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));
        app.add_plugins(TnuaAvian3dPlugin::new(FixedUpdate));
        app.add_plugins(TnuaControllerPlugin::new(FixedUpdate));
        //app.add_plugins(TnuaCrouchEnforcerPlugin::new(FixedUpdate));
        app.add_plugins(PhysicsDebugPlugin::new(FixedUpdate));

        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Ground
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Static)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::new(100.0, 0.1, 100.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
}
