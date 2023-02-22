use bevy::prelude::*;

#[derive(Reflect)]
pub enum NodeType {
    Incomming,
    Outgoing,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RoadNode {
    pub r#type: NodeType,
}

impl Default for RoadNode {
    fn default() -> Self {
        Self {
            r#type: NodeType::Outgoing,
        }
    }
}
