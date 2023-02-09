use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod flycam;
use bevy_rapier3d::{
    prelude::{Collider, CollisionGroups, Group, NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod mouse_picking;

pub mod road;
use mouse_picking::drag_road;
use road::{generate_mesh, RoadEdge, RoadEnd, RoadNode, SelectedNode};

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
            .init_resource::<SelectedNode>()
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera)
            .add_system(drag_road)
            // .add_system(drag_road_end)
            .add_system(generate_mesh);
    }
}

// fn update_road_end(
//     cursors: Query<&Intersection<PickingRaycastSet>>,
//     mut query: Query<&mut Transform, With<RoadEnd>>,
//     selected_node: Res<SelectedNode>,
// ) {
//     if selected_node.node.is_none() {
//         return;
//     }

//     // Set the cursor translation to the top pick's world coordinates
//     let intersection = match cursors.iter().last() {
//         Some(x) => x,
//         None => return,
//     };

//     if let Some(new_matrix) = intersection.normal_ray() {
//         let entity = selected_node.node.unwrap();
//         let mut s = query
//             .get_mut(entity)
//             .expect("Selected node did not have a RoadNode");
//         s.translation = Transform::from_matrix(new_matrix.to_transform()).translation;
//     }
// }

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
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(10.0, 0.01, 10.0))
        .insert(CollisionGroups::new(
            Group::GROUP_2.into(),
            Group::GROUP_2.into(),
        ))
        .insert(Name::new("Ground"));

    // Road system
    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.15, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RoadNode::default())
        .insert(Name::new("Start node"))
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    material: materials.add(Color::rgb(1., 0., 0.).into()),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Collider::cuboid(0.13, 0.13, 0.13))
                .insert(CollisionGroups::new(
                    Group::GROUP_1.into(),
                    Group::GROUP_1.into(),
                ))
                .insert(Name::new("Road End"))
                .insert(RoadEnd);
        });
}
