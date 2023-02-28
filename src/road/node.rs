use bevy::{ecs::system::EntityCommands, prelude::*};
use petgraph::{graph::NodeIndex, Graph};

#[derive(Reflect)]
pub enum NodeType {
    Incomming,
    Outgoing,
}

#[derive(Component)]
pub struct RoadNode {
    pub r#type: NodeType,
    pub index: NodeIndex,
}

impl RoadNode {
    pub fn new(
        mut commands: EntityCommands,
        graph: &mut Graph<Entity, Entity>,
        r#type: NodeType,
    ) -> NodeIndex {
        let index = graph.add_node(commands.id());

        commands.insert(Self { r#type, index });

        index
    }
}
