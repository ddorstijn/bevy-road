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
    twist: u32,
    center: Vec2,
    start: Vec2,
    end: Vec2,
    radius: f32,
    length: f32,
    lanes: u32,
}

impl From<&RoadEdge> for Curve {
    fn from(edge: &RoadEdge) -> Self {
        let rel_start = edge.start().translation.xz() - edge.center().xz();
        let rel_end = edge.end().translation.xz() - edge.center().xz();

        // Use center and angle as start and end point for straight lines
        match edge.twist() {
            Twist::Clockwise => Self {
                twist: 1,
                center: edge.center().xz(),
                start: rel_start,
                end: rel_end,
                radius: edge.radius(),
                length: edge.length(),
                lanes: edge.lanes() as u32,
            },
            Twist::CounterClockwise => Self {
                twist: 0,
                center: edge.center().xz(),
                start: rel_start,
                end: rel_end,
                radius: edge.radius(),
                length: edge.length(),
                lanes: edge.lanes() as u32,
            },
            Twist::Straight => Self {
                twist: 2,
                center: edge.center().xz(),
                start: rel_start,
                end: rel_end,
                radius: 0.0,
                length: edge.length(),
                lanes: edge.lanes() as u32,
            },
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct WorldMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<Image>>,

    #[storage(2, read_only)]
    pub curves: Vec<Curve>,
}

impl Material for WorldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/world.wgsl".into()
    }
}

fn init_world(
    settings: Res<WorldSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WorldMaterial>>,
    asset_server: Res<AssetServer>,
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
                    material: materials.add(WorldMaterial {
                        color_texture: Some(asset_server.load("textures/road.png")),
                        curves: Vec::new(),
                    }),
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
                        .expect("World Tile has entity that is not a road edge"),
                )
            })
            .collect();

        tile.dirty = false;
    }
}
