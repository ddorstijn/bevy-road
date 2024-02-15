use bevy::{ecs::system::EntityCommands,  prelude::*};
use petgraph::graph::NodeIndex;

use super::RoadGraph;

#[derive(Component)]
pub struct RoadNode {
    pub index: NodeIndex,
}

#[derive(Component)]
pub struct RoadSpawner; 

#[derive(Component)]
pub struct RoadNodeGroup;

impl RoadNode {
    pub fn new(
        mut commands: EntityCommands,
        graph: &mut ResMut<RoadGraph>,
    ) -> NodeIndex {
        let index = graph.add_node(commands.id());

        commands.insert(Self { index });

        index
    }
}
