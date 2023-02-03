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
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
    pub road_type: SegmentType,
}

impl Default for RoadSegment {
    fn default() -> Self {
        Self {
            start: Vec2::new(0.0, 0.0),
            end: Vec2::new(2.0, 1.),
            normal: Vec2::new(0., 1.).normalize(),
            road_type: SegmentType::Arc,
        }
    }
}

impl RoadSegment {
    pub fn generate_segment(self: &mut Self) -> Mesh {
        if (self.end - self.start).length() == 0. {
            // TODO: throw error
            panic!("Length of the road is 0");
        }

        self.road_type = match is_straight(self.normal.perp().angle_between(self.end - self.start))
        {
            true => SegmentType::Line,
            false => SegmentType::Arc,
        };

        let arc = match self.road_type {
            SegmentType::Line => generate_line(self.start, self.end, -1.0 * self.normal, THICKNESS),
            SegmentType::Arc => generate_arc(self.start, self.end, -1.0 * self.normal, THICKNESS),
        };

        generate_model(arc)
    }
}

#[derive(Component, Reflect)]
pub struct RoadEnd;

pub fn regenerate_mesh(
    mut segments: Query<(&mut Handle<Mesh>, &mut RoadSegment, &Children), Changed<RoadSegment>>,
    mut gizmos: Query<(&mut Transform, With<RoadEnd>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut mesh, mut segment, children) in &mut segments {
        *mesh = meshes.add(segment.generate_segment());

        for &child in children.iter() {
            let (mut transform, _) = gizmos
                .get_mut(child)
                .expect("No child with component of type RoadEnd");
            transform.translation = Vec3::new(segment.end.x, 0.0, segment.end.y);
        }
    }
}

const DETAIL: usize = 5;
const TOLERANCE: f32 = 0.05;
const THICKNESS: f32 = 0.25;

fn is_straight(angle: f32) -> bool {
    // Calculate the difference between the angle and a half circle
    angle.abs() < TOLERANCE || (angle.abs() - std::f32::consts::PI).abs() < TOLERANCE
}

fn generate_point_on_circle(center: Vec2, radius: f32, angle: f32) -> Vec2 {
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
    let num_points = (arc_length * DETAIL as f32).abs().ceil() as usize;
    let angle_step = angle_diff / (num_points - 1) as f32;

    (0..num_points)
        .map(|i| {
            let angle = angle_start - angle_step * i as f32;
            // Calculate the position of the vertex using the current angle
            let inner_point = generate_point_on_circle(center, radius + thickness, angle);
            let outer_point = generate_point_on_circle(center, radius - thickness, angle);

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
