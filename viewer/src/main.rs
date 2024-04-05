use std::f32::consts::PI;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use camera::{CameraPlugin, PanOrbitCamera};
use debug::DebugPlugin;
use road::RoadPlugin;
use states::GameStatePlugin;

pub(self) mod camera;
pub(self) mod debug;
pub(self) mod road;
pub(self) mod states;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: false,
            ..default()
        })
        .add_plugins((CameraPlugin, GameStatePlugin, RoadPlugin))
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    // Environment and player
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10., 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::X, -0.5 * PI)),
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
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
        Name::new("Sun"),
    ));
}
