use bevy::prelude::*;

use self::node::RoadNodeBundle;

mod control;
mod edge;
mod node;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_scene);
    }
}

fn test_scene(mut commands: Commands) {
    commands.spawn(RoadNodeBundle {
        pbr: PbrBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
