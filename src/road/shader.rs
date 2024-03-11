use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

use super::{edge::RoadEdge, ROAD_WIDTH};

pub struct RoadShaderPlugin;
impl Plugin for RoadShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_shader)
            .add_systems(Update, update_shader)
            .add_plugins(MaterialPlugin::<CustomMaterial>::default());
    }
}

fn init_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(100.0, 100.0)),
            material: materials.add(CustomMaterial {
                curves: Vec::with_capacity(0),
            }),
            ..default()
        },
        ShaderMarker,
    ));
}

fn update_shader(
    shader: Query<&Handle<CustomMaterial>, With<ShaderMarker>>,
    edges: Query<&RoadEdge>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let handle = shader.single();
    let mat = materials.get_mut(handle).unwrap();

    let curves = edges
        .into_iter()
        .map(|edge| Curve::from(edge))
        .collect::<Vec<Curve>>();

    mat.curves = curves;
}

#[derive(Component)]
struct ShaderMarker;

#[derive(ShaderType, Debug, Clone)]
struct Curve {
    rotation: Vec2,
    center: Vec2,
    angle: Vec2,
    radius: f32,
    thickness: f32,
}

impl From<&RoadEdge> for Curve {
    fn from(edge: &RoadEdge) -> Self {
        let curve = Self {
            rotation: edge.rotation(),
            center: edge.center().xz(),
            angle: Vec2::new((edge.angle() * 0.5).sin(), (edge.angle() * 0.5).cos()),
            radius: edge.radius(),
            thickness: edge.lanes() as f32 * ROAD_WIDTH,
        };

        curve
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct CustomMaterial {
    #[storage(2, read_only)]
    pub curves: Vec<Curve>,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/curves.wgsl".into()
    }
}
