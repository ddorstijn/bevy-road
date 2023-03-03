use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use petgraph::graph::{EdgeIndex, NodeIndex};

use super::{curves::arc::BiArc, node::RoadNode, RoadGraph};

#[derive(Component, Default, Debug)]
pub struct RoadEdge {
    arc: BiArc,
    index: EdgeIndex,
}

impl RoadEdge {
    pub fn new(
        mut commands: EntityCommands,
        graph: &mut ResMut<RoadGraph>,
        start: NodeIndex,
        end: NodeIndex,
    ) -> EdgeIndex {
        let index = graph.add_edge(start, end, commands.id());
        commands.insert(Self { index, ..default() });

        index
    }

    fn generate_mesh(&self) -> Mesh {
        const DETAIL: usize = 20;

        let positions = (0..=DETAIL)
            .map(|i| {
                self.arc
                    .interpolate(DETAIL as f32 / i as f32 * self.arc.length())
                    .translation
            })
            .collect::<Vec<Vec3>>();

        positions.iter().for_each(|p| println!("{}", p));

        let normals = vec![Vec3::Y; DETAIL + 1];
        let indices = (0..=DETAIL as u32).collect::<Vec<u32>>();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(mesh::Indices::U32(indices)));

        mesh
    }
}

pub fn update_node_edges(
    mut changed_nodes: Query<(&mut Handle<Mesh>, &RoadNode), Changed<RoadNode>>,
    nodes: Query<&Transform, With<RoadNode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut edges: Query<&mut RoadEdge>,
    graph: Res<RoadGraph>,
) {
    for (mut handle, node) in &mut changed_nodes {
        let outgoing = graph.edges_directed(node.index, petgraph::Direction::Outgoing);
        let incoming = graph.edges_directed(node.index, petgraph::Direction::Incoming);

        for edge_ref in outgoing.chain(incoming) {
            let mut edge = edges.get_mut(*edge_ref.weight()).unwrap();

            let endpoints = graph.edge_endpoints(edge.index).unwrap();
            let t_start = nodes.get(*graph.node_weight(endpoints.0).unwrap()).unwrap();
            let t_end = nodes.get(*graph.node_weight(endpoints.1).unwrap()).unwrap();

            edge.arc = BiArc::new(t_start, t_end);
            println!("{:?}", edge.arc);

            *handle = meshes.add(edge.generate_mesh());
        }
    }
}
