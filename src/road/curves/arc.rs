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

#[derive(Default, Debug)]
pub struct BiArc {
    pub arc1: Arc,
    pub arc2: Arc,
}

impl BiArc {
    pub fn new(t1: &Transform, t2: &Transform) -> Self {
        let dir = t2.translation - t1.translation;
        let tangent = t1.forward() + t2.forward();
        let dt = dir.dot(tangent);
        let angle_diff = t1.forward().dot(t2.forward());
        let denominator = 2.0 * (1.0 - angle_diff);

        let discriminant = dt * dt + denominator * dir.length_squared();
        let d = (-dt + discriminant.sqrt()) / denominator;

        let pm = (t2.translation + (t1.forward() - t2.forward()) * d + t1.translation) * 0.5;

        Self {
            arc1: Arc::new(t1.translation, pm, t1.forward(), d.is_sign_positive()),
            arc2: Arc::new(t2.translation, pm, t2.forward(), d.is_sign_positive()),
        }
    }

    pub fn length(&self) -> f32 {
        self.arc1.length + self.arc2.length
    }

    pub fn interpolate(&self, distance: f32) -> Transform {
        let (arc, dist) = match distance < self.arc1.length {
            true => (&self.arc1, distance),
            false => (&self.arc2, distance - self.arc1.length),
        };

        let mut t = Transform::from_translation(arc.center + arc.start);
        t.rotate_axis(Vec3::Y, dist);
        t
    }
}

impl Arc {
    fn new(start: Vec3, end: Vec3, tangent: Vec3, minor: bool) -> Self {
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
}
