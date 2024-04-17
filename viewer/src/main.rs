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
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use building::BuilderPlugin;
use debug::DebugPlugin;
use road::RoadPlugin;
use states::GameStatePlugin;

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
            PanOrbitCameraPlugin,
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: false,
            ..default()
        })
        .add_plugins((GameStatePlugin, RoadPlugin, BuilderPlugin))
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    // Environment and player
    commands
        .spawn((Camera3dBundle::default(), Name::new("Player")))
        .insert(PanOrbitCamera {
            radius: Some(50.0),
            ..default()
        });

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
