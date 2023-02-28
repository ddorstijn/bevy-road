use bevy::{pbr::wireframe::Wireframe, prelude::*};
use petgraph::graph::Graph;

use self::{
    edge::RoadEdge,
    node::RoadNode,
    nodegroup::{generate_intersection_mesh, RoadNodeGroup},
};

pub mod curves;
mod edge;
mod node;
mod nodegroup;

#[derive(Resource, Default)]
pub struct RoadGraph(Graph<Entity, Entity>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadGraph>()
            .add_startup_system(test_scene)
            .add_system(generate_intersection_mesh);
    }
}

fn test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graph: ResMut<RoadGraph>,
) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..default()
                },
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                ..default()
            },
            RoadNodeGroup,
            Wireframe,
            Name::new("Group"),
        ))
        .with_children(|parent| {
            let node1 = parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
                    material: materials.add(Color::rgb(0.4, 0.0, 0.4).into()),
                    transform: Transform::from_translation(Vec3::new(-6.0, 0.0, -2.0))
                        .looking_at(Vec3::new(2.0, 0.0, -6.0), Vec3::Y),
                    ..default()
                },
                Name::new("Node 1"),
            ));
            let n1 = RoadNode::new(node1, &mut graph.0, node::NodeType::Outgoing);

            let node2 = parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
                    material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
                    transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                        .looking_at(Vec3::new(5.0, 0.0, -9.0), Vec3::Y),
                    ..default()
                },
                Name::new("Node 2"),
            ));
            let n2 = RoadNode::new(node2, &mut graph.0, node::NodeType::Incomming);

            let edge = parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
                    material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
                    transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                        .looking_at(Vec3::new(5.0, 0.0, -9.0), Vec3::Y),
                    ..default()
                },
                Name::new("Node 2"),
            ));

            RoadEdge::new(edge, &mut graph.0, n1, n2);
        });
}
