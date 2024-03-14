use bevy::prelude::*;

use crate::states::GameState;

use self::{
    edge::RoadEdge,
    placeholder::{BuildSystemSet, RoadPlaceholder},
    world::{RoadGridPlugin, WorldSystemSet, WorldTile},
};

pub mod biarc;
pub mod edge;
pub mod placeholder;
pub mod world;

pub const ROAD_WIDTH: f32 = 1.0;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadEdge>()
            .register_type::<WorldTile>()
            .add_plugins((RoadGridPlugin, placeholder::PlaceholderPlugin))
            .configure_sets(
                Update,
                (
                    BuildSystemSet::Building.run_if(any_with_component::<RoadPlaceholder>),
                    BuildSystemSet::NotBuilding.run_if(not(any_with_component::<RoadPlaceholder>)),
                    WorldSystemSet,
                )
                    .chain()
                    .run_if(in_state(GameState::Building)),
            )
            .configure_sets(OnEnter(GameState::Building), BuildSystemSet::EnterBuildMode)
            .configure_sets(OnExit(GameState::Building), BuildSystemSet::ExitBuildMode);
    }
}

#[derive(Component)]
pub struct RoadSpawner;
