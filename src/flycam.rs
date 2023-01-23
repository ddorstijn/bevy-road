use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub speed: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            speed: 3.0,
        }
    }
}

pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
    time: Res<Time>,
) {
    let (mut pan_orbit, mut transform) = query
        .get_single_mut()
        .expect("More than one Player Camera in scene");

    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut has_moved = false;

    // Orbit
    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }

    // Zoom
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    // Pan
    let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();
    let left = Vec3::new(transform.left().x, 0.0, transform.left().z).normalize();
    let speed = pan_orbit.speed;

    if keyboard_input.pressed(KeyCode::A) {
        has_moved = true;
        pan_orbit.focus += left * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        has_moved = true;
        pan_orbit.focus -= left * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        has_moved = true;
        pan_orbit.focus -= forward * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::W) {
        has_moved = true;
        pan_orbit.focus += forward * time.delta_seconds() * speed;
    }

    if rotation_move.length_squared() > 0.0 {
        has_moved = true;
        let window = get_primary_window_size(&windows);
        let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
        let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
        transform.rotation = transform.rotation * pitch; // rotate around local x axis
    } else if scroll.abs() > 0.0 {
        has_moved = true;
        pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
        // dont allow zoom to reach zero or you get stuck
        pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
    }

    if has_moved {
        // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
        // parent = x and y rotation
        // child = z-offset
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
