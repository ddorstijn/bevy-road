use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};

use super::curves::arc::{self, BiArc};

#[derive(Reflect)]
pub enum EdgeType {
    Curve,
    Arc,
    Line,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RoadEdge {
    pub r#type: EdgeType,
    pub length: f32,
    pub center: Option<Vec2>,
}

impl Default for RoadEdge {
    fn default() -> Self {
        Self {
            r#type: EdgeType::Arc,
            length: 0.0,
            center: None,
        }
    }
}

fn generate_mesh() {}

// Helper functions
fn polar_to_world(center: Vec2, radius: f32, angle: f32) -> Vec2 {
    let x = center.x + angle.cos() * radius;
    let y = center.y + angle.sin() * radius;

    Vec2::new(x, y)
}

fn calculate_center(start: Vec2, end: Vec2, normal: Vec2) -> (Vec2, f32) {
    let base = (end - start).length() / 2.0;
    let angle = normal.angle_between(end - start);
    let radius = base / angle.cos();
    let center = start + normal * radius;

    (center, radius)
}

fn generate_arc(start: Vec2, end: Vec2, normal: Vec2, thickness: f32) -> Vec<Vec2> {
    // Calculate the center point of the arc
    let (center, radius) = calculate_center(start, end, normal);

    let dir_start = start - center;
    let angle_start = dir_start.y.atan2(dir_start.x);
    let dir_end = end - center;
    let angle_end = dir_end.y.atan2(dir_end.x);
    let angle_diff = angle_end - angle_start;

    let arc_length = angle_diff * radius;
    let num_points = (arc_length * 10.0).abs().ceil() as usize;
    let angle_step = angle_diff / (num_points - 1) as f32;

    (0..num_points)
        .map(|i| {
            let angle = angle_start + angle_step * i as f32;
            // Calculate the position of the vertex using the current angle
            let inner_point = polar_to_world(center, radius + thickness, angle);
            let outer_point = polar_to_world(center, radius - thickness, angle);

            [inner_point, outer_point]
        })
        .flatten()
        .collect()
}

fn generate_line(start: Vec2, end: Vec2, normal: Vec2, thickness: f32) -> Vec<Vec2> {
    let p_bl = start - normal * thickness;
    let p_br = start + normal * thickness;
    let p_tl = end - normal * thickness;
    let p_tr = end + normal * thickness;

    vec![p_bl, p_br, p_tl, p_tr]
}

fn generate_model(arc: Vec<Vec2>) -> Mesh {
    let vertices: Vec<Vec3> = arc.iter().map(|a| Vec3::new(a.x, 0.0, a.y)).collect();
    let normals: Vec<Vec3> = arc.iter().map(|_| Vec3::Y).collect();

    let levels = arc.len() / 2 - 1;
    let mut indices: Vec<u32> = Vec::with_capacity(levels * 6);
    for t in 0..levels {
        let bl = 2 * t as u32;
        let br = bl + 1;
        let tr = bl + 3;
        let tl = bl + 2;

        indices.append(&mut vec![bl, tl, tr, bl, tr, br]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    mesh
}

#[derive(Component)]
pub struct ControlPoint;
fn draw_arc(
    control_points: Query<(&Transform, &mut Handle<Mesh>), With<ControlPoint>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut iter = control_points.into_iter();

    let (t, mut m) = iter.next().unwrap();
    let biarc = BiArc::new(t, iter.next().unwrap().0);

    const DETAIL: usize = 20;

    let positions = (0..DETAIL)
        .map(|i| {
            biarc
                .interpolate(DETAIL as f32 / i as f32 * biarc.length())
                .translation
        })
        .collect::<Vec<Vec3>>();

    let normals = vec![Vec3::Y; DETAIL];

    let indices = (0..DETAIL as u32).collect::<Vec<u32>>();

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    *m = meshes.add(mesh);
}
