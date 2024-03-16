use std::f32::consts::PI;

use bevy::prelude::*;
use camera::{CameraPlugin, PanOrbitCamera};
use debug::DebugPlugin;
use road::{RoadPlugin, RoadSpawner};
use states::GameStatePlugin;

pub mod camera;
mod debug;
pub mod raycast;
pub mod road;
pub mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, GameStatePlugin, RoadPlugin))
        // .add_plugins(DebugPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Environment and player
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10., 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::X, -0.5 * PI)),
            ..default()
        },
        Name::new("Player"),
        PanOrbitCamera { ..default() },
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
        Name::new("Sun"),
    ));

    let cuboid = Cuboid::from_size(Vec3::ONE).mesh();
    let aabb = cuboid.compute_aabb().unwrap();
    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            mesh: meshes.add(cuboid),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
                ..default()
            },
            ..default()
        },
        aabb,
        Name::new("Road Spawner"),
        RoadSpawner,
    ));
}
