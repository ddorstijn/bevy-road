use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RoadNode {
    pub lanes: u8,
}

impl Default for RoadNode {
    fn default() -> Self {
        Self { lanes: 1 }
    }
}
