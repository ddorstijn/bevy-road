use bevy::prelude::*;
use petgraph::graph::Graph;

mod placeholder;
pub mod node;
mod edge;


#[derive(Resource, Default, Deref, DerefMut)]
pub struct RoadGraph(Graph<Entity, Entity>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoadGraph>()
            .add_plugins(placeholder::PlaceholderPlugin);
    }
}