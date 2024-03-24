use bevy::{math::bounding::Aabb3d, prelude::*};

use crate::states::GameState;

use self::{
    placeholder::{BuildSystemSet, RoadPlaceholder},
    world::{RoadGridPlugin, WorldSystemSet, WorldTile},
};

pub mod biarc;
pub mod edge;
pub mod placeholder;
pub mod world;

pub mod arc;
pub mod collision;
pub mod line;

pub const ROAD_WIDTH: f32 = 1.0;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorldTile>()
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

pub trait RoadEdge {
    fn interpolate(&self, length: f32, lane_offset: f32) -> Transform;
    fn intersects_point(&self, point: Vec2) -> bool;
    fn coord_to_length(&self, coord: Vec2) -> f32;

    fn resize(&mut self, length: f32);

    fn aabb3(&self) -> Aabb3d;
    fn length(&self) -> f32;
    fn lanes(&self) -> u8;
}
