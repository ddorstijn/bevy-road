use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

#[derive(Component, Debug)]
pub struct Waypoint {
    pub position: Vec2,
    pub normal: Vec2,
    pub group: &WaypointGroup,
}

#[derive(Component, Debug)]
pub struct Connection {
    pub start: &Waypoint,
    pub end: &Waypoint,
    pub center: Option<Vec2>,
    pub length: f32,
}

#[derive(Component, Debug)]
struct WaypointGroup {
    position: Vec3,
    normal: f32,
    waypoints: vec<&Waypoint>,
}

trait Traversable {
    fn traverse(self: &Self, road_segment: &RoadSegment, progress: f32) -> Vec2;
}

impl RoadSegment {
    pub fn generate_segment(self: &mut Self) -> Mesh {
        if (self.end - self.start).length() == 0. {
            // TODO: throw error
            panic!("Length of the road is 0");
        }

        todo!();
    }
}

impl Traversable for CurvedRoadSegment {
    fn traverse(self: &Self, progress: f32) -> Vec2 {
        return Vec2::X;
    }
}

impl Traversable for StraightRoadSegment {
    fn traverse(self: &Self, progress: f32) -> Vec2 {}
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
    println!("{}", angle_start);
    let dir_end = end - center;
    let angle_end = dir_end.y.atan2(dir_end.x);
    let angle_diff = angle_end - angle_start;

    let arc_length = angle_diff * radius;
    let num_points = (arc_length * DETAIL as f32).abs().ceil() as usize;
    let angle_step = angle_diff / (num_points - 1) as f32;

    (0..num_points)
        .map(|i| {
            let angle = angle_start + angle_step * i as f32;
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
