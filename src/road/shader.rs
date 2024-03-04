use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

pub struct RoadShaderPlugin;
impl Plugin for RoadShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_shader)
            .add_plugins(MaterialPlugin::<CustomMaterial>::default());
    }
}

fn init_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        mesh: meshes.add(Plane3d::new(Vec3::Y)),
        material: materials.add(CustomMaterial {
            curves: vec![
                Curve {
                    center: Vec2::ONE,
                    radius: 0.25,
                    thickness: 0.1,
                },
                Curve {
                    center: Vec2::new(-0.25, -0.25),
                    radius: 0.1,
                    thickness: 0.0,
                },
            ],
        }),
        ..default()
    });
}

#[derive(ShaderType, Debug, Clone)]
struct Curve {
    center: Vec2,
    radius: f32,
    thickness: f32,
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct CustomMaterial {
    #[storage(0, read_only)]
    pub curves: Vec<Curve>,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/curves.wgsl".into()
    }
}
