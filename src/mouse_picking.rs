use bevy::prelude::{Res, Vec3};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

/* Cast a ray inside of a system. */
fn cast_ray(rapier_context: Res<RapierContext>) {
    let ray_pos = Vec3::new(1.0, 2.0, 3.0);
    let ray_dir = Vec3::new(0.0, 1.0, 0.0);
    let max_toi = 4.0;
    let solid = true;
    let filter = QueryFilter::default();

    if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = ray_pos + ray_dir * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);
    }
}
