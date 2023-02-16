use bevy::prelude::*;
use petgraph::graph::Graph;

use self::{
    control::{RoadController, RoadControllerBundle},
    edge::EdgeWeight,
    node::{regenerate_intersection_mesh, NodeWeight, RoadNodeBundle},
};

mod control;
mod edge;
mod node;

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
            RoadNodeBundle {
                pbr: PbrBundle {
                    transform: Transform {
                        translation: Vec3::ZERO,
                        ..default()
                    },
                    mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                    ..default()
                },
                ..default()
            },
            Name::new("Node1"),
        ))
        .with_children(|parent| {
            parent.spawn((
                RoadControllerBundle {
                    pbr: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube::default())),
                        transform: Transform::from_translation(Vec3::new(-6.0, 0.0, 0.0))
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    controller: RoadController,
                },
                Name::new("Controller1"),
            ));

            parent.spawn((
                RoadControllerBundle {
                    pbr: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube::default())),
                        transform: Transform::from_translation(Vec3::new(9.0, 0.0, 5.0))
                            .looking_at(Vec3::new(10.0, 0.0, 5.0), Vec3::Y),
                        ..default()
                    },
                    controller: RoadController,
                },
                Name::new("Controller2"),
            ));

            parent.spawn((
                RoadControllerBundle {
                    pbr: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube::default())),
                        transform: Transform::from_translation(Vec3::new(-1.0, 0.0, 10.0))
                            .looking_at(Vec3::new(6.0, 0.0, 10.0), Vec3::Y),
                        ..default()
                    },
                    controller: RoadController,
                },
                Name::new("Controller3"),
            ));
        });
}
