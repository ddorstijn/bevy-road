use bevy::prelude::*;

use super::edge::RoadEdge;

pub fn compute_biarc(start: Transform, end: Transform, lanes: u8) -> (RoadEdge, RoadEdge) {
    let midpoint = compute_midpoint(start, end);

    let edge1 = RoadEdge::from_start_end(start, midpoint, lanes);

    let mid_transform = edge1.end();
    let edge2 = RoadEdge::from_start_end(mid_transform, end.translation, lanes);
    (edge1, edge2)
}

fn compute_midpoint(start: Transform, end: Transform) -> Vec3 {
    let startpoint = start.translation.xz();
    let endpoint = end.translation.xz();
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
