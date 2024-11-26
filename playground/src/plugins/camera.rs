use std::f32::consts::{FRAC_PI_2, PI, TAU};

use avian3d::prelude::Position;
use bevy::prelude::*;

use crate::plugins::{
    character::{LocalControl, PlayerCharacter},
    input::MouseMovement,
};

/// Plugin that handles player cameras
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit_camera);
    }
}

/// Component to keep track of where the camera is pointing
#[derive(Component, Debug)]
pub struct OrbitCamera {
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            distance: 10.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(OrbitCamera { ..default() });
}

/// handles rotation and translation of the player camera
fn orbit_camera(
    time: Res<Time>,
    local_player_transforms: Query<&Position, (With<PlayerCharacter>, With<LocalControl>)>,
    mut orbit_cameras: Query<
        (&mut Transform, &mut OrbitCamera),
        (Without<PlayerCharacter>, Without<LocalControl>),
    >,
    mouse_movement: Res<MouseMovement>,
) {
    let dt = time.delta_seconds();
    for player_transform in &local_player_transforms {
        for (mut camera_transform, mut orbit_camera) in &mut orbit_cameras {
            if mouse_movement.x != 0.0 {
                orbit_camera.yaw += mouse_movement.x * -0.1 * dt;
                // wrap around, to stay between +- 180 degrees
                if orbit_camera.yaw > PI {
                    orbit_camera.yaw -= TAU;
                }
                if orbit_camera.yaw < -PI {
                    orbit_camera.yaw += TAU;
                }
            }
            if mouse_movement.y != 0.0 {
                orbit_camera.pitch += mouse_movement.y * -0.1 * dt;
                // wrap around, to stay between +- 180 degrees
                if orbit_camera.pitch > PI {
                    orbit_camera.pitch -= TAU;
                }
                if orbit_camera.pitch < -PI {
                    orbit_camera.pitch += TAU;
                }
                //prevent camera from going upside-down
                orbit_camera.pitch = orbit_camera.pitch.clamp(-PI / 2.0, PI / 2.0);
            }

            camera_transform.rotation =
                Quat::from_euler(EulerRot::YXZ, orbit_camera.yaw, orbit_camera.pitch, 0.0);
            camera_transform.translation =
                **player_transform + camera_transform.back() * orbit_camera.distance;
        }
    }
}
