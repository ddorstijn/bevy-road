use std::f32::consts::PI;

use bevy::prelude::*;
use camera::{CameraPlugin, PanOrbitCamera};
use debug::DebugPlugin;
use road::{node::RoadSpawner, RoadPlugin};
use states::GameStatePlugin;

pub mod camera;
mod debug;
pub mod raycast;
pub mod road;
pub mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, DebugPlugin, GameStatePlugin, RoadPlugin))
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
            transform: Transform::from_translation(Vec3::new(0.0, 5., 1.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
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

    let plane = Plane3d::new(Vec3::Y).mesh().size(100.0, 100.0).build();
    commands.spawn((
        plane.compute_aabb().unwrap(),
        PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4)),
            mesh: meshes.add(plane),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            ..default()
        },
        GroundMarker,
        Name::new("Ground"),
    ));

    let cuboid = Cuboid::from_size(Vec3::ONE).mesh();
    let aabb = cuboid.compute_aabb().unwrap();
    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            mesh: meshes.add(cuboid),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                // rotation: Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
                ..default()
            },
            ..default()
        },
        aabb,
        Name::new("Road Spawner"),
        RoadSpawner,
    ));
}

#[derive(Component)]
pub struct GroundMarker;
