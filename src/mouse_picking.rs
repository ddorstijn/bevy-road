use bevy::prelude::*;
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

/// A type alias for the concrete [RaycastMesh](bevy_mod_raycast::RaycastMesh) type used for Picking.
pub type PickableMesh = RaycastMesh<PickingRaycastSet>;
/// A type alias for the concrete [RaycastSource](bevy_mod_raycast::RaycastSource) type used for Picking.
pub type PickingCamera = RaycastSource<PickingRaycastSet>;

/// This unit struct is used to tag the generic ray casting types `RaycastMesh` and
/// `RaycastSource`. This means that all Picking ray casts are of the same type. Consequently, any
/// meshes or ray sources that are being used by the picking plugin can be used by other ray
/// casting systems because they will have distinct types, e.g.: `RaycastMesh<PickingRaycastSet>`
/// vs. `RaycastMesh<MySuperCoolRaycastingType>`, and as such wil not result in collisions.
pub struct PickingRaycastSet;

pub struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app
            // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
            // plugin. This includes building rays, casting them, and placing a debug cursor at the
            // intersection. For more advanced uses, you can compose the systems in this plugin however
            // you need. For example, you might exclude the debug cursor system.
            .add_plugin(DefaultRaycastingPlugin::<PickingRaycastSet>::default())
            // You will need to pay attention to what order you add systems! Putting them in the wrong
            // order can result in multiple frames of latency. Ray casting should probably happen near
            // start of the frame. For example, we want to be sure this system runs before we construct
            // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
            // order your systems with the ones provided by the raycasting plugin.
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<PickingRaycastSet>),
            );
    }
}
// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut PickingCamera>,
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
