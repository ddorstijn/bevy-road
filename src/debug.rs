use std::f32::consts::PI;

use bevy::math::bounding::BoundingVolume;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::road::edge::RoadEdge;
use crate::road::placeholder::RoadPlaceholder;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .init_gizmo_group::<DebugGizmos>()
            .add_systems(Startup, setup_gizmos)
            .add_systems(Update, debug_edges)
            .add_systems(Update, draw_axis)
            .add_systems(Update, (debug_aabb, debug_edges_aabb))
            .add_systems(Update, debug_road_ends);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos {}

fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (my_config, _) = config_store.config_mut::<DebugGizmos>();

    my_config.depth_bias = -1.0;
}

fn debug_edges(
    edge_query: Query<(&GlobalTransform, &RoadEdge), With<RoadPlaceholder>>,
    mut gizmos: Gizmos<DebugGizmos>,
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
        gizmos.sphere(center, Quat::IDENTITY, edge.radius(), Color::BLUE);

        gizmos.line(
            edge.start().translation,
            edge.end().translation,
            Color::YELLOW,
        );

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

fn debug_aabb(aabbs: Query<&Aabb>, mut gizmos: Gizmos) {
    for aabb in aabbs.iter() {
        let transform = Transform::from_translation(aabb.center.into())
            .with_scale(2.0 * Vec3::from(aabb.half_extents));
        gizmos.cuboid(transform, Color::GREEN);
    }
}

fn debug_edges_aabb(edges: Query<&RoadEdge>, mut gizmos: Gizmos) {
    for edge in &edges {
        let aabb = edge.aabb3();
        let transform = Transform::from_translation(aabb.center().into())
            .with_scale(2.0 * Vec3::from(aabb.half_size()));

        gizmos.cuboid(transform, Color::GREEN);
    }
}

fn debug_road_ends(
    query: Query<&RoadEdge, Without<RoadPlaceholder>>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    for edge in query.into_iter() {
        let end = edge.end();
        gizmos.ray(end.translation, *end.forward(), Color::YELLOW_GREEN);
    }
}

fn draw_axis(mut gizmos: Gizmos<DebugGizmos>) {
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
}
