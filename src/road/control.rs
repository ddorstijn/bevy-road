use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoadController;

#[derive(Bundle, Default)]
pub struct RoadControllerBundle {
    #[bundle]
    pbr: PbrBundle,
    controller: RoadController,
}

impl RoadController {
    fn generate_mesh() {
        todo!();
    }
}
