use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugLinesPlugin::default())
            .add_systems(Startup, test_arc_scene)
            .add_systems(Update, update_test_scene);
    }
}

fn test_arc_scene(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<DebugLines>
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: Transform::from_translation(Vec3::new(-2.0, 0.0, -2.0)),
            ..default()
        },
        Name::new("From"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_translation(Vec3::new(2.0, 0.0, 2.0)),
            ..default()
        },
        Name::new("To"),
    ));

    lines.line(Vec3::new(-0.1, 0., 0.), Vec3::new(0.1, 0., 0.), f32::INFINITY);
    lines.line(Vec3::new(0., -0.1, 0.), Vec3::new(0., 0.1, 0.), f32::INFINITY);
    lines.line(Vec3::new(0., 0., -0.1), Vec3::new(0., 0., 0.1), f32::INFINITY);
}


fn line_line_intersection(p1: Vec3, d1: Vec3, p2: Vec3, d2: Vec3) -> Option<Vec3> {
    let direction = p2 - p1;
    let cross1 = d1.cross(d2);
    let cross2 = direction.cross(d2);

    let planar_factor = direction.dot(cross1).abs();

    //is coplanar, and not parrallel
    if planar_factor < 0.0001 && cross1.length_squared() > 0.0001 {
        let s = cross2.dot(cross1) / cross1.length_squared();
        Some(p1 + (d1 * s))
    } else {
        None
    }
}


fn generate_arc_points(start: Vec3, end: Vec3, center: Vec3, clockwise: bool) -> Vec<Vec3> {
    let radius = (start - center).length();
    let start_angle = (start.z - center.z).atan2(start.x - center.x);
    let end_angle = (end.z - center.z).atan2(end.x - center.x);

    let (start_angle, end_angle) = match clockwise {
        true if start_angle < end_angle => (start_angle + 2.0 * PI, end_angle),
        false if end_angle < start_angle => (start_angle, end_angle + 2.0 * PI),
        _ => (start_angle, end_angle),
    };

    // Calculate the total arc length
    let arc_length = radius * (end_angle - start_angle).abs();

    // Calculate the number of steps, ensuring there is approximately 1 unit arc length between points
    let num_steps = arc_length.ceil() as usize;

    // Adjust the step size based on the total arc length and number of steps
    let step = (end_angle - start_angle).abs() / num_steps as f32;

    (0..=num_steps).map(|i| {
        let angle = start_angle + step * i as f32;
        let x = center.x + radius * angle.cos();
        let y = center.y; // Assuming the arc is in the xz-plane
        let z = center.z + radius * angle.sin();
        Vec3::new(x, y, z)
    }).collect()
}


fn update_test_scene(mut query: Query<(&GlobalTransform, &mut Transform, &Name)>, mut lines: ResMut<DebugLines>,) {
    let mut q = query.iter_mut();
    let from = q.find(|(_, _, name)| name.as_str() == "From").unwrap();
    let to = q.find(|(_, _, name)| name.as_str() == "To").unwrap();
    
    let direction = to.1.translation - from.1.translation;
    let midpoint = (to.1.translation + from.1.translation) / 2.0;
    
    let normal = direction.any_orthogonal_vector().normalize();

    let intersection = line_line_intersection(from.1.translation, from.0.right(), midpoint, normal).unwrap_or_else(|| midpoint);

    let positions = generate_arc_points(from.1.translation, to.1.translation, intersection, from.0.forward().angle_between(direction).is_sign_negative());
    positions.iter().for_each(|p| {
        lines.line(intersection, *p, 0.);
    })
}