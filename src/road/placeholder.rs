use bevy::{
    input::{common_conditions::input_just_released, mouse::MouseMotion},
    prelude::*,
};

use crate::{raycast::Raycast, states::GameState};

use super::{biarc, edge::RoadEdge, world::WorldTile, RoadSpawner};

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
            (remove_placeholders, hide_nodes).in_set(BuildSystemSet::ExitBuildMode),
        )
        .add_systems(OnEnter(GameState::Building), show_nodes);
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, SystemSet)]
pub enum BuildSystemSet {
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
        Visibility::default(),
        RoadEdge::from_start_end(Transform::from(*start), hitpoint, 1),
        RoadPlaceholder,
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    // raycast_edges: Raycast<With<RoadEdge>>,
    raycast_ground: Raycast<With<WorldTile>>,
    mut query_placeholders: Query<(Entity, &mut RoadEdge), With<RoadPlaceholder>>,
    // query_edges: Query<&RoadEdge, Without<RoadPlaceholder>>,
    mut commands: Commands,
) {
    // // Case: Edge
    // for (entity, hitpoint) in raycast_edges.cursor_ray_intersections().into_iter() {
    //     // Filter to hit roadedge if applicable
    //     let Ok(hit_edge) = query_edges.get(entity) else {
    //         continue;
    //     };

    //     if !hit_edge.check_hit(hitpoint) {
    //         continue;
    //     }

    //     let hit_transform = hit_edge.interpolate(hit_edge.coord_to_length(hitpoint));

    //     let mut placeholder_iter = query_placeholders.iter_mut();
    //     let (entity, first_edge_placeholder) = placeholder_iter.next().unwrap();
    //     let (biarc_first_edge, midpoint, biarc_last_edge) = biarc::compute_biarc(
    //         first_edge_placeholder.start(),
    //         hit_transform,
    //         first_edge_placeholder.lanes(),
    //     );

    //     commands.entity(entity).insert(biarc_first_edge);

    //     let Some((entity, mut placeholder_last_edge)) = placeholder_iter.next() else {
    //         commands.spawn((
    //             Name::new("RoadPlaceholder 2"),
    //
    //             RoadPlaceholder,
    //             biarc_last_edge,
    //         ));

    //         return;
    //     };

    //     commands
    //         .entity(entity)
    //         .insert(GlobalTransform::from(midpoint));
    //     *placeholder_last_edge = biarc_last_edge;

    //     return;
    // }

    // Case: ground
    let Some((_, hitpoint)) = raycast_ground.cursor_ray() else {
        return;
    };

    let mut placeholder_iter = query_placeholders.iter_mut();
    let Some((_, mut edge)) = placeholder_iter.next() else {
        return;
    };

    *edge = RoadEdge::from_start_end(edge.start(), hitpoint, edge.lanes());

    if let Some((entity, _)) = placeholder_iter.next() {
        commands.entity(entity).despawn_recursive();
    }
}

fn finalize_road(mut commands: Commands, query: Query<(Entity, &RoadEdge), With<RoadPlaceholder>>) {
    let (_, edge) = query.iter().last().unwrap();

    for (entity, _) in query.iter() {
        commands.entity(entity).remove::<RoadPlaceholder>();
    }

    commands.spawn((
        Name::new("RoadPlaceholder"),
        RoadEdge::from_start_end(edge.end(), Vec3::ZERO, edge.lanes()),
        RoadPlaceholder,
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

        let second_half = RoadEdge::new(
            hit_edge.end(),
            end,
            hit_edge.center(),
            hit_edge.radius(),
            angle_second_half,
            hit_edge.twist(),
            hit_edge.lanes(),
        );

        commands.spawn((Name::new("RoadEdge"), second_half));
    }
}
