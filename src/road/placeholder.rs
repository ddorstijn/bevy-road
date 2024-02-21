use bevy::{input::common_conditions::input_just_released, prelude::*};

use crate::{
    raycast::{Raycast, RaycastGroup},
    states::GameState,
};

use super::{edge::RoadEdge, node::RoadSpawner};

pub struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    start_building.run_if(input_just_released(MouseButton::Left)),
                    hover_road,
                )
                    .in_set(BuildSystemSet::NotBuilding),
                (
                    adjust_lanes,
                    move_road_placeholder.after(adjust_lanes),
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

fn start_building(raycast: Raycast, node_query: Query<&GlobalTransform>, mut commands: Commands) {
    let Some((entity, hitpoint)) = raycast.cursor_ray(None) else {
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
        RaycastGroup { group: 0b1 },
    ));
}

#[derive(Component)]
pub struct RoadPlaceholder;

fn move_road_placeholder(
    raycast: Raycast,
    mut query: Query<(&mut Handle<Mesh>, &GlobalTransform, &mut RoadEdge), With<RoadPlaceholder>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Some((_, hitpoint)) = raycast.cursor_ray(None) else {
        return;
    };

    let count = query.iter().count();

    if count == 1 {
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
}

fn adjust_lanes(
    mut query: Query<&mut RoadEdge, With<RoadPlaceholder>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut edge = query.single_mut();
    if keys.just_pressed(KeyCode::Digit1) {
        edge.lanes = 1
    }

    if keys.just_pressed(KeyCode::Digit2) {
        edge.lanes = 2
    }

    if keys.just_pressed(KeyCode::Digit3) {
        edge.lanes = 3
    }

    if keys.just_pressed(KeyCode::Digit4) {
        edge.lanes = 4
    }

    if keys.just_pressed(KeyCode::Digit5) {
        edge.lanes = 5
    }

    if keys.just_pressed(KeyCode::Digit6) {
        edge.lanes = 6
    }

    if keys.just_pressed(KeyCode::Digit7) {
        edge.lanes = 7
    }

    if keys.just_pressed(KeyCode::Digit8) {
        edge.lanes = 8
    }

    if keys.just_pressed(KeyCode::Digit9) {
        edge.lanes = 9
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

    commands.entity(entity).remove::<RoadPlaceholder>();
    commands
        .entity(entity)
        .insert(edge.mesh().compute_aabb().unwrap());

    for lane in 0..edge.lanes {
        let end = edge.get_end_transform(Some(lane));

        const NODE_END_HALF_WIDTH: f32 = 0.20;
        let id = commands
            .spawn((
                PbrBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 0.0)),
                    mesh: meshes.add(Cuboid {
                        half_size: Vec3::splat(NODE_END_HALF_WIDTH),
                    }),
                    transform: end,
                    ..default()
                },
                Name::new(format!("Road Endpoint lane {}", lane)),
                RoadSpawner,
            ))
            .id();

        commands.entity(entity).add_child(id);
    }
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

fn hover_road(
    mut gizmos: Gizmos,
    raycast: Raycast,
    query: Query<(&GlobalTransform, &RoadEdge), Without<RoadPlaceholder>>,
) {
    let Some((entity, hitpoint)) = raycast.cursor_ray(Some(0b1)) else {
        return;
    };

    gizmos.sphere(hitpoint, Quat::IDENTITY, 1.0, Color::GREEN);

    let Ok((transform, edge)) = query.get(entity) else {
        return;
    };

    gizmos.sphere(
        transform.translation() + transform.right() * edge.lanes as f32,
        Quat::IDENTITY,
        0.5,
        Color::ORANGE,
    );
}
