use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::camera::PanOrbitCamera;

pub fn cast_ray_from_cursor(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    filter: QueryFilter,
) -> Option<(Entity, Vec3)> {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    else {
        return None;
    };

    let Some((entity, toi)) =
        rapier_context.cast_ray(ray.origin, ray.direction.into(), 1000.0, true, filter)
    else {
        return None;
    };

    Some((entity, ray.origin + ray.direction * toi))
}
