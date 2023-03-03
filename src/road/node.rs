use bevy::{ecs::system::EntityCommands, prelude::*};
use petgraph::graph::NodeIndex;

use super::RoadGraph;

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

#[derive(Component)]
pub struct RoadNodeGroup;

impl RoadNode {
    pub fn new(
        mut commands: EntityCommands,
        graph: &mut ResMut<RoadGraph>,
        r#type: NodeType,
    ) -> NodeIndex {
        let index = graph.add_node(commands.id());

        commands.insert(Self { r#type, index });

        index
    }
}
