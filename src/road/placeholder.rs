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
        RoadEdge::new(Transform::from(*start), hitpoint, 1),
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

    *edge = RoadEdge::new(Transform::from(*transform), hitpoint, edge.lanes());

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

    for (entity, _) in query.iter() {
        commands.entity(entity).remove::<RoadPlaceholder>();
    }

    commands.spawn((
        Name::new("RoadPlaceholder"),
        PbrBundle {
            transform: edge.get_end_transform(None),
            ..default()
        },
        RoadPlaceholder,
        RoadEdge::new(
            edge.get_end_transform(None),
            edge.get_end_transform(None).translation,
            edge.lanes(),
        ),
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
    mut edges: Query<(Entity, &GlobalTransform, &mut RoadEdge)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, hitpoint) in raycast_edges.cursor_ray_intersections().into_iter() {
        // Filter to hit roadedge if applicable
        let Ok((entity, hit_transform, mut hit_edge)) = edges.get_mut(entity) else {
            continue;
        };

        if !hit_edge.check_hit(hitpoint) {
            continue;
        }

        let length_first_half = hit_edge.coord_to_length(hitpoint);
        let length_second_half = hit_edge.length() - length_first_half;

        // hit_edge.length = length_first_half;

        // let second_half = RoadEdge {
        //     lanes: hit_edge.lanes(),
        //     radius: hit_edge.radius(),
        //     length: length_second_half,
        //     start: hit_edge.get_end_transform(None),
        //     end: todo!(),
        //     center: todo!(),
        //     angle: todo!(),
        //     twist: todo!(),
        // };

        // commands.spawn((
        //     Name::new("RoadEdge"),
        //     PbrBundle {
        //         transform: hit_transform
        //             .mul_transform(hit_edge.get_end_transform(None))
        //             .compute_transform(),
        //         ..default()
        //     },
        //     second_half,
        // ));
    }
}
