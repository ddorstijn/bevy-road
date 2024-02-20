use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::{CameraPlugin, PanOrbitCamera};
use debug::DebugPlugin;
use road::{node::RoadSpawner, RoadPlugin};
use states::GameStatePlugin;

pub mod camera;
mod debug;
pub mod road;
pub mod states;
pub mod utility;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_plugins((CameraPlugin, DebugPlugin, GameStatePlugin, RoadPlugin))
        .register_type::<SelectedRoadNode>()
        .init_resource::<SelectedRoadNode>()
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

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4)),
            mesh: meshes.add(Plane3d::new(Vec3::Y)),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 0.1, 10.0),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        Name::new("Ground"),
    ));

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            mesh: meshes.add(Cuboid {
                half_size: Vec3::splat(0.5),
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                // rotation: Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
        Name::new("Road Spawner"),
        RoadSpawner,
    ));
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct SelectedRoadNode(Option<Entity>);
