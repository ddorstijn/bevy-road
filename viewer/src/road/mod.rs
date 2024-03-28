use bevy::prelude::*;
use bevy_road_core::load;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_opendrive)
            .add_systems(Update, debug_center_line);
    }
}

fn load_opendrive(mut commands: Commands) {
    let project = load("C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Line-Spiral-Arc.xodr");
}

fn debug_center_line(opendrive: Res<OpenDriveRes>, mut gizmos: Gizmos) {
    let opendrive = opendrive.opendrive.as_ref().unwrap();

    // for road in opendrive.road.iter() {
    //     let pos: Vec<_> = (0..)
    //         .step_by(5)
    //         .map(|s| (s as f32, road.interpolate(s as f32)))
    //         .take_while(|(s, _)| s <= &road.length)
    //         .map(|(_, t)| t.translation)
    //         .collect();

    //     gizmos.linestrip(pos, Color::YELLOW);
    // }
}

fn create_road_mesh() {}
