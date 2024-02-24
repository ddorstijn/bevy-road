use bevy::{
    ecs::system::{lifetimeless::Read, SystemParam},
    math::bounding::{Aabb3d, RayCast3d},
    prelude::*,
    render::primitives::Aabb,
    window::PrimaryWindow,
};

use crate::camera::PanOrbitCamera;

#[derive(SystemParam)]
pub struct Raycast<'w, 's, T: bevy::prelude::Component> {
    primary_window: Query<'w, 's, Read<Window>, With<PrimaryWindow>>,
    main_camera: Query<'w, 's, (Read<Camera>, Read<GlobalTransform>), With<PanOrbitCamera>>,
    objects: Query<
        'w,
        's,
        (
            Entity,
            Read<Aabb>,
            Read<GlobalTransform>,
            Read<ViewVisibility>,
        ),
        With<T>,
    >,
}

impl<'w, 's, T: bevy::prelude::Component> Raycast<'w, 's, T> {
    pub fn cursor_ray_intersections(&self) -> Vec<(Entity, Vec3)> {
        let Some(cursor_position) = self.primary_window.single().cursor_position() else {
            return Vec::new();
        };

        let (camera, camera_transform) = self.main_camera.single();
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return Vec::new();
        };

        // Calculate if and where the ray is hitting the ground plane.
        self.objects
            .iter()
            .filter(|(_, _, _, visibility)| visibility.get())
            .map(|(entity, aabb, transform, _)| {
                let world_to_model = transform.compute_matrix().inverse();
                let origin = world_to_model.transform_point(ray.origin);
                let dir = Direction3d::new(world_to_model.transform_vector3(ray.direction.into()))
                    .unwrap();
                let cast = RayCast3d::new(origin, dir, 100.0);
                (
                    entity,
                    cast.aabb_intersection_at(&Aabb3d {
                        max: Vec3::from(aabb.max()),
                        min: Vec3::from(aabb.min()),
                    }),
                )
            })
            .filter(|(_, hit)| hit.is_some())
            .map(|(entity, hit)| (entity, ray.get_point(hit.unwrap())))
            .collect()
    }

    pub fn cursor_ray(&self) -> Option<(Entity, Vec3)> {
        self.cursor_ray_intersections()
            .into_iter()
            .reduce(|min, (entity, hitpoint)| {
                let current_distance = hitpoint.length_squared();
                let minimum_distance = min.1.length_squared();

                if minimum_distance > current_distance {
                    return (entity, hitpoint);
                }

                min
            })
    }
}
