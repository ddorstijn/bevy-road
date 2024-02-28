use bevy::{
    input::{common_conditions::input_just_released, mouse::MouseMotion},
    prelude::*,
};

use crate::{raycast::Raycast, states::GameState, GroundMarker};

use super::{edge::RoadEdge, node::RoadSpawner};

pub struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (start_building.run_if(input_just_released(MouseButton::Left)),)
                    .in_set(BuildSystemSet::NotBuilding),
                (
                    move_road_placeholder.run_if(on_event::<MouseMotion>()),
                    finalize_road.run_if(input_just_released(MouseButton::Left)),
                )
                    .in_set(BuildSystemSet::Building),
            ),
        )
        .add_systems(
            OnExit(GameState::Building),
            (
                remove_placeholder.run_if(any_with_component::<RoadPlaceholder>),
                hide_nodes,
            )
                .in_set(BuildSystemSet::ExitBuildMode),
        )
        .add_systems(OnEnter(GameState::Building), show_nodes)
        .configure_sets(
            Update,
            (
                BuildSystemSet::NotBuilding.run_if(not(any_with_component::<RoadPlaceholder>)),
                BuildSystemSet::Building.run_if(any_with_component::<RoadPlaceholder>),
            )
                .run_if(in_state(GameState::Building)),
        )
        .configure_sets(OnEnter(GameState::Building), BuildSystemSet::EnterBuildMode)
        .configure_sets(OnExit(GameState::Building), BuildSystemSet::ExitBuildMode);
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, SystemSet)]
enum BuildSystemSet {
    EnterBuildMode,
    ExitBuildMode,
    Building,
    NotBuilding,
}

fn start_building(
    raycast: Raycast<With<RoadSpawner>>,
    node_query: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let Some((entity, hitpoint)) = raycast.cursor_ray() else {
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
        RoadEdge::new(start.transform_point(hitpoint), 1),
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    raycast_edges: Raycast<(With<RoadEdge>, Without<RoadPlaceholder>)>,
    raycast_ground: Raycast<With<GroundMarker>>,
    query_edges: Query<(&GlobalTransform, &RoadEdge), Without<RoadPlaceholder>>,
    mut query: Query<(&mut Handle<Mesh>, &GlobalTransform, &mut RoadEdge), With<RoadPlaceholder>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: Gizmos,
) {
    for (entity, hitpoint) in raycast_edges.cursor_ray_intersections().into_iter() {
        let Ok((transform, edge)) = query_edges.get(entity) else {
            continue;
        };

        let local_hitpoint = transform
            .compute_matrix()
            .inverse()
            .transform_point(hitpoint);

        if !edge.check_hit(local_hitpoint) {
            continue;
        }

        gizmos.sphere(hitpoint, Quat::IDENTITY, 1.0, Color::GREEN);

        let length = edge.coordinates_to_length(local_hitpoint.xz());
        let a = edge.interpolate_lane(length, edge.lanes);

        gizmos.sphere(
            transform.transform_point(a.translation),
            Quat::IDENTITY,
            0.1,
            Color::ORANGE,
        );

        return;
    }

    let Some((_, hitpoint)) = raycast_ground.cursor_ray() else {
        return;
    };

    let Ok((mut handle, transform, mut edge)) = query.get_single_mut() else {
        return;
    };

    let point = transform
        .compute_matrix()
        .inverse()
        .transform_point(hitpoint);
    *edge = RoadEdge::new(point, edge.lanes);
    if edge.length != 0.0 {
        *handle = meshes.add(edge.mesh());
    }
}

fn finalize_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query: Query<(Entity, &GlobalTransform, &RoadEdge), With<RoadPlaceholder>>,
) {
    let Ok((entity, global_transform, edge)) = query.get_single() else {
        return;
    };

    commands.entity(entity).remove::<RoadPlaceholder>();
    commands
        .entity(entity)
        .insert(edge.mesh().compute_aabb().unwrap());

    for lane in 0..edge.lanes {
        let end = edge.get_end_transform(Some(lane));

        const NODE_END_HALF_WIDTH: f32 = 0.20;
        let cuboid = Cuboid {
            half_size: Vec3::splat(NODE_END_HALF_WIDTH),
        };

        let id = commands
            .spawn((
                cuboid.mesh().compute_aabb().unwrap(),
                PbrBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 0.0)),
                    mesh: meshes.add(cuboid),
                    transform: end,
                    ..default()
                },
                Name::new(format!("Road Endpoint lane {}", lane)),
                RoadSpawner,
            ))
            .id();

        commands.entity(entity).add_child(id);
    }

    commands.spawn((
        Name::new("RoadPlaceholder"),
        PbrBundle {
            transform: global_transform
                .mul_transform(edge.get_end_transform(None))
                .compute_transform(),
            ..default()
        },
        RoadPlaceholder,
        RoadEdge {
            lanes: edge.lanes,
            ..default()
        },
    ));
}

fn remove_placeholder(mut commands: Commands, query: Query<Entity, With<RoadPlaceholder>>) {
    let entity = query.single();

    commands.entity(entity).despawn_recursive();
}

fn hide_nodes(mut query: Query<&mut Visibility, With<RoadSpawner>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn show_nodes(mut query: Query<&mut Visibility, With<RoadSpawner>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}
