use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

#[derive(Debug, Reflect)]
pub enum SegmentType {
    Line,
    Arc,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RoadSegment {
    pub start: Vec3,
    pub end: Vec3,
    pub tangent: Vec2,
    pub road_type: SegmentType,
}

impl Default for RoadSegment {
    fn default() -> Self {
        Self {
            start: Vec3::new(0.0, 0.0, 0.0),
            end: Vec3::new(2.0, 0., 1.),
            tangent: Vec2::new(0., 1.).normalize(),
            road_type: SegmentType::Arc,
        }
    }
}

const DETAIL: usize = 10;
const THICKNESS: f32 = 0.25;

fn generate_point_on_circle(center: Vec2, radius: f32, angle: f32) -> Vec3 {
    let x = center.x + angle.cos() * radius;
    let y = center.y + angle.sin() * radius;
    Vec3::new(x, 0.0, y)
}

fn calculate_center(start: Vec2, end: Vec2, normal: Vec2) -> (Vec2, f32) {
    let base = (end - start).length() / 2.0;
    let angle = normal.angle_between(end - start);
    let radius = base / angle.cos();
    let center = start + normal * radius;

    (center, radius)
}

fn generate_arc(start: Vec2, end: Vec2, normal: Vec2) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::with_capacity(DETAIL * 2);
    let mut normals: Vec<Vec3> = Vec::with_capacity(DETAIL * 2);
    let mut indices: Vec<u32> = Vec::with_capacity(DETAIL * 6);

    // Calculate the center point of the arc
    let (center, radius) = calculate_center(start, end, normal);
    let angle_start = (start - center).angle_between(center);
    let angle_end = (end - center).angle_between(center);
    let angle_diff = angle_end - angle_start;
    let angle_step = angle_diff / (DETAIL - 1) as f32;

    for i in 0..DETAIL {
        let angle = angle_start - angle_step * i as f32;

        // Calculate the position of the vertex using the current angle
        let inner_point = generate_point_on_circle(center, radius + THICKNESS, angle);
        let outer_point = generate_point_on_circle(center, radius - THICKNESS, angle);

        // Add the vertex to the list of vertices
        vertices.push(inner_point);
        vertices.push(outer_point);

        normals.push(Vec3::Y);
        normals.push(Vec3::Y);

        if i < DETAIL - 1 {
            let bl = 2 * i as u32;
            let br = bl + 1;
            let tr = bl + 3;
            let tl = bl + 2;

            indices.append(&mut vec![bl, tl, tr, bl, tr, br]);
        }
    }

    (vertices, normals, indices)
}

fn generate_line(start: Vec2, end: Vec2, normal: Vec2) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::with_capacity(4);
    let mut normals: Vec<Vec3> = Vec::with_capacity(4);
    let mut indices: Vec<u32> = Vec::with_capacity(6);

    let p_bl = start - normal * THICKNESS;
    let pos_bl = Vec3::new(p_bl.x, 0.0, p_bl.y);
    let p_br = start + normal * THICKNESS;
    let pos_br = Vec3::new(p_br.x, 0.0, p_br.y);
    let p_tl = end - normal * THICKNESS;
    let pos_tl = Vec3::new(p_tl.x, 0.0, p_tl.y);
    let p_tr = end + normal * THICKNESS;
    let pos_tr = Vec3::new(p_tr.x, 0.0, p_tr.y);

    vertices.append(&mut vec![pos_bl, pos_br, pos_tl, pos_tr]);
    normals.append(&mut vec![Vec3::Y; 4]);

    let bl = 0;
    let br = bl + 1;
    let tr = bl + 3;
    let tl = bl + 2;

    indices.append(&mut vec![bl, tl, tr, bl, tr, br]);

    (vertices, normals, indices)
}

pub fn generate_segment(s: &mut RoadSegment) -> Mesh {
    let start = Vec2::new(s.start.x, s.start.z);
    let end = Vec2::new(s.end.x, s.end.z);

    s.road_type = match s.tangent.angle_between(end - start).abs() < 0.005 {
        true => SegmentType::Line,
        false => SegmentType::Arc,
    };

    let (vertices, normals, indices) = match s.road_type {
        SegmentType::Line => generate_line(start, end, -1. * s.tangent.perp()),
        SegmentType::Arc => generate_arc(start, end, -1. * s.tangent.perp()),
    };

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    mesh
}

pub fn update_dirty(
    mut segments: Query<(&mut Handle<Mesh>, &mut RoadSegment, &Children), Changed<RoadSegment>>,
    mut gizmos: Query<(&mut Transform, &Name)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut mesh, mut segment, children) in &mut segments {
        *mesh = meshes.add(generate_segment(&mut segment));

        for &child in children.iter() {
            let (mut transform, name) = gizmos.get_mut(child).unwrap();
            match name.as_str() {
                "Center" => {
                    let start = Vec2::new(segment.start.x, segment.start.z);
                    let end = Vec2::new(segment.end.x, segment.end.z);
                    let (center, _) = calculate_center(start, end, -1. * segment.tangent.perp());
                    transform.translation = Vec3::new(center.x, 0.0, center.y);
                }
                "Start" => {
                    transform.translation = segment.start;
                }
                "End" => {
                    transform.translation = segment.end;
                }
                _ => {
                    println!("Not found");
                }
            }
        }
    }
}
