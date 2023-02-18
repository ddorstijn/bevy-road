use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use itertools::Itertools;

use super::{node::RoadNode, RoadGraph};

#[derive(Component, Default)]
pub struct RoadNodeGroup;

// Systems
pub fn regenerate_intersection_mesh(
    mut groups: Query<(&mut Handle<Mesh>, &Children), Added<RoadNode>>,
    nodes: Query<(&Transform, &RoadNode)>,
    graph: Res<RoadGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut mesh_handle, children) in &mut groups {
        let mut positions = children
            .iter()
            .filter_map(|c| nodes.get(*c).ok())
            .sorted_by(|(a, _), (b, _)| {
                a.translation
                    .angle_between(Vec3::X)
                    .partial_cmp(&b.translation.angle_between(Vec3::X))
                    .unwrap()
            })
            .flat_map(|(t, RoadNode(i))| {
                let lanes = graph.0.node_weight(*i).unwrap().lanes;
                let offset = t.left() * (0.5 * lanes as f32);

                vec![t.translation - offset, t.translation + offset]
            })
            .collect::<Vec<Vec3>>();

        let length = positions.len() as u32;
        let indices = (0..length)
            .flat_map(|i| vec![length, (i + 1) % length, i])
            .collect::<Vec<u32>>();

        // Center vertices
        positions.push(Vec3::ZERO);

        let normals = vec![Vec3::Y; positions.len()];

        positions
            .iter()
            .for_each(|p| println!("({}, {})", p.x, p.z));

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        *mesh_handle = meshes.add(mesh);
    }
}
