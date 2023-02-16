use bevy::prelude::*;
use petgraph::stable_graph::EdgeIndex;

pub struct EdgeWeight {
    pub lanes: u8,
    pub length: f32,
    pub center: Option<Vec2>,
}

#[derive(Component, Default)]
pub struct RoadEdge(EdgeIndex);

#[derive(Bundle, Default)]
pub struct RoadEdgeBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub edge: RoadEdge,
}
