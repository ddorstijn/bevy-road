use bevy::prelude::*;
use opendrive::{core::OpenDrive, Interpolatable};

mod geometry;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OpenDriveRes>()
            .add_systems(Startup, load_opendrive)
            .add_systems(Update, debug_center_line);
    }
}

#[derive(Debug, Resource, Default)]
struct OpenDriveRes {
    opendrive: Option<OpenDrive>,
}

fn load_opendrive(mut opendrive: ResMut<OpenDriveRes>) {
    opendrive.opendrive = Some(opendrive::load_opendrive("C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Line-Spiral-Arc.xodr").unwrap());
}

fn debug_center_line(opendrive: Res<OpenDriveRes>, mut gizmos: Gizmos) {
    let opendrive = opendrive.opendrive.as_ref().unwrap();

    for road in opendrive.road.iter() {
        let mut s = 0.0;
        loop {
            if s > road.length {
                break;
            }

            let t = road.interpolate(s);
            gizmos.sphere(
                t.translation,
                t.rotation,
                0.1,
                Color::hsla(360.0 * s / road.length, 1.0, 0.5, s / road.length),
            );

            s += 5.0;
        }
    }
}
