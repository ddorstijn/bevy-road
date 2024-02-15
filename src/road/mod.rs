use bevy::prelude::*;
use petgraph::graph::Graph;

use self::edge::RoadEdge;

mod placeholder;
pub mod node;
pub mod edge;


#[derive(Resource, Default, Deref, DerefMut)]
pub struct RoadGraph(Graph<Entity, Entity>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoadGraph>()
            .register_type::<RoadEdge>()
            .add_plugins(placeholder::PlaceholderPlugin);
    }
}