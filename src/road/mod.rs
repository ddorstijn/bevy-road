use bevy::prelude::*;

mod builder;
mod connection;

use self::builder::{
    drag_node, drop_node, mark_node_end, run_if_dragging, run_if_selecting, select_node,
    set_road_end_mesh, RoadEndModel,
};

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadEndModel>()
            .add_startup_system(set_road_end_mesh)
            .add_system(mark_node_end)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_dragging)
                    .with_system(drop_node)
                    .with_system(drag_node),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_selecting)
                    .with_system(select_node),
            );
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RoadNode {
    pub lanes: u32,
    pub connections: Vec<Entity>,
}

impl Default for RoadNode {
    fn default() -> Self {
        Self {
            lanes: 1,
            connections: vec![],
        }
    }
}

#[derive(Component, Debug)]
struct RoadSection {}

#[derive(Bundle)]
pub struct RoadBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub node: RoadNode,
}

impl Default for RoadBundle {
    fn default() -> Self {
        Self {
            pbr: PbrBundle::default(),
            node: RoadNode::default(),
        }
    }
}
