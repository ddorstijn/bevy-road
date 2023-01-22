use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

pub mod flycam;
use flycam::{pan_orbit_camera, PanOrbitCamera};

pub mod mesh_generator;
use mesh_generator::{update_dirty, RoadSegment};

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
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

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 3500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 2.0),
            ..default()
        })
        .insert(Name::new("Sun"));

    commands
        .spawn(PbrBundle {
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(RoadSegment { ..default() })
        .insert(Name::new("Piecewise Road"))
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    material: materials.add(Color::rgb(1., 0., 1.).into()),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.15 })),
                    ..default()
                })
                .insert(Name::new("Center"));
            parent
                .spawn(PbrBundle {
                    material: materials.add(Color::rgb(0., 1., 0.).into()),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                    ..default()
                })
                .insert(Name::new("Start"));
            parent
                .spawn(PbrBundle {
                    material: materials.add(Color::rgb(1., 0., 0.).into()),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                    ..default()
                })
                .insert(Name::new("End"));
        });
}
