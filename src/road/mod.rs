use bevy::prelude::*;
use petgraph::graph::Graph;

use self::{
    edge::EdgeWeight,
    node::{NodeWeight, RoadNode},
    nodegroup::{regenerate_intersection_mesh, RoadNodeGroup},
};

mod edge;
mod node;
mod nodegroup;

#[derive(Resource, Default)]
pub struct RoadGraph(Graph<NodeWeight, EdgeWeight>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadGraph>()
            .add_startup_system(test_scene)
            .add_system(regenerate_intersection_mesh);
    }
}

fn test_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..default()
                },
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                ..default()
            },
            RoadNodeGroup,
            Name::new("Group"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())),
                    transform: Transform::from_translation(Vec3::new(-6.0, 0.0, -2.0))
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                RoadNode::default(),
                Name::new("Node 1"),
            ));

            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())),
                    transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                RoadNode::default(),
                Name::new("Node 2"),
            ));
        });
}
