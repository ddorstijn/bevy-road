use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Graph,
};

use super::{curves::arc::BiArc, node::RoadNode, RoadGraph};

#[derive(Debug, Default)]
pub enum EdgeType {
    #[default]
    Connection,
    Transition,
}

#[derive(Component, Default, Debug)]
pub struct RoadEdge {
    arc: BiArc,
    index: EdgeIndex,
    r#type: EdgeType,
}

impl RoadEdge {
    pub fn new(
        mut commands: EntityCommands,
        graph: &mut Graph<Entity, Entity>,
        start: NodeIndex,
        end: NodeIndex,
    ) -> EdgeIndex {
        let index = graph.add_edge(start, end, commands.id());
        commands.insert(Self { index, ..default() });

        index
    }

    fn generate_mesh(&self) -> Mesh {
        const DETAIL: usize = 20;

        let positions = (0..DETAIL)
            .map(|i| {
                self.arc
                    .interpolate(DETAIL as f32 / i as f32 * self.arc.length())
                    .translation
            })
            .collect::<Vec<Vec3>>();

        let normals = vec![Vec3::Y; DETAIL];
        let indices = (0..DETAIL as u32).collect::<Vec<u32>>();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(mesh::Indices::U32(indices)));

        mesh
    }
}

fn update_node_edges(
    changed_nodes: Query<&RoadNode, Changed<RoadNode>>,
    edges: Query<&RoadEdge>,
    graph: Res<RoadGraph>,
) {
    for node in changed_nodes.into_iter() {
        let outgoing = graph
            .0
            .edges_directed(node.index, petgraph::Direction::Outgoing);

        let incoming = graph
            .0
            .edges_directed(node.index, petgraph::Direction::Incoming);

        for edge in outgoing.chain(incoming) {
            println!("{:?}", edges.get(*edge.weight()).unwrap());
        }
    }
}
