use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;

// use debug::DebugPlugin;
//use road::RoadPlugin;

// pub mod road;
// mod debug;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins,
            WireframePlugin,
            WorldInspectorPlugin::default(),
            GamePlugin
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(RoadPlugin)
            // .add_plugins(DebugPlugin)

            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Environment and player
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5., 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Player")
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
                ..default()
            },
            ..default()
        },
        Name::new("Sun")
    ));

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.4, 0.4).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20., ..default() })),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        // PickableBundle::default(), // Optional: adds selection, highlighting, and helper components.
        On::<Pointer<Click>>::target_commands_mut(|click, _target_commands| {
            println!("{:?}", click);
        }),
        Name::new("Ground")
    ));
}

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, raycast);
    }
}