use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ordered_float::OrderedFloat;

use crate::road::RoadComponent;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .init_gizmo_group::<DebugGizmos>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (draw_axis, draw_reference_line, update_hit_text, move_car),
            );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos {}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    let (my_config, _) = config_store.config_mut::<DebugGizmos>();

    my_config.depth_bias = -1.0;

    commands.spawn(TextBundle::from_section(
        "x: 0.0, y: 0.0, z: 0.0",
        TextStyle::default(),
    ));
}

fn draw_axis(mut gizmos: Gizmos<DebugGizmos>) {
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::YELLOW);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
}

fn draw_reference_line(roads: Query<&RoadComponent>, mut gizmos: Gizmos<DebugGizmos>) {
    for road in &roads {
        let positions = (0..)
            .step_by(5)
            .map(|s| (s as f32, road.0.interpolate(OrderedFloat(s as f32))))
            .take_while(|(s, _)| s <= &road.0.length)
            .map(|(_, transform)| transform.translation)
            .collect::<Vec<_>>();

        gizmos.linestrip(positions, Color::WHITE);
    }
}

fn update_hit_text(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    mut text: Query<&mut Text>,
) {
    let Some(cursor_position) = primary_window.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = main_camera.single();
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let p = ray.get_point((-ray.origin.y) / ray.direction.y);
    text.single_mut().sections[0].value = format!("x: {}, y: {}", p.x, -p.z);
}

#[derive(Default)]
struct Car {
    s: OrderedFloat<f32>,
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

    let max_length = road.iter().map(|r| r.0.length).max().unwrap_or_default();

    car.s = (car.s + 5.0 * time.delta_seconds()) % *max_length;
}
