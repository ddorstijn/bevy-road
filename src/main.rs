use bevy::{
    input::common_conditions::input_just_released, pbr::wireframe::WireframePlugin, prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use camera::PanOrbitCamera;
use debug::RoadEdge;

// use debug::DebugPlugin;
// use road::RoadPlugin;

// pub mod road;
mod camera;
mod debug;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            WireframePlugin,
            WorldInspectorPlugin::default(),
            GamePlugin,
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(RoadPlugin)
            // .add_plugins(DebugPlugin)
            .add_plugins(camera::CameraPlugin)
            .register_type::<SelectedEntity>()
            .init_resource::<SelectedEntity>()
            .add_systems(Startup, setup_scene)
            .add_systems(
                PreUpdate,
                handle_selection.run_if(input_just_released(MouseButton::Left)),
            )
            .add_systems(
                Update,
                (
                    create_road_placeholder.run_if(not(any_with_component::<RoadPlaceholder>())),
                    move_road_placeholder.run_if(any_with_component::<RoadPlaceholder>()),
                ),
            ); //.add_systems(Update, create_road_segment);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Environment and player
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5., 0.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Player"),
        PanOrbitCamera { ..default() },
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            ..default()
        },
        Name::new("Sun"),
    ));

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 20.,
                ..default()
            })),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        Collider::cuboid(10.0, 0.1, 10.0),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        Name::new("Ground"),
    ));

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.5,
                ..default()
            })),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
        RoadSpawner,
    ));
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct SelectedEntity(Option<Entity>);

fn handle_selection(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    mut selection: ResMut<SelectedEntity>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    else {
        return;
    };

    let max_toi = 100.0;
    let solid = true;
    let filter = QueryFilter::new().groups(CollisionGroups::new(
        Group::all().difference(Group::GROUP_2),
        Group::all().difference(Group::GROUP_1),
    ));

    let Some((entity, _)) =
        rapier_context.cast_ray(ray.origin, ray.direction, max_toi, solid, filter)
    else {
        selection.0 = None;
        return;
    };

    selection.0 = Some(entity);
}

#[derive(Component)]
struct RoadSpawner;

#[derive(Component)]
struct RoadNode;

#[derive(Component)]
struct RoadPlaceholder {
    start_position: GlobalTransform,
}

fn create_road_placeholder(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&GlobalTransform, With<RoadSpawner>>,
    selected_entity: Res<SelectedEntity>,
) {
    let Some(entity) = selected_entity.0 else {
        return;
    };
    let Ok(selected_transform) = query.get(entity) else {
        return;
    };

    commands.spawn((
        PbrBundle {
            transform: selected_transform.compute_transform(),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.5,
                ..default()
            })),
            ..default()
        },
        RoadPlaceholder {
            start_position: selected_transform.clone(),
        },
    ));
}

fn move_road_placeholder(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    mut query: Query<(Entity, &RoadPlaceholder, &mut Transform)>,
) {
    let Ok((entity, placeholder, mut transform)) = query.get_single_mut() else {
        return;
    };

    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    else {
        return;
    };

    let max_toi = 100.0;
    let solid = true;
    let filter =
        QueryFilter::default().groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));
    let Some((_, toi)) = rapier_context.cast_ray(ray.origin, ray.direction, max_toi, solid, filter)
    else {
        return;
    };

    transform.translation = ray.origin + ray.direction * toi;
    commands.entity(entity).insert(RoadEdge::new(&placeholder.start_position, transform.translation));
}

// fn create_road_segment(query: Query<&GlobalTransform, Or<(With<RoadSpawner>, With<RoadNode>)>>) {
// if let Ok(road_start) = query.get() {
// road_start

// let edge = RoadEdge::new(road_start, hover.read().last().unwrap().position.unwrap());
// let mesh = edge.generate_mesh();
// println!("{:?}", mesh);
// }
// }
