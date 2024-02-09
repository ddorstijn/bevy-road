use bevy::prelude::*;

use petgraph::graph::Graph;

use self::{
    edge::{update_node_edges, RoadEdge},
    node::RoadNode,
};

pub mod curves;
mod edge;
mod node;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct RoadGraph(Graph<Entity, Entity>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadGraph>()
            .add_systems(Startup, test_scene)
            .add_systems(Update, update_node_edges);
    }
}

fn test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graph: ResMut<RoadGraph>,
) {
    let node1 = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.4, 0.0, 0.4).into()),
            transform: Transform::from_translation(Vec3::new(-6.0, 0.0, -2.0))
                .looking_at(Vec3::new(2.0, 0.0, -6.0), Vec3::Y),
            ..default()
        },
        Name::new("Node 1"),
    ));
    let n1 = RoadNode::new(node1, &mut graph, node::NodeType::Outgoing);

    let node2 = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                .looking_at(Vec3::new(5.0, 0.0, -9.0), Vec3::Y),
            ..default()
        },
        Name::new("Node 2"),
    ));
    let n2 = RoadNode::new(node2, &mut graph, node::NodeType::Incomming);

    let edge = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                .looking_at(Vec3::new(5.0, 0.0, -9.0), Vec3::Y),
            ..default()
        },
        Name::new("edge 1"),
    ));

    RoadEdge::new(edge, &mut graph, n1, n2);
}
