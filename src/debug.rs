use std::panic;

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


fn intersection(point1: Vec2, dir1: Vec2, point2: Vec2, dir2: Vec2) -> Option<Vec2> {
    let mat_a = Mat2::from_cols(dir1, dir2 * -1.0);
    
    if let Ok(inverse) = panic::catch_unwind(|| mat_a.inverse()) { 
        let rhs = point2 - point1;
        let result = inverse * rhs;

        Some(point1 + dir1 * result.x)
    } else {
        None
    }
}

fn test_arc_scene(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<DebugLines>
) {

    let to = Vec3::new(2.0, 0.0, 2.0);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        },
        Name::new("From"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_translation(to),
            ..default()
        },
        Name::new("To"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
            material: materials.add(Color::rgb(0., 0., 1.).into()),
            ..default()
        },
        Name::new("Projection"),
    ));

    lines.line(Vec3::new(-1., 0., 0.), Vec3::new(1., 0., 0.), f32::INFINITY);
    lines.line(Vec3::new(0., -1., 0.), Vec3::new(0., 1., 0.), f32::INFINITY);
    lines.line(Vec3::new(0., 0., -1.), Vec3::new(0., 0., 1.), f32::INFINITY);
}

fn update_test_scene(mut query: Query<(&GlobalTransform, &mut Transform, &Name)>, mut lines: ResMut<DebugLines>) {
    let mut q = query.iter_mut();
    let from = q.find(|(_, _, name)| name.as_str() == "From").unwrap();
    let to = q.find(|(_, _, name)| name.as_str() == "To").unwrap();
    let mut projection = q.find(|(_, _, name)| name.as_str() == "Projection").unwrap();
    
    let midpoint = (to.1.translation + from.1.translation) / 2.0;
    let direction = to.1.translation - from.1.translation;
    
    let normal = direction.any_orthogonal_vector().normalize().xz();

    let intersection = intersection(from.1.translation.xz(), from.0.right().xz(), midpoint.xz(), normal).unwrap_or_else(|| midpoint.xz());
    projection.1.translation = Vec3::new(intersection.x, 0., intersection.y);

    // Show right direction
    lines.line_colored(from.1.translation, from.1.translation + from.0.right() * 10., 0., Color::PINK);

    // Show line from -> to
    lines.line_colored(from.1.translation, to.1.translation, 0., Color::CRIMSON);

    // Show bisector
    lines.line_colored(midpoint + Vec3::new(normal.x, 0., normal.y) * -5., midpoint + Vec3::new(normal.x, 0., normal.y) * 5., 0., Color::AQUAMARINE);
}