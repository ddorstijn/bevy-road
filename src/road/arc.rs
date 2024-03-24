use std::f32::consts::{PI, TAU};

use bevy::{math::bounding::Aabb3d, prelude::*};

use super::{RoadEdge, ROAD_WIDTH};

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq)]
pub enum Twist {
    #[default]
    CounterClockwise,
    Clockwise,
}

#[derive(Component)]
pub struct ArcEdge {
    center: Vec2,
    start: Vec2,
    tangent: Vec2,
    aperture: f32,
    radius: f32,
    length: f32,
    twist: Twist,
    lanes: u8,

    aabb3: Aabb3d,
}

impl ArcEdge {
    pub fn new(
        center: Vec2,
        start: Vec2,
        tangent: Vec2,
        aperture: f32,
        radius: f32,
        length: f32,
        twist: Twist,
        lanes: u8,

        aabb3: Aabb3d,
    ) -> Self {
        Self {
            center,
            start,
            tangent,
            aperture,
            length,
            radius,
            twist,
            lanes,
            aabb3,
        }
    }

    pub fn from_start_end(start: Vec2, tangent: Vec2, end: Vec2, lanes: u8) -> Result<Self, ()> {
        let chord = end - start;
        let normal = start.perp();

        let scalar = chord.dot(normal);
        // Straight line
        if scalar.abs() < 0.0001 {
            return Err(());
        }

        let radius = chord.length_squared() / (2.0 * scalar);
        let center = start + normal * radius;
        let radius = radius.abs();

        let c_start = start - center;
        let c_end = end - center;

        let twist = match c_start.perp_dot(tangent).is_sign_negative() {
            true => Twist::Clockwise,
            false => Twist::CounterClockwise,
        };

        let aperture = match twist {
            Twist::CounterClockwise => c_start.angle_between(c_end).rem_euclid(TAU),
            Twist::Clockwise => (-c_start.angle_between(c_end)).rem_euclid(TAU),
        };

        let length = aperture * radius;

        let aabb3 = compute_aabb3(start, end, center, radius, twist, lanes);

        Ok(ArcEdge {
            start,
            tangent,
            center,
            radius,
            length,
            aperture,
            twist,
            lanes,
            aabb3,
        })
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl RoadEdge for ArcEdge {
    fn interpolate(&self, length: f32, lane_offset: f32) -> Transform {
        let angle = match self.twist {
            Twist::Clockwise => -length / self.radius,
            Twist::CounterClockwise => length / self.radius,
        };

        let rot = Mat2::from_angle(angle);

        let pos = rot.mul_vec2(self.start).extend(0.0).xzy();
        let tangent = rot.mul_vec2(self.tangent).extend(0.0).xzy();

        Transform::from_translation(pos).looking_to(tangent, Vec3::Y)
    }

    fn coord_to_length(&self, coord: Vec2) -> f32 {
        let c_start = self.start - self.center;
        let c_coord = coord - self.center;

        match self.twist {
            Twist::CounterClockwise => c_start.angle_between(c_coord).rem_euclid(TAU) * self.radius,
            Twist::Clockwise => (-c_start.angle_between(c_coord)).rem_euclid(TAU) * self.radius,
        }
    }

    fn intersects_point(&self, point: bevy::prelude::Vec2) -> bool {
        if self.coord_to_length(point) > self.length {
            return false;
        }

        let road_thickness = self.lanes as f32 * ROAD_WIDTH * 0.5;
        let radius = self.center.distance(point);

        if radius < self.radius - road_thickness || radius > self.radius + road_thickness {
            return false;
        }

        true
    }

    fn length(&self) -> f32 {
        self.length
    }

    fn aabb3(&self) -> Aabb3d {
        self.aabb3
    }

    fn lanes(&self) -> u8 {
        self.lanes
    }

    fn resize(&mut self, length: f32) {
        self.length = length;
    }
}

#[rustfmt::skip]
fn compute_aabb3(start: Vec2, end: Vec2, center: Vec2, radius: f32, twist: Twist, lanes: u8) -> Aabb3d {
    let half_width = lanes as f32 * ROAD_WIDTH * 0.5;

    let c_min_x = center.x - radius - half_width;
    let c_max_x = center.x + radius + half_width;
    let c_min_z = center.y - radius - half_width;
    let c_max_z = center.y + radius + half_width;

    let s_angle = (start - center).to_angle().rem_euclid(TAU);
    let e_angle = (end - center).to_angle().rem_euclid(TAU);

    let q0: bool;
    let q1: bool;
    let q2: bool;
    let q3: bool;

    match twist {
        Twist::CounterClockwise => {
            q0 = s_angle >= e_angle;
            q1 = (s_angle - 0.5 * PI).rem_euclid(TAU) >= (e_angle - 0.5 * PI).rem_euclid(TAU);
            q2 = (s_angle - PI).rem_euclid(TAU) >= (e_angle - PI).rem_euclid(TAU);
            q3 = (s_angle - 1.5 * PI).rem_euclid(TAU) >= (e_angle - 1.5 * PI).rem_euclid(TAU);
        }
        Twist::Clockwise => {
            q0 = s_angle <= e_angle;
            q1 = (s_angle - 0.5 * PI).rem_euclid(TAU) <= (e_angle - 0.5 * PI).rem_euclid(TAU);
            q2 = (s_angle - PI).rem_euclid(TAU) <= (e_angle - PI).rem_euclid(TAU);
            q3 = (s_angle - 1.5 * PI).rem_euclid(TAU) <= (e_angle - 1.5 * PI).rem_euclid(TAU);
        }
    }

    let max_x = if q0 { c_max_x } else { start.x.max(end.x) + half_width };
    let max_z = if q1 { c_max_z } else { start.y.max(end.y) + half_width };
    let min_x = if q2 { c_min_x } else { start.x.min(end.x) - half_width };
    let min_z = if q3 { c_min_z } else { start.y.min(end.y) - half_width };

    Aabb3d {
        min: Vec3::new(min_x, -0.1, min_z),
        max: Vec3::new(max_x, 0.1, max_z),
    }
}
