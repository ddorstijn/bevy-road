use bevy::prelude::*;

#[derive(Component)]
pub struct RoadController;

#[derive(Bundle)]
pub struct RoadControllerBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub controller: RoadController,
}
