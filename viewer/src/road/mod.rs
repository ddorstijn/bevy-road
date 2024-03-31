use bevy::prelude::*;
use bevy_road_core::load;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_opendrive);
    }
}

fn load_opendrive(mut commands: Commands) {
    let project = load("C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Line-Spiral-Arc.xodr");
}
