use bevy::{
    ecs::system::{lifetimeless::Read, SystemParam},
    math::bounding::{Aabb3d, RayCast3d},
    prelude::*,
    render::primitives::Aabb,
    window::PrimaryWindow,
};

use crate::camera::PanOrbitCamera;

#[derive(Component)]
pub struct RaycastGroup {
    pub group: u16,
}

#[derive(SystemParam)]
pub struct Raycast<'w, 's> {
    primary_window: Query<'w, 's, Read<Window>, With<PrimaryWindow>>,
    main_camera: Query<'w, 's, (Read<Camera>, Read<GlobalTransform>), With<PanOrbitCamera>>,
    objects: Query<
        'w,
        's,
        (
            Entity,
            Read<Aabb>,
            Option<Read<RaycastGroup>>,
            Read<ViewVisibility>,
        ),
    >,
}

impl<'w, 's> Raycast<'w, 's> {
    pub fn cursor_ray(&self, filter: Option<u16>) -> Option<(Entity, Vec3)> {
        let Some(cursor_position) = self.primary_window.single().cursor_position() else {
            return None;
        };

        let (camera, camera_transform) = self.main_camera.single();
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return None;
        };

        let cast = RayCast3d::from_ray(ray, 100.0);

        // Calculate if and where the ray is hitting the ground plane.
        let Some((entity, distance)) = self
            .objects
            .iter()
            .filter(|(_, _, group, _)| {
                let Some(filter) = filter else {
                    return true;
                };

                let Some(group) = group else {
                    return false;
                };

                group.group == filter
            })
            .filter(|(_, _, _, visibility)| visibility.get())
            .map(|(entity, aabb, _group, _)| {
                (
                    entity,
                    cast.aabb_intersection_at(&Aabb3d {
                        max: Vec3::from(aabb.max()),
                        min: Vec3::from(aabb.min()),
                    }),
                )
            })
            .filter(|(_, hit)| hit.is_some())
            .map(|(entity, hit)| (entity, hit.unwrap()))
            .reduce(|min, (entity, distance)| {
                if min.1 > distance {
                    return (entity, distance);
                }

                min
            })
        else {
            return None;
        };

        Some((entity, ray.get_point(distance)))
    }
}
