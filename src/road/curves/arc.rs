use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct Arc {
    pub start: Vec3,
    pub end: Vec3,
    pub center: Vec3,
    pub radius: f32,
    pub angle: f32,
    pub length: f32,
}

impl Arc {
    pub fn new(start: Vec3, end: Vec3, tangent: Vec3, minor: bool) -> Self {
        let chord = start - end;
        let up = chord.cross(tangent);
        let normal = tangent.cross(up);
        let twist = normal.dot(chord);
        let denominator = 2.0 * twist;

        let center_dist = chord.length_squared() / denominator;
        let center = start + normal * center_dist;

        let radius = center_dist.abs() * normal.length();
        let length = chord.length() / (2.0 * radius);
        let angle = match minor {
            true => length / radius,
            false => 2.0 * PI - length / radius,
        };

        Self {
            start: (start - center),
            end: (end - center),
            center,
            radius,
            angle,
            length,
        }
    }

    pub fn new2() {

    }
}
