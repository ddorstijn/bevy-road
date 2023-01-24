use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod flycam;
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod road;
use road::{regenerate_mesh, RoadEnd, RoadSegment};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadSegment>()
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera)
            .add_system(regenerate_mesh)
            .add_system(update_mouse);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut c_materials: ResMut<Assets<CustomMaterial>>,
) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert(Name::new("Player"));

    const HALF_SIZE: f32 = 10.0;
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                // Configure the projection to better fit the scene
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Sun"));

    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.05, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RoadSegment { ..default() })
        .insert(Name::new("Piecewise Road"))
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    material: materials.add(Color::rgb(1., 0., 0.).into()),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                    ..default()
                })
                .insert(RoadEnd);
        });

    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.1, 0.0).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20. })),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: c_materials.add(CustomMaterial {
            mouse_position: Vec2::new(1.0, 0.0),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    mouse_position: Vec2,
    alpha_mode: AlphaMode,
}

fn update_mouse(
    query: Query<&mut Handle<CustomMaterial>>,
    mut c_materials: ResMut<Assets<CustomMaterial>>,
) {
    for handle in query.iter() {
        c_materials.get_mut(handle).unwrap().mouse_position += 0.001;
    }
}
