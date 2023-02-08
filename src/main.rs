use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod flycam;
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod mouse_picking;

pub mod road;
use road::{generate_mesh, RoadEdge, RoadEnd, RoadNode, SelectedNode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadNode>()
            .register_type::<RoadEdge>()
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera)
            .add_system(drag_road_end)
            .add_system(generate_mesh);
    }
}

#[allow(clippy::too_many_arguments)]
fn drag_road_end(
    mut commands: Commands,
    // Pointer Events
    mut drag_start_events: EventReader<PointerDragStart>,
    mut drag_events: EventReader<PointerDrag>,
    mut drag_end_events: EventReader<PointerDragEnd>,
    // Inputs
    pointers: Res<PointerMap>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    locations: Query<&PointerLocation>,
    // Outputs
    mut road_ends: Query<(Entity, &mut Transform, With<RoadEnd>)>,
) {
    // When we start dragging a square, we need to change the focus policy so that picking passes
    // through it. Because the square will be locked to the cursor, it will block the pointer and we
    // won't be able to tell what we are dropping it onto unless we do this.
    for drag_start in drag_start_events.iter() {
        let road = road_ends.get_mut(drag_start.target()).unwrap();
        commands.entity(road.0).remove::<PickRaycastTarget>();
    }

    // While being dragged, update the position of the road to be under the pointer.
    for dragging in drag_events.iter() {
        let pointer_entity = pointers.get_entity(dragging.pointer_id()).unwrap();
        let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
        let pointer_position = pointer_location.position;
        let target = pointer_location
            .target
            .get_render_target_info(&windows, &images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;

        let road = road_ends.get_mut(dragging.target()).unwrap();
        road.1.translation = (pointer_position - (target_size / 2.0)).extend(z);
    }

    //
    for drag_end in drag_end_events.iter() {
        let road = road_ends.get_mut(drag_end.target()).unwrap();
        commands.entity(road.0).insert(PickRaycastTarget::default());
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
        .insert(PickRaycastSource::default())
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
        .insert(PickableBundle::default())
        .insert(PickRaycastTarget::default())
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
                    ..default()
                })
                .insert(PickableBundle::default())
                .insert(RoadEnd);
        });
}
