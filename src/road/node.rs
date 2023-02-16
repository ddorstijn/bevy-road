use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use itertools::Itertools;
use petgraph::graph::NodeIndex;

use super::control::RoadController;

pub struct NodeWeight;

#[derive(Component, Default)]
pub struct RoadNode(NodeIndex);

#[derive(Bundle, Default)]
pub struct RoadNodeBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub node: RoadNode,
}

// Systems
pub fn regenerate_intersection_mesh(
    controls: Query<&Transform, With<RoadController>>,
    mut nodes: Query<(&mut Handle<Mesh>, &Children), Added<RoadNode>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut node, children) in &mut nodes {
        let mut positions = children
            .iter()
            .filter_map(|c| controls.get(*c).ok())
            .map(|t| t.translation)
            .sorted_by(|a, b| a.z.atan2(a.x).partial_cmp(&b.z.atan2(b.x)).unwrap())
            .collect::<Vec<Vec3>>();

        let length = positions.len() as u32;
        let indices = (0..length)
            .flat_map(|i| {
                let a = vec![length, i, (i + 1) % length];
                println!("{:?}", a);
                a
            })
            .collect_vec();

        // Center vertices
        positions.push(Vec3::ZERO);
        positions
            .iter()
            .enumerate()
            .for_each(|(i, x)| println!("{}: {}", i, x));

        let normals = vec![Vec3::Y; positions.len()];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        *node = meshes.add(mesh);
    }
}

// Helper
