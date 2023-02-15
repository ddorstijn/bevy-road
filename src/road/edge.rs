use crate::RoadNode;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RoadEdge {
    pub start: Entity, // With<RoadNode>
    pub end: Entity,   // With<RoadNode>

    pub lanes: u32,
    pub center: Option<Vec2>,
    pub length: f32,
}

#[derive(Bundle)]
pub struct RoadEdgeBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub edge: RoadEdge,
}

impl Default for RoadEdge {
    fn default() -> Self {
        Self {
            start: Entity {
                generation: 0,
                index: 0,
            },
            end: Entity {
                generation: 0,
                index: 0,
            },
            lanes: 0,
            center: None,
            length: 0.0,
        }
    }
}

impl RoadEdge {
    pub fn generate_mesh(self: &mut Self) -> Mesh {
        todo!();
    }
}
