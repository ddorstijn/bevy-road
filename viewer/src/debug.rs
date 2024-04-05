use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::road::RoadComponent;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .init_gizmo_group::<DebugGizmos>()
            .add_systems(Startup, setup_gizmos)
            .add_systems(Update, (draw_axis, draw_reference_line, move_car));
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos {}

fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (my_config, _) = config_store.config_mut::<DebugGizmos>();

    my_config.depth_bias = -1.0;
}

fn draw_axis(mut gizmos: Gizmos<DebugGizmos>) {
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
}

fn draw_reference_line(roads: Query<&RoadComponent>, mut gizmos: Gizmos<DebugGizmos>) {
    for road in &roads {
        let positions = (0..)
            .step_by(5)
            .map(|s| (s as f32, road.0.interpolate(s as f32)))
            .take_while(|(s, _)| s <= &road.0.length)
            .map(|(_, transform)| transform.translation)
            .collect::<Vec<_>>();

        gizmos.linestrip(positions, Color::WHITE);
    }
}

#[derive(Default)]
struct Car {
    s: f32,
}

fn move_car(
    mut gizmos: Gizmos<DebugGizmos>,
    mut car: Local<Car>,
    road: Query<&RoadComponent>,
    time: Res<Time>,
) {
    for road in &road {
        let road = &road.0;
        let transform = road.interpolate(car.s);

        gizmos.arrow(
            transform.translation - *transform.forward() * 5.0,
            transform.translation + *transform.forward() * 5.0,
            Color::YELLOW,
        );
    }

    let max_length = road.iter().map(|r| r.0.length).max().unwrap();

    car.s = (car.s + 5.0 * time.delta_seconds()) % *max_length;
}
