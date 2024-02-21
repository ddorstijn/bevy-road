use bevy::prelude::*;

use super::edge::RoadEdge;

pub fn compute_biarc(
    start: GlobalTransform,
    end: GlobalTransform,
    lanes: u8,
) -> (RoadEdge, RoadEdge) {
    let midpoint = compute_midpoint(start, end);

    let start_local_mid = start
        .compute_matrix()
        .inverse()
        .transform_point(midpoint.translation);

    let edge1 = RoadEdge::new(start_local_mid, lanes);

    let mid_local_end = midpoint
        .compute_matrix()
        .inverse()
        .transform_point(end.translation());

    let edge2 = RoadEdge::new(mid_local_end, lanes);
    (edge1, edge2)
}

fn compute_midpoint(start: GlobalTransform, end: GlobalTransform) -> Transform {
    let startpoint = start.translation().xz();
    let endpoint = end.translation().xz();
    let start_tangent = start.forward().xz();
    let end_tangent = end.forward().xz();

    let v = endpoint - startpoint;

    // compute the denominator for the quadratic formula
    let t = start_tangent + end_tangent;

    let v_dot_t = v.dot(t);
    // abc-formula
    let denominator = 2.0 * (1.0 - start_tangent.dot(end_tangent));

    // if the special case d is infinity, the only solution is to
    // interpolate across two semicircles
    if denominator < 0.001 && v.dot(end_tangent).abs() < 0.001 {
        return Transform::from_translation(v.extend(0.0).xzy() * 0.5)
            .looking_to(v.perp().normalize().extend(0.0).xzy(), Vec3::Y);
    }

    // if the quadratic formula denominator is zero, the tangents are equal
    // and we need a special case
    let d = match denominator < 0.001 {
        true => v.length_squared() / (4.0 * v.dot(end_tangent)),
        false => {
            let discriminant = v_dot_t * v_dot_t + denominator * v.length_squared();
            // use the positive result of the quadratic formula
            (-v_dot_t + discriminant.sqrt()) / denominator
        }
    };

    // compute the connection point (i.e. the mid point)
    let midpoint = (endpoint + startpoint + (start_tangent - end_tangent) * d) * 0.5;

    let q2 = endpoint - d * end_tangent;

    Transform::from_translation(midpoint.extend(0.0).xzy())
        .looking_at(q2.extend(0.0).xzy(), Vec3::Y)
}
