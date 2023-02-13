use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    flycam::PanOrbitCamera,
    road::{RoadBundle, RoadNode},
};

#[derive(Resource, Default)]
pub struct RoadEndModel {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub collider: Collider,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct NewConnection {
    pub start_node: Option<Entity>,
    pub end_node: Option<Entity>,
}

// Marker component that is clickable to expand the road
#[derive(Component, Reflect)]
pub struct RoadEnd;

#[derive(Bundle)]
pub struct RoadEndBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub collider: Collider,
    pub end: RoadEnd,
}

// Startup systems
pub fn set_road_end_mesh(
    mut road_end_model: ResMut<RoadEndModel>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    road_end_model.mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.05,
        ..default()
    }));
}

// Run criteria
pub fn run_if_dragging(connection: Res<NewConnection>) -> ShouldRun {
    match connection.start_node {
        Some(_) => ShouldRun::Yes,
        None => ShouldRun::No,
    }
}

pub fn run_if_selecting(
    connection: Res<NewConnection>,
    mouse_button: Res<Input<MouseButton>>,
) -> ShouldRun {
    if connection.start_node.is_none() && mouse_button.just_pressed(MouseButton::Left) {
        return ShouldRun::Yes;
    }

    ShouldRun::No
}

// Systems
pub fn mark_node_end(
    query: Query<(Entity, &Children, &RoadNode), Changed<RoadNode>>,
    mut commands: Commands,
) {
    for (entity, children, node) in query.iter() {
        if node.connections.len() > 0 {
            println!("Unmarking node {:?}", entity);

            for child in children.iter() {
                commands.entity(entity).remove_children(&[*child]);
                commands.entity(*child).despawn();
            }

            continue;
        }

        println!("Marking node {:?}", entity);
        commands.get_entity(entity).unwrap().insert(RoadEnd);
    }
}

pub fn drop_node(
    mouse_button: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut connection: ResMut<NewConnection>,
) {
    // Drop selected node
    if mouse_button.just_pressed(MouseButton::Left) {
        commands.entity(connection.start_node.unwrap()).despawn();

        // Reset connection
        connection.start_node = None;
        connection.end_node = None;

        return;
    }
}

pub fn drag_node(
    ctx: Res<RapierContext>,
    camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    windows: Res<Windows>,
    connection: Res<NewConnection>,
    mut road_nodes: Query<&mut Transform, With<RoadNode>>,
) {
    let node = match connection.end_node {
        Some(x) => x,
        None => return,
    };

    let ray = match build_screen_ray(camera, windows) {
        Some(x) => x,
        None => return,
    };

    let filter = QueryFilter::new().groups(CollisionGroups::new(
        Group::GROUP_1.into(),
        Group::GROUP_1.into(),
    ));

    if let Some((_, toi)) = ctx.cast_ray(ray.origin, ray.direction, f32::MAX, false, filter) {
        let hit_point = ray.origin + ray.direction * toi;
        match road_nodes.get_mut(node) {
            Ok(mut x) => {
                x.translation = Vec3::new(hit_point.x, 0.0, hit_point.z);
            }
            Err(_) => {
                return;
            }
        }
    }
}

pub fn select_node(
    ctx: Res<RapierContext>,
    camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    windows: Res<Windows>,
    model: Res<RoadEndModel>,
    road_ends: Query<With<RoadEnd>>,
    mut connection: ResMut<NewConnection>,
    mut commands: Commands,
) {
    let ray = match build_screen_ray(camera, windows) {
        Some(x) => x,
        None => return,
    };

    println!("Firing selecting ray");

    if let Some((entity, _)) = ctx.cast_ray(
        ray.origin,
        ray.direction,
        f32::MAX,
        false,
        QueryFilter::new().predicate(&|e| road_ends.contains(e)),
    ) {
        let end_node = commands
            .spawn((
                RoadBundle {
                    pbr: PbrBundle {
                        material: model.material.clone(),
                        mesh: model.mesh.clone(),
                        ..default()
                    },
                    node: RoadNode::default(),
                },
                Name::new("New node"),
            ))
            .id();

        println!("New node: {:?}", end_node);
        connection.start_node = Some(entity);
        connection.end_node = Some(end_node);
    }
}

fn build_screen_ray(
    camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    windows: Res<Windows>,
) -> Option<Ray> {
    // Build ray from screenspace
    let cursor_pos = match windows
        .get_primary()
        .expect("No window found for game")
        .cursor_position()
    {
        Some(pos) => pos,
        None => {
            return None;
        }
    };

    // Get main camera and it's transform
    let (cam, cam_transform) = camera.get_single().unwrap();

    Some(cam.viewport_to_world(cam_transform, cursor_pos).unwrap())
}
