use bevy::prelude::*;

#[derive(Reflect)]
pub enum EdgeType {
    Connection,
    Transition,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RoadEdge {
    pub r#type: EdgeType,
    pub length: f32,
    pub center: Option<Vec2>,
}

impl Default for RoadEdge {
    fn default() -> Self {
        Self {
            r#type: EdgeType::Connection,
            length: 0.0,
            center: None,
        }
    }
}
