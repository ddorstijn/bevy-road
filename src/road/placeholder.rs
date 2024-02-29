use bevy::{
    input::{common_conditions::input_just_released, mouse::MouseMotion},
    prelude::*,
};

use crate::{raycast::Raycast, road::biarc, states::GameState, GroundMarker};

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
    mut query_placeholders: Query<
        (Entity, &mut Handle<Mesh>, &GlobalTransform, &mut RoadEdge),
        With<RoadPlaceholder>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (entity, hitpoint) in raycast_edges.cursor_ray_intersections().into_iter() {
        let Ok((hit_transform, hit_edge)) = query_edges.get(entity) else {
            continue;
        };

        let local_hitpoint = hit_transform
            .compute_matrix()
            .inverse()
            .transform_point(hitpoint);

        if !hit_edge.check_hit(local_hitpoint) {
            continue;
        }

        let length = hit_edge.coordinates_to_length(local_hitpoint.xz());
        let end = hit_transform.mul_transform(hit_edge.interpolate_lane(length, hit_edge.lanes));

        let mut placeholder_iter = query_placeholders.iter_mut();
        let (entity, mut handle, transform, first_edge_placeholder) =
            placeholder_iter.next().unwrap();
        let (biarc_first_edge, midpoint, biarc_last_edge) =
            biarc::compute_biarc(*transform, end, first_edge_placeholder.lanes);

        *handle = meshes.add(biarc_first_edge.mesh());
        commands.entity(entity).insert(biarc_first_edge);

        let Some((entity, mut handle, _, mut placeholder_last_edge)) = placeholder_iter.next()
        else {
            commands.spawn((
                Name::new("RoadPlaceholder 2"),
                PbrBundle {
                    transform: midpoint,
                    mesh: meshes.add(biarc_last_edge.mesh()),
                    ..default()
                },
                biarc_last_edge,
                RoadPlaceholder,
            ));

            return;
        };

        commands
            .entity(entity)
            .insert(GlobalTransform::from(midpoint));
        *handle = meshes.add(biarc_last_edge.mesh());
        *placeholder_last_edge = biarc_last_edge;

        return;
    }

    let Some((_, hitpoint)) = raycast_ground.cursor_ray() else {
        return;
    };

    let mut placeholder_iter = query_placeholders.iter_mut();
    let Some((_, mut handle, transform, mut edge)) = placeholder_iter.next() else {
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

    if let Some((entity, _, _, _)) = placeholder_iter.next() {
        commands.entity(entity).despawn_recursive();
    }
}

fn finalize_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query: Query<(Entity, &GlobalTransform, &RoadEdge), With<RoadPlaceholder>>,
) {
    let (entity, global_transform, edge) = query.iter().last().unwrap();
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

    for (entity, _, edge) in query.iter() {
        commands.entity(entity).remove::<RoadPlaceholder>();
        commands
            .entity(entity)
            .insert(edge.mesh().compute_aabb().unwrap());
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
