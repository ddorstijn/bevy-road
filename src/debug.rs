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

fn intersection(point1: Vec2, dir1: Vec2, point2: Vec2, dir2: Vec2) -> Option<Vec2> {
    let mat_a = Mat2::from_cols(dir1, dir2 * -1.0);
    
    if let Ok(inverse) = std::panic::catch_unwind(|| mat_a.inverse()) { 
        let rhs = point2 - point1;
        let result = inverse * rhs;

        Some(point1 + dir1 * result.x)
    } else {
        None
    }
}

fn generate_arc_points(start: Vec2, end: Vec2, center: Vec2, clockwise: bool) -> Vec<Vec2> {
    let radius = (start - center).length();
    let mut start_angle = (start.y - center.y).atan2(start.x - center.x);
    let mut end_angle = (end.y - center.y).atan2(end.x - center.x);

    if clockwise {
        if start_angle < end_angle {
            start_angle += 2.0 * std::f32::consts::PI;
        }
    } else {
        if end_angle < start_angle {
            end_angle += 2.0 * std::f32::consts::PI;
        }
    }

    let mut points = Vec::new();

    // We'll generate points every radian along the arc
    let step = if clockwise { -0.01 } else { 0.01 };
    let mut angle = start_angle;
    while (clockwise && angle >= end_angle) || (!clockwise && angle <= end_angle) {
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec2::new(x, y));
        angle += step;
    }

    points
}


fn update_test_scene(mut query: Query<(&GlobalTransform, &mut Transform, &Name)>, mut lines: ResMut<DebugLines>,) {
    let mut q = query.iter_mut();
    let from = q.find(|(_, _, name)| name.as_str() == "From").unwrap();
    let to = q.find(|(_, _, name)| name.as_str() == "To").unwrap();
    
    let direction = to.1.translation - from.1.translation;
    let midpoint = (to.1.translation + from.1.translation) / 2.0;
    
    let normal = direction.any_orthogonal_vector().normalize();

    let intersection = intersection(from.1.translation.xz(), from.0.right().xz(), midpoint.xz(), normal.xz()).unwrap_or_else(|| midpoint.xz());
    
    let positions = generate_arc_points(from.1.translation.xz(), to.1.translation.xz(), intersection, from.0.forward().xz().angle_between(direction.xz()).is_sign_negative());
    positions.iter().for_each(|p| {
        lines.line(intersection.extend(0.).xzy(), p.extend(0.0).xzy(), 0.);
    })
}