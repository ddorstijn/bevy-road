use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    flycam::PanOrbitCamera,
    road::{RoadEnd, SelectedNode},
};

pub fn drag_road(
    rapier: Res<RapierContext>,
    camera: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected_node: ResMut<SelectedNode>,
    mut transform: Query<&mut Transform, With<RoadEnd>>,
) {
    // Drop selected node
    if selected_node.node.is_some() && mouse_button.just_pressed(MouseButton::Left) {
        selected_node.node = None;
        return;
    }

    // Do nothing if not selected and not selecting anything
    if selected_node.node.is_none() && !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Build ray from screenspace
    let cursor_pos = match windows.get_primary().unwrap().cursor_position() {
        Some(pos) => pos,
        None => {
            return;
        }
    };

    let (cam, cam_transform) = camera.get_single().unwrap();
    let ray = cam.viewport_to_world(cam_transform, cursor_pos).unwrap();

    let filter = match selected_node.node {
        Some(_) => QueryFilter::exclude_dynamic().groups(CollisionGroups::new(
            Group::GROUP_2.into(),
            Group::GROUP_2.into(),
        )),
        None => QueryFilter::exclude_dynamic().groups(CollisionGroups::new(
            Group::GROUP_1.into(),
            Group::GROUP_1.into(),
        )),
    };

    if let Some((entity, toi)) = rapier.cast_ray(ray.origin, ray.direction, f32::MAX, false, filter)
    {
        match selected_node.node {
            Some(x) => {
                let hit_point = ray.origin + ray.direction * toi;
                transform.get_mut(x).unwrap().translation = hit_point;
                println!("{}", hit_point);
            }
            None => {
                selected_node.node = Some(entity);
            }
        }
    }
}
