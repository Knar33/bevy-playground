//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Spinner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, setup_physics))
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, (keys, mouse, cursor_grab))
        .run();
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    //primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(Restitution::coefficient(2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        //dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        //println!("Ball altitude: {}", transform.translation.y);
    }
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
            if (transform.rotation.z != 0.0 || transform.rotation.y != 0.0) {
                println!("x: {} y: {}", transform.rotation.z, transform.rotation.y);
            }
        }
    }
}
