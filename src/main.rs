use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use bevy_rapier3d::{
    prelude::{Collider, CollisionGroups, Group, NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin
};
use debug::DebugPlugin;
//use road::RoadPlugin;

pub mod road;
mod debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((WireframePlugin, WorldInspectorPlugin::default()))
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin::default()))
        .add_plugins(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            // .add_plugins(RoadPlugin)
            .add_plugins(DebugPlugin)
            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Environment and player
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
        Name::new("Player")
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            ..default()
        },
        Name::new("Sun")
    ));

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20., ..default() })),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 0.01, 10.0),
        CollisionGroups::new(
            Group::GROUP_1.into(),
            Group::GROUP_1.into(),
        ),
        Name::new("Ground")
    ));
}
