use bevy::prelude::*;
use petgraph::graph::NodeIndex;

pub struct NodeWeight {
    pub group: Entity,
    pub lanes: u8,
}

#[derive(Component, Default)]
pub struct RoadNode(pub NodeIndex);
