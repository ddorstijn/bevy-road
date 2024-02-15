use bevy::{input::common_conditions::input_just_released, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{camera::PanOrbitCamera, states::GameState, utility::cast_ray_from_cursor};

use super::edge::RoadEdge;

pub struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_build_selection
                    .run_if(input_just_released(MouseButton::Left))
                    .run_if(not(any_with_component::<RoadPlaceholder>())),
                move_road_placeholder.run_if(any_with_component::<RoadPlaceholder>()),
                // finalize_road,
            )
                .run_if(in_state(GameState::Building)),
        );
    }
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
        cast_ray_from_cursor(rapier_context, window_query, camera_query, filter) else { return; };
    let Ok(start) = node_query.get(entity) else {
        return;
    };

    commands.spawn((
        Name::new("RoadPlaceholder"),
        PbrBundle::default(),
        RoadPlaceholder,
        RoadEdge::new(start, hitpoint, None),
    ));
}

#[derive(Component)]
struct RoadPlaceholder;

fn move_road_placeholder(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,

    gizmos: Gizmos,

    mut query: Query<(&mut Handle<Mesh>, &mut RoadEdge), With<RoadPlaceholder>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((mut handle, mut edge)) = query.get_single_mut() else {
        return;
    };

    let filter =
        QueryFilter::default().groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));
    let Some((_, hitpoint)) =
        cast_ray_from_cursor(rapier_context, window_query, camera_query, filter)
    else {
        return;
    };

    edge.recalculate(hitpoint, Some(gizmos));
    *handle = meshes.add(edge.generate_mesh());
}

// fn finalize_road(
//     mut commands: Commands,
//     query: Query<(Entity, &RoadEdge, &Transform), With<RoadPlaceholder>>,
// ) {
//     let Ok((entity, edge, transform)) = query.get_single() else {
//         return;
//     };

//     let final_edge = edge;

//     commands.entity(entity).remove::<RoadPlaceholder>();
// }
