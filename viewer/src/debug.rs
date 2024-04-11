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
                (
                    draw_axis,
                    draw_reference_line,
                    update_hit_text,
                    debug_heading,
                    move_car,
                ),
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
        let steps = road.0.length.ceil() * 10.0;
        let step_size = road.0.length / steps;
        let positions = (0..=steps as u32)
            .map(|step| {
                let (x, neg_z, y, _) = road.0.interpolate(step_size * step as f32);
                Vec3::new(x, y, -neg_z)
            })
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

fn debug_heading(mut gizmos: Gizmos<DebugGizmos>, road: Query<&RoadComponent>) {
    for road in &road {
        let road = &road.0;

        for rl in road.reference_line.values() {
            let (x, neg_z, hdg) = rl.interpolate(rl.length);
            let start = Vec3::new(x, 0.0, -neg_z);
            let end = start + Vec3::new(hdg.cos(), 0.0, -hdg.sin());
            gizmos.arrow(start, end, Color::CYAN);
        }
    }
}

fn move_car(mut gizmos: Gizmos<DebugGizmos>, road: Query<&RoadComponent>, time: Res<Time>) {
    for road in &road {
        let road = &road.0;
        let (x, neg_z, y, hdg) =
            road.interpolate(OrderedFloat(2.0 * time.elapsed_seconds_wrapped()) % road.length);

        let p = Vec3::new(x, y, -neg_z);
        let h = Vec3::new(hdg.cos(), 0.0, -hdg.sin());
        gizmos.arrow(p, p + h * 5.0, Color::YELLOW);
    }
}
