use bevy::prelude::*;
use petgraph::stable_graph::EdgeIndex;

pub enum EdgeType {
    Connection,
    Transition,
}

pub struct EdgeWeight {
    pub r#type: EdgeType,
    pub length: f32,
    pub center: Option<Vec2>,
}

#[derive(Component, Default)]
pub struct RoadEdge(pub EdgeIndex);
