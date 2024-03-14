use bevy::{
    math::bounding::{Aabb3d, IntersectsVolume},
    prelude::*,
    render::{
        primitives::Aabb,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    },
    utils::HashSet,
};

use super::edge::{RoadEdge, Twist};

pub struct RoadGridPlugin;
impl Plugin for RoadGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings {
            world_size: Vec2::splat(6000.),
            grid_size: Vec2::splat(2500.),
        })
        .add_plugins(MaterialPlugin::<WorldMaterial>::default())
        .add_systems(Startup, (init_world, group_ground).chain())
        .add_systems(
            Update,
            (add_edge_to_tile, update_edge_of_tile).in_set(WorldSystemSet),
        )
        .add_systems(
            PostUpdate,
            (remove_edge_from_tile, update_material)
                .chain()
                .in_set(WorldSystemSet),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldSystemSet;

#[derive(Resource)]
struct WorldSettings {
    pub world_size: Vec2,
    pub grid_size: Vec2,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct WorldTile {
    pub edges: HashSet<Entity>,
    pub dirty: bool,
}

#[derive(ShaderType, Debug, Clone)]
struct Curve {
    rotation: Vec2,
    center: Vec2,
    angle: Vec2,
    radius: f32,
    lanes: u32,
}

impl From<&RoadEdge> for Curve {
    fn from(edge: &RoadEdge) -> Self {
        // Use center and angle as start and end point for straight lines
        match edge.twist() {
            Twist::Straight => Self {
                rotation: Vec2::new(0.0, 1.0),
                center: edge.start().translation.xz(),
                angle: edge.end().translation.xz() - edge.start().translation.xz(),
                radius: 0.0,
                lanes: edge.lanes() as u32,
            },
            _ => Self {
                rotation: edge.rotation(),
                center: edge.center().xz(),
                angle: Vec2::new((edge.angle() * 0.5).sin(), (edge.angle() * 0.5).cos()),
                radius: edge.radius(),
                lanes: edge.lanes() as u32,
            },
        }
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct WorldMaterial {
    #[storage(2, read_only)]
    pub curves: Vec<Curve>,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for WorldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/curves.wgsl".into()
    }
}

fn init_world(
    settings: Res<WorldSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WorldMaterial>>,
) {
    let grid_resolution = (settings.world_size / settings.grid_size).ceil().as_uvec2();

    let grid_items = (0..(grid_resolution.x * grid_resolution.y))
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
                mesh.compute_aabb().unwrap(),
                MaterialMeshBundle::<WorldMaterial> {
                    transform,
                    mesh: meshes.add(mesh),
                    material: materials.add(WorldMaterial { curves: Vec::new() }),
                    ..default()
                },
                WorldTile::default(),
            )
        })
        .collect::<Vec<(Aabb, MaterialMeshBundle<WorldMaterial>, WorldTile)>>();

    commands.spawn_batch(grid_items);
}

fn group_ground(query: Query<Entity, With<WorldTile>>, mut commands: Commands) {
    let container = commands
        .spawn((SpatialBundle::default(), Name::new("Ground")))
        .id();

    for entity in &query {
        commands.entity(entity).set_parent(container);
    }
}

fn remove_edge_from_tile(
    mut removed_edges: RemovedComponents<RoadEdge>,
    mut tiles: Query<&mut WorldTile>,
) {
    removed_edges.read().for_each(|entity| {
        for mut tile in &mut tiles {
            tile.edges.remove(&entity);
        }
    });
}

fn add_edge_to_tile(
    added_edges: Query<(Entity, &RoadEdge), Added<RoadEdge>>,
    mut tiles: Query<(&mut WorldTile, &Aabb)>,
) {
    for (entity, edge) in &added_edges {
        for (mut tile, tile_aabb) in &mut tiles {
            let tile_aabb3 = Aabb3d {
                min: tile_aabb.min().into(),
                max: tile_aabb.max().into(),
            };

            if edge.aabb3().intersects(&tile_aabb3) {
                tile.edges.insert(entity);
            }
        }
    }
}

fn update_edge_of_tile(
    changed_edges: Query<(Entity, &RoadEdge), Changed<RoadEdge>>,
    mut tiles: Query<(&mut WorldTile, &Aabb)>,
) {
    for (entity, edge) in &changed_edges {
        for (mut tile, tile_aabb) in &mut tiles {
            if tile.edges.contains(&entity) {
                let tile_aabb3 = Aabb3d {
                    min: tile_aabb.min().into(),
                    max: tile_aabb.max().into(),
                };

                if !edge.aabb3().intersects(&tile_aabb3) {
                    tile.edges.remove(&entity);
                }

                tile.dirty = true;
            }
        }
    }
}

fn update_material(
    mut changed_tiles: Query<(&Handle<WorldMaterial>, &mut WorldTile), Changed<WorldTile>>,
    edges: Query<&RoadEdge>,
    mut materials: ResMut<Assets<WorldMaterial>>,
) {
    for (handle, mut tile) in &mut changed_tiles {
        let mat = materials.get_mut(handle).unwrap();
        mat.curves = tile
            .edges
            .iter()
            .map(|entity| {
                Curve::from(
                    edges
                        .get(*entity)
                        .expect("World Tile has entity that is not roadedge"),
                )
            })
            .collect();

        tile.dirty = false;
    }
}
