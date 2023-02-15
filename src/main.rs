use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod flycam;
use bevy_rapier3d::{
    prelude::{Collider, CollisionGroups, Group, NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod road;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadNode>()
            .register_type::<RoadEdge>()
            .add_plugin(RoadPlugin)
            .init_resource::<NewConnection>()
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    // Environment and player
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert(Name::new("Player"));

    const HALF_SIZE: f32 = 10.0;
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                // Configure the projection to better fit the scene
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Sun"));
    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20. })),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(10.0, 0.01, 10.0))
        .insert(CollisionGroups::new(
            Group::GROUP_1.into(),
            Group::GROUP_1.into(),
        ))
        .insert(Name::new("Ground"));

    // Road system
    commands
        .spawn(RoadBundle {
            pbr: PbrBundle {
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.025,
                    ..default()
                })),
                ..default()
            },
            node: RoadNode::default(),
        })
        .insert(Name::new("Start node"));
}
