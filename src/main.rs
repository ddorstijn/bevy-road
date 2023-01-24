use bevy::prelude::*;
use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle,
};

pub mod flycam;
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod mesh_generator;
use mesh_generator::{update_dirty, RoadEnd, RoadSegment};

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin) // <- Adds the debug cursor (optional)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RoadSegment>()
            .add_startup_system(setup_scene)
            .add_system(pan_orbit_camera)
            .add_system(update_dirty);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;

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
        .insert(PickingCameraBundle::default())
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
                .spawn((
                    PbrBundle {
                        material: materials.add(Color::rgb(1., 0., 0.).into()),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                        ..default()
                    },
                    PickableBundle::default(),
                ))
                .insert(RoadEnd);
        });

    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.1, 0.0).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20. })),
            ..default()
        })
        .insert(PickableBundle::default())
        .insert(Name::new("Ground"));
}
