use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use itertools::Itertools;

use super::node::RoadNode;

const DETAIL: u32 = 10;

#[derive(Component, Default)]
pub struct RoadNodeGroup;

// Systems
pub fn generate_intersection_mesh(
    mut groups: Query<(&mut Handle<Mesh>, &Children), Added<RoadNodeGroup>>,
    nodes: Query<(&Transform, &RoadNode)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut mesh_handle, children) in &mut groups {
        let sorted_nodes = children
            .iter()
            .filter_map(|c| nodes.get(*c).ok())
            .sorted_by(|(a, _), (b, _)| {
                a.translation
                    .angle_between(Vec3::X)
                    .partial_cmp(&b.translation.angle_between(Vec3::X))
                    .unwrap()
            })
            .collect::<Vec<(&Transform, &RoadNode)>>();

        println!(
            "{}",
            get_point_on_bezier(sorted_nodes[0].0, sorted_nodes[1].0, 1.0),
        );

        let positions = (0..=DETAIL)
            .map(|i| {
                println!("{}", i);
                get_point_on_bezier(
                    sorted_nodes[0].0,
                    sorted_nodes[1].0,
                    i as f32 / DETAIL as f32,
                )
            })
            .collect::<Vec<Vec3>>();

        let indices = (0..=DETAIL).collect::<Vec<u32>>();
        let normals = vec![Vec3::Y; positions.len()];

        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        *mesh_handle = meshes.add(mesh);
    }
}

// Helpers
fn get_point_on_bezier(t1: &Transform, t2: &Transform, t: f32) -> Vec3 {
    let p0 = t1.translation;
    let p1 = t1.translation + t1.forward() * 2.0;

    let p2 = t2.translation + t2.forward() * 4.0;
    let p3 = t2.translation;

    let a = p0.lerp(p1, t);
    let b = p1.lerp(p2, t);
    let c = p2.lerp(p3, t);

    let d = a.lerp(b, t);
    let e = b.lerp(c, t);

    d.lerp(e, t)
}
