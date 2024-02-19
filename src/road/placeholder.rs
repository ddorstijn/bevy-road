use bevy::{input::common_conditions::input_just_released, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{camera::PanOrbitCamera, states::GameState, utility::cast_ray_from_cursor};

use super::{edge::RoadEdge, node::RoadSpawner};

pub struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_build_selection
                    .run_if(input_just_released(MouseButton::Left))
                    .in_set(BuildSystemSet::NotBuilding),
                (
                    move_road_placeholder,
                    finalize_road.run_if(input_just_released(MouseButton::Left)),
                )
                    .in_set(BuildSystemSet::Building),
            ),
        )
        .configure_sets(
            Update,
            (
                BuildSystemSet::NotBuilding.run_if(not(any_with_component::<RoadPlaceholder>())),
                BuildSystemSet::Building.run_if(any_with_component::<RoadPlaceholder>()),
            )
                .run_if(in_state(GameState::Building)),
        );
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, SystemSet)]
enum BuildSystemSet {
    Building,
    NotBuilding,
}

fn handle_build_selection(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,

    node_query: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let filter = QueryFilter::default();

    let Some((entity, hitpoint)) =
        cast_ray_from_cursor(rapier_context, window_query, camera_query, filter)
    else {
        return;
    };
    let Ok(start) = node_query.get(entity) else {
        return;
    };

    commands.spawn((
        Name::new("RoadPlaceholder"),
        PbrBundle {
            transform: start.compute_transform(),
            ..default()
        },
        RoadPlaceholder,
        RoadEdge::new(start.transform_point(hitpoint)),
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,

    mut query: Query<(&mut Handle<Mesh>, &GlobalTransform, &mut RoadEdge), With<RoadPlaceholder>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((mut handle, transform, mut edge)) = query.get_single_mut() else {
        return;
    };

    let filter =
        QueryFilter::default().groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));
    let Some((_, hitpoint)) =
        cast_ray_from_cursor(rapier_context, window_query, camera_query, filter)
    else {
        return;
    };

    let point = transform
        .compute_matrix()
        .inverse()
        .transform_point(hitpoint);
    *edge = RoadEdge::new(point);
    if point.angle_between(Vec3::Z) > 0.001 {
        *handle = meshes.add(edge.generate_mesh());
    }
}

fn finalize_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query: Query<(Entity, &RoadEdge), With<RoadPlaceholder>>,
) {
    let Ok((entity, edge)) = query.get_single() else {
        return;
    };

    let end = edge.get_end_transform();

    let id = commands
        .spawn((
            PbrBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: 0.5,
                    ..default()
                })),
                transform: end,
                ..default()
            },
            Collider::cuboid(0.5, 0.5, 0.5),
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            Name::new("Road Endpoint"),
            RoadSpawner,
        ))
        .id();

    commands.entity(entity).add_child(id);
    commands.entity(entity).remove::<RoadPlaceholder>();
}
