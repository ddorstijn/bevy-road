use bevy::prelude::*;

use self::{edge::RoadEdge, shader::RoadShaderPlugin, world::RoadGridPlugin};

pub mod biarc;
pub mod edge;
pub mod placeholder;
pub mod shader;
pub mod world;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadEdge>()
            .add_plugins(placeholder::PlaceholderPlugin)
            .add_plugins(RoadShaderPlugin)
            .add_plugins(RoadGridPlugin);
    }
}

#[derive(Component)]
pub struct RoadSpawner;
