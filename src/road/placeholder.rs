use bevy::{
    input::{common_conditions::input_just_released, mouse::MouseMotion},
    prelude::*,
};

use crate::{raycast::Raycast, states::GameState};

use super::{edge::RoadEdge, world::GroundMarker, RoadSpawner};

pub struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    start_building.run_if(input_just_released(MouseButton::Left)),
                    snip_road.run_if(input_just_released(MouseButton::Right)),
                )
                    .in_set(BuildSystemSet::NotBuilding),
                (
                    move_road_placeholder.run_if(on_event::<MouseMotion>()),
                    finalize_road.run_if(input_just_released(MouseButton::Left)),
                )
                    .chain()
                    .in_set(BuildSystemSet::Building),
            ),
        )
        .add_systems(
            OnExit(GameState::Building),
            (
                remove_placeholders.run_if(any_with_component::<RoadPlaceholder>),
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
        RoadEdge::from_start_end(Transform::from(*start), hitpoint, 1),
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    raycast_ground: Raycast<With<GroundMarker>>,
    mut query_placeholders: Query<(Entity, &GlobalTransform, &mut RoadEdge), With<RoadPlaceholder>>,
    mut commands: Commands,
) {
    let Some((_, hitpoint)) = raycast_ground.cursor_ray() else {
        return;
    };

    let mut placeholder_iter = query_placeholders.iter_mut();
    let Some((_, transform, mut edge)) = placeholder_iter.next() else {
        return;
    };

    *edge = RoadEdge::from_start_end(Transform::from(*transform), hitpoint, edge.lanes());

    if let Some((entity, _, _)) = placeholder_iter.next() {
        commands.entity(entity).despawn_recursive();
    }
}

fn finalize_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query: Query<(Entity, &RoadEdge), With<RoadPlaceholder>>,
) {
    let (entity, edge) = query.iter().last().unwrap();
    for lane in 0..edge.lanes() {
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

        commands.entity(id).set_parent_in_place(entity);
    }

    for (entity, edge) in query.iter() {
        commands.entity(entity).remove::<RoadPlaceholder>();
        commands.entity(entity).insert(edge.aabb());
    }

    commands.spawn((
        Name::new("RoadPlaceholder"),
        PbrBundle {
            transform: edge.end(),
            ..default()
        },
        RoadPlaceholder,
        RoadEdge::from_start_end(edge.end(), Vec3::ZERO, edge.lanes()),
    ));
}

fn remove_placeholders(mut commands: Commands, query: Query<Entity, With<RoadPlaceholder>>) {
    for entity in query.into_iter() {
        commands.entity(entity).despawn_recursive();
    }
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

fn snip_road(
    raycast_edges: Raycast<With<RoadEdge>>,
    mut edges: Query<&mut RoadEdge>,
    mut commands: Commands,
) {
    for (entity, hitpoint) in raycast_edges.cursor_ray_intersections().into_iter() {
        // Filter to hit roadedge if applicable
        let Ok(mut hit_edge) = edges.get_mut(entity) else {
            continue;
        };

        if !hit_edge.check_hit(hitpoint) {
            continue;
        }

        let end = hit_edge.end();
        let angle_first_half = hit_edge.coord_to_angle(hitpoint);
        let angle_second_half = hit_edge.angle() - angle_first_half;

        hit_edge.resize(angle_first_half);
        commands.entity(entity).insert(hit_edge.aabb());

        let second_half = RoadEdge::new(
            hit_edge.end(),
            end,
            hit_edge.center(),
            hit_edge.radius(),
            angle_second_half,
            hit_edge.twist(),
            hit_edge.lanes(),
        );

        commands.spawn((
            Name::new("RoadEdge"),
            PbrBundle {
                transform: second_half.start(),
                ..default()
            },
            second_half.aabb(),
            second_half,
        ));
    }
}
