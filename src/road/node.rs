use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoadNode;

#[derive(Bundle, Default)]
pub struct RoadNodeBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub node: RoadNode,
}

impl RoadNode {
    fn generate_mesh() {
        todo!();
    }
}
