use bevy::prelude::*;

pub struct BiArc {
    pub center: Vec3,
    pub axis1: Vec3,
    pub axis2: Vec3,
    pub radius: f32,
    pub angle: f32,
    pub length: f32,
}

impl BiArc {
    fn new(tangent: Vec3, point: Vec3, point_to_mid: Vec3) {
        if !tangent.is_normalized() {
            tangent.normalize();
        }

        let normal = point_to_mid.cross(tangent);
        let perpendicular = tangent.cross(normal);
        let denominator = 2.0 * perpendicular.dot(point_to_mid);

        let center_dist = point_to_mid.dot(point_to_mid) / denominator;
        let center = point + perpendicular * center_dist;

        let radius = center_dist.abs() * perpendicular.length();

        let curvature = 1.0 / radius;
    }
}
