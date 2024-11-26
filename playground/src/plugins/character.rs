use crate::plugins::{
    bindings::{Action, FixedActions},
    camera::OrbitCamera,
    input::MouseMovement,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

/// Plugin that handles the player movement and character controller
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
        app.add_systems(
            FixedUpdate,
            (move_character, rotate_character).in_set(TnuaUserControlsSystemSet),
        );
    }
}

/// struct to designate an entity as a PlayerCharacter
#[derive(Component)]
pub struct PlayerCharacter;

/// struct to designate an entity as locally controlled
#[derive(Component)]
pub struct LocalControl;

/// system that spawns the player character into the world
fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule(0.5, 1.0))
        .insert(TnuaControllerBundle::default())
        //.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
        .insert(materials.add(StandardMaterial {
            base_color: Color::srgb(2.0, 0.0, 0.0),
            ..default()
        }))
        .insert(PlayerCharacter)
        .insert(LocalControl);
}

/// system that handles moving a locally controlled player character
fn move_character(
    actions: Res<FixedActions>,
    mut player_query: Query<
        (&mut TnuaController, &Rotation),
        (With<PlayerCharacter>, With<LocalControl>),
    >,
) {
    for (mut controller, rotation) in &mut player_query {
        let mut movement_vector = Vec3::ZERO;
        if actions.holding.contains(&Action::MoveForward) {
            movement_vector -= *rotation * Vec3::Z;
        }
        if actions.holding.contains(&Action::MoveBackward) {
            movement_vector += *rotation * Vec3::Z;
        }
        if actions.holding.contains(&Action::MoveRight) {
            movement_vector += *rotation * Vec3::X;
        }
        if actions.holding.contains(&Action::MoveLeft) {
            movement_vector -= *rotation * Vec3::X;
        }

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: movement_vector.normalize_or_zero() * 100.0,
            float_height: 5.0,
            ..Default::default()
        });

        println!("{:?}", controller.is_airborne());
        if actions.holding.contains(&Action::Jump) {
            controller.action(TnuaBuiltinJump {
                height: 4.0,
                ..Default::default()
            });
        }
    }
}

/// system that handles moving a locally controlled player character
fn rotate_character(
    time: Res<Time>,
    mut rotations: Query<&mut Rotation, (With<PlayerCharacter>, With<LocalControl>)>,
    orbit_cameras: Query<
        &Transform,
        (
            With<OrbitCamera>,
            Without<PlayerCharacter>,
            Without<LocalControl>,
        ),
    >,
) {
    for mut rotation in &mut rotations {
        let orbit_camera = orbit_cameras.single();
        *rotation = Rotation(Quat::from_rotation_y(
            orbit_camera.rotation.to_euler(EulerRot::YXZ).0,
        ));
    }
}
