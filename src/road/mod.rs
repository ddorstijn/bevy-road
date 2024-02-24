use bevy::prelude::*;
use petgraph::graph::Graph;

use self::edge::RoadEdge;

pub mod biarc;
pub mod edge;
pub mod node;
pub mod placeholder;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct RoadGraph(Graph<Entity, Entity>);

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadGraph>()
            .register_type::<RoadEdge>()
            .add_systems(Startup, test_biarc)
            .add_plugins(placeholder::PlaceholderPlugin);
    }
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BiarcGizmos {}

fn test_biarc(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let start = GlobalTransform::from(Transform::IDENTITY);
    let end = GlobalTransform::from(Transform {
        translation: Vec3::new(10.0, 0.0, -10.0),
        rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI),
        ..default()
    });

    let (edge1, midpoint, edge2) = biarc::compute_biarc(start, end, 1);

    commands.spawn((
        Name::new("RoadPlaceholder 1"),
        PbrBundle {
            transform: start.compute_transform(),
            mesh: meshes.add(edge1.mesh()),
            ..default()
        },
        edge1.mesh().compute_aabb().unwrap(),
        edge1,
    ));

    let aabb2 = edge2.mesh().compute_aabb().unwrap();
    println!(
        "aabb: {:?}, center: {}",
        aabb2,
        midpoint.translation + midpoint.right() * edge2.radius
    );

    commands.spawn((
        Name::new("RoadPlaceholder 2"),
        PbrBundle {
            transform: midpoint,
            mesh: meshes.add(edge2.mesh()),
            ..default()
        },
        aabb2,
        edge2,
    ));
}
