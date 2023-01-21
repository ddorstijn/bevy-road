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
            start: Vec3::new(1.0, 0., 1.0),
            end: Vec3::new(2.0, 0., 2.),
            tangent: Vec2::new(1., 1.).normalize(),
            road_type: SegmentType::Line,
        }
    }
}

const DETAIL: usize = 10;
const THICKNESS: f32 = 2.;

fn generate_point_on_circle(center: Vec2, radius: f32, angle: f32) -> Vec3 {
    let x = center.x + angle.cos() * radius;
    let y = center.y + angle.sin() * radius;
    Vec3::new(x, 0.0, y)
}

fn calculate_center(start: Vec2, end: Vec2, tangent: Vec2) -> Vec2 {
    // Calculate the midpoint of the start and end points
    let midpoint = (start + end) / 2.0;
    // Calculate the normal of the tangent vector using the perp method
    let normal = tangent.perp();
    // Calculate the base length of isosceles triangle
    let base = (end - start).length() / 2.0;
    // Calculate the angle of the triangle
    let angle = tangent.angle_between(end - start) / 2.0;
    // Calculate the hypotenuse of the triangle using cosine and base length
    let hypotenuse = base * angle.cos();
    // Calculate the center point of the arc using the normal and hypotenuse
    let center = midpoint + normal * hypotenuse;
    center
}

fn generate_arc(start: Vec2, end: Vec2, tangent: Vec2) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::with_capacity(DETAIL * 2);
    let mut normals: Vec<Vec3> = Vec::with_capacity(DETAIL * 2);
    let mut indices: Vec<u32> = Vec::with_capacity(DETAIL * 6);

    // calculate the center point of the arc
    let center = calculate_center(start, end, tangent);
    // calculate radius of the arc
    let radius = (end - center).length();
    // Calculate the angle of the arc
    let angle = (end - center).length() / radius;
    // Calculate the angle increment for each segment
    let angle_inc = angle / segments as f32;
    // Calculate the initial rotation angle
    let angle_start = tangent.angle_between(end - start);

    let mut current_angle = angle_start;
    for _ in 0..segments {
        // Calculate the position of the vertex using the current angle
        let x = center.x + radius * current_angle.cos();
        let y = center.y + radius * current_angle.sin();
        let position = Vec3::new(x, y, 0.0);
        // Add the vertex to the list of vertices
        vertices.push(position);
        // Rotate the angle by the angle increment
        current_angle += angle_inc;
    }

    (vertices, normals, indices)
}

pub fn generate_segment(s: &mut RoadSegment) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let start = Vec2::new(s.start.x, s.start.z);
    let end = Vec2::new(s.end.x, s.end.z);

    let is_straight = s.tangent.angle_between((start - end) - start).abs() < 0.005;

    s.road_type = match is_straight {
        true => SegmentType::Line,
        false => SegmentType::Arc,
    };

    let (vertices, normals, indices) = match s.road_type {
        SegmentType::Line => generate_line(start, end, s.tangent),
        SegmentType::Arc => generate_arc(start, end, s.tangent),
    };

    // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    // mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh
}

fn generate_line(start: Vec2, end: Vec2, tangent: Vec2) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::with_capacity(4);
    let mut normals: Vec<Vec3> = Vec::with_capacity(4);
    let mut indices: Vec<u32> = Vec::with_capacity(6);

    let normal = tangent.perp();

    let p_bl = start + normal * THICKNESS;
    let pos_bl = Vec3::new(p_bl.x, 0.0, p_bl.y);
    let p_br = start - normal * THICKNESS;
    let pos_br = Vec3::new(p_br.x, 0.0, p_br.y);
    let p_tl = end + normal * THICKNESS;
    let pos_tl = Vec3::new(p_tl.x, 0.0, p_tl.y);
    let p_tr = end - normal * THICKNESS;
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
                    println!("Center");
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
