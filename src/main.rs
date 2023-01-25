use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};

pub mod flycam;
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod road;
use road::{regenerate_mesh, RoadEnd, RoadSegment};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadSegment>()
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
            )
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera)
            .add_system(regenerate_mesh)
            .add_system(update_debug_cursor);
    }
}

/// This is a unit struct we will use to mark our generic `RaycastMesh`s and `RaycastSource` as part
/// of the same group, or "RaycastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
struct MyRaycastSet;

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

fn update_debug_cursor(
    mut materials: ResMut<Assets<CustomMaterial>>,
    cursors: Query<&Intersection<MyRaycastSet>>,
    road: Query<&Handle<CustomMaterial>>,
) {
    // Set the cursor translation to the top pick's world coordinates
    let intersection = match cursors.iter().last() {
        Some(x) => x,
        None => return,
    };
    if let Some(new_matrix) = intersection.normal_ray() {
        let coord = Transform::from_matrix(new_matrix.to_transform()).translation;
        for handle in &road {
            materials.get_mut(handle).unwrap().mouse_position = Vec2::new(coord.x, coord.z);
        }
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

    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert(RaycastSource::<MyRaycastSet>::new())
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
        .insert(Name::new("Ground"))
        .insert(RaycastMesh::<MyRaycastSet>::default());

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
