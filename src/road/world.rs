use bevy::prelude::*;

pub struct RoadGridPlugin;
impl Plugin for RoadGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings {
            world_size: Vec2::splat(6000.),
            grid_size: Vec2::splat(2500.),
        })
        .add_systems(Startup, (init_world, group_ground).chain());
    }
}

#[derive(Resource)]
struct WorldSettings {
    pub world_size: Vec2,
    pub grid_size: Vec2,
}

#[derive(Component)]
pub struct GroundMarker;

fn init_world(
    settings: Res<WorldSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_resolution = (settings.world_size / settings.grid_size).ceil().as_uvec2();

    let grid_items: Vec<(PbrBundle, GroundMarker)> = (0..(grid_resolution.x * grid_resolution.y))
        .map(|i| {
            let plane = Plane3d::new(Vec3::Y);
            let mesh = plane
                .mesh()
                .size(settings.grid_size.x, settings.grid_size.y)
                .build();

            let transform = Transform::from_xyz(
                (i % grid_resolution.x) as f32 * settings.grid_size.x
                    - (grid_resolution.x - 1) as f32 * 0.5 * settings.grid_size.x,
                -0.001,
                (i / grid_resolution.y) as f32 * settings.grid_size.y
                    - (grid_resolution.y - 1) as f32 * 0.5 * settings.grid_size.y,
            );

            (
                PbrBundle {
                    transform,
                    mesh: meshes.add(mesh),
                    material: materials.add(Color::BLUE),
                    ..default()
                },
                GroundMarker,
            )
        })
        .collect();

    commands.spawn_batch(grid_items);
}

fn group_ground(query: Query<Entity, With<GroundMarker>>, mut commands: Commands) {
    let container = commands
        .spawn((SpatialBundle::default(), Name::new("Ground")))
        .id();

    for entity in &query {
        commands.entity(entity).set_parent(container);
    }
}
