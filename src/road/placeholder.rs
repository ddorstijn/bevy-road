use bevy::{
    input::{common_conditions::input_just_released, mouse::MouseMotion},
    prelude::*,
};

use crate::{raycast::Raycast, states::GameState};

use super::{
    biarc,
    edge::{RoadEdge, Twist},
    world::WorldTile,
    RoadSpawner,
};

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
        RoadEdge::from_start_end(Transform::from(*start), hitpoint, 2),
        RoadPlaceholder,
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    world_cast: Raycast<With<WorldTile>>,
    world_tiles: Query<&WorldTile>,
    edges: Query<&RoadEdge, Without<RoadPlaceholder>>,
    mut placeholders: Query<(Entity, &mut RoadEdge), With<RoadPlaceholder>>,
    mut commands: Commands,
) {
    let Some((tile_entity, hitpoint)) = world_cast.cursor_ray() else {
        return;
    };

    let mut placeholder_iter = placeholders.iter_mut();
    let Some((_, mut edge)) = placeholder_iter.next() else {
        return;
    };

    let possible_edges = world_tiles
        .get(tile_entity)
        .unwrap()
        .edges
        .iter()
        .filter_map(|edge_entity| edges.get(*edge_entity).ok())
        .collect::<Vec<&RoadEdge>>();

    for edge in possible_edges {
        if !edge.check_hit(hitpoint) {
            continue;
        }

        let lane = match edge.twist() {
            Twist::Straight => match edge.start().left().dot(hitpoint).is_sign_negative() {
                true => edge.lanes() as i32,
                false => -1,
            },
            _ => match edge.radius().powi(2) < (hitpoint - edge.center()).length_squared() {
                true => edge.lanes() as i32,
                false => -1,
            },
        };

        let hit_transform = edge.interpolate_lane(edge.coord_to_length(hitpoint), lane);

        let mut placeholder_iter = placeholders.iter_mut();
        let (_, mut first_edge_placeholder) = placeholder_iter.next().unwrap();
        let (biarc_first_edge, biarc_last_edge) = biarc::compute_biarc(
            first_edge_placeholder.start(),
            hit_transform,
            first_edge_placeholder.lanes(),
        );

        let Some((_, mut placeholder_last_edge)) = placeholder_iter.next() else {
            commands.spawn((
                Name::new("RoadPlaceholder 2"),
                RoadPlaceholder,
                biarc_last_edge,
            ));

            return;
        };

        *first_edge_placeholder = biarc_first_edge;
        *placeholder_last_edge = biarc_last_edge;

        return;
    }

    // No edge
    *edge = RoadEdge::from_start_end(edge.start(), hitpoint, edge.lanes());

    if let Some((entity, _)) = placeholder_iter.next() {
        commands.entity(entity).despawn_recursive();
    }
}

fn finalize_road(mut commands: Commands, query: Query<(Entity, &RoadEdge), With<RoadPlaceholder>>) {
    let (_, edge) = query.iter().last().unwrap();

    for (entity, _) in query.iter() {
        commands.entity(entity).remove::<RoadPlaceholder>();
        commands.entity(entity).insert(Name::new("Road Edge"));
    }

    commands.spawn((
        Name::new("RoadPlaceholder"),
        RoadEdge::from_start_end(
            edge.end(),
            edge.end().translation + *edge.end().forward() * 0.01 + *edge.end().left() * 0.01,
            edge.lanes(),
        ),
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
    world_cast: Raycast<With<WorldTile>>,
    world_tiles: Query<&WorldTile>,
    mut edges: Query<&mut RoadEdge>,
    mut commands: Commands,
) {
    let Some((tile_entity, hitpoint)) = world_cast.cursor_ray() else {
        return;
    };

    for edge_entity in &world_tiles.get(tile_entity).unwrap().edges {
        let mut edge = edges.get_mut(*edge_entity).unwrap();

        if !edge.check_hit(hitpoint) {
            continue;
        }

        let end = edge.end();
        let length_first_half = edge.coord_to_length(hitpoint);
        let length_second_half = edge.length() - length_first_half;

        edge.resize(length_first_half);

        let second_half = RoadEdge::new(
            edge.end(),
            end,
            edge.center(),
            edge.radius(),
            length_second_half,
            edge.twist(),
            edge.lanes(),
        );

        commands.spawn((Name::new("RoadEdge"), second_half));
    }
}
