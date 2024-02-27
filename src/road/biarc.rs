use bevy::prelude::*;

use super::edge::RoadEdge;

pub fn compute_biarc(
    start: GlobalTransform,
    end: GlobalTransform,
    lanes: u8,
) -> (RoadEdge, Transform, RoadEdge) {
    let midpoint = compute_midpoint(start, end);

    let start_local_mid = start.compute_matrix().inverse().transform_point(midpoint);

    let edge1 = RoadEdge::new(start_local_mid, lanes);

    let mid_transform = start.mul_transform(edge1.get_end_transform(None));

    let mid_local_end = mid_transform
        .compute_matrix()
        .inverse()
        .transform_point(end.translation());

    let edge2 = RoadEdge::new(mid_local_end, lanes);
    (edge1, mid_transform.compute_transform(), edge2)
}

fn compute_midpoint(start: GlobalTransform, end: GlobalTransform) -> Vec3 {
    let startpoint = start.translation().xz();
    let endpoint = end.translation().xz();
    let start_tangent = start.forward().xz();
    let end_tangent = end.forward().xz();

    let v = endpoint - startpoint;
    let v_dot_t = v.dot(start_tangent + end_tangent);

    // compute the denominator for the quadratic formula
    let denominator = 2.0 * (1.0 - start_tangent.dot(end_tangent));

    // if the special case d is infinity, the only solution is to
    // interpolate across two semicircles
    if denominator < 0.001 && v.dot(end_tangent).abs() < 0.001 {
        return v.extend(0.0).xzy() * 0.5;
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

    midpoint.extend(0.0).xzy()
}
