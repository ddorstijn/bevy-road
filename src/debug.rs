use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::road::edge::RoadEdge;
use crate::road::placeholder::RoadPlaceholder;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, debug_edges)
            .add_systems(Update, draw_axis)
            .add_systems(Update, debug_road_ends);
    }
}

fn debug_edges(
    edge_query: Query<(&GlobalTransform, &RoadEdge), With<RoadPlaceholder>>,
    mut gizmos: Gizmos,
) {
    for (transform, edge) in edge_query.iter() {
        // gizmos.line(transform.translation(), transform.translation() + transform.right(), Color::WHITE);
        gizmos.line(
            transform.translation(),
            transform.translation() + transform.forward(),
            Color::BLACK,
        );

        let center = transform.translation() + transform.left() * edge.radius();

        let rot = Quat::from_axis_angle(Vec3::Y, 0.25 * PI);
        let mut point = transform.translation() - center;

        gizmos.sphere(center, Quat::IDENTITY, 0.1, Color::BLUE);
        // gizmos.sphere(center, Quat::IDENTITY, edge.radius(), Color::BLUE);
        gizmos.arc_3d(
            edge.angle(),
            edge.radius(),
            edge.center(),
            Quat::from_rotation_y(edge.rotation().to_angle()),
            Color::NAVY,
        );

        gizmos.line(edge.start.translation, edge.end.translation, Color::YELLOW);

        gizmos.ray(
            edge.center(),
            edge.rotation().extend(0.0).xzy(),
            Color::PINK,
        );

        point = rot.mul_vec3(point);

        gizmos.ray(center, point, Color::GREEN);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::LIME_GREEN);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::YELLOW);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::ORANGE);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::RED);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::PURPLE);

        point = rot.mul_vec3(point);
        gizmos.ray(center, point, Color::BLUE);
    }
}

// fn debug_aabb(aabbs: Query<(&Aabb, &GlobalTransform)>, mut gizmos: Gizmos) {
//     for (aabb, transform) in aabbs.iter() {
//         gizmos.cuboid(transform.compute_transform(), Color::WHITE).;
//     }
// }

fn debug_road_ends(
    query: Query<(&RoadEdge, &GlobalTransform), Without<RoadPlaceholder>>,
    mut gizmos: Gizmos,
) {
    for (edge, transform) in query.into_iter() {
        let end = transform.mul_transform(edge.get_end_transform(None));
        gizmos.ray(end.translation(), end.forward(), Color::YELLOW_GREEN);
    }
}

fn draw_axis(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
}
