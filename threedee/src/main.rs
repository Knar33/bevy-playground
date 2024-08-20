//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
struct Spinner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keys, mouse))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(6.0)),
        material: materials.add(Color::linear_rgb(0.0, 1.0, 0.0)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });
    // cube
    let cube = commands
        .spawn((
            Spinner,
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        ))
        .id();
    // camera
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-11.0, 9.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();
    commands.entity(cube).push_children(&[camera]);
}

fn keys(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Spinner>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut query {
        let rot = transform.rotation;
        if keyboard.pressed(KeyCode::KeyW) {
            transform.translation +=
                rot * Vec3::X * Vec3::from_array([1.0, 0.0, 1.0]) * time.delta_seconds() * 10.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation -=
                rot * Vec3::X * Vec3::from_array([1.0, 0.0, 1.0]) * time.delta_seconds() * 10.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation += rot * Vec3::Z * time.delta_seconds() * 10.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation -= rot * Vec3::Z * time.delta_seconds() * 10.0;
        }
    }
}

fn mouse(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Spinner>>,
    mut evr_motion: EventReader<MouseMotion>,
) {
    for ev in evr_motion.read() {
        for mut transform in &mut query {
            transform.rotate_local_z(ev.delta.y * -0.01);
            transform.rotate_y(ev.delta.x * -0.01);
            //println!("x: {} y: {}", transform.rotation.z, transform.rotation.y);
        }
    }
}
