use std::f32::consts::TAU;

use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb};

use super::ROAD_WIDTH;

#[derive(Debug, Default, Reflect, Clone, Copy)]
pub enum Twist {
    #[default]
    CounterClockwise,
    Clockwise,
    Straight,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct RoadEdge {
    start: Transform,
    end: Transform,
    center: Vec3,
    radius: f32,
    angle: f32,
    twist: Twist,
    lanes: u8,
}

impl RoadEdge {
    pub fn new(
        start: Transform,
        end: Transform,
        center: Vec3,
        radius: f32,
        angle: f32,
        twist: Twist,
        lanes: u8,
    ) -> Self {
        Self {
            start,
            end,
            center,
            angle,
            radius,
            twist,
            lanes,
        }
    }

    pub fn from_start_end(start: Transform, end: Vec3, lanes: u8) -> Self {
        let startpoint = start.translation.xz();
        let endpoint = end.xz();
        let chord = endpoint - startpoint;
        let tangent = start.forward().xz();
        let normal = start.left().xz();

        let scalar = chord.dot(normal);
        if scalar.abs() < f32::EPSILON {
            let center = (end - start.translation) / 2.0;
            let end = Transform::from_translation(end).looking_to(start.forward().into(), Vec3::Y);
            let radius = (endpoint - startpoint).length();
            return Self {
                start,
                end,
                center,
                radius,
                angle: 0.0,
                twist: Twist::Straight,
                lanes,
            };
        }

        let radius = chord.length_squared() / (2.0 * scalar);
        let center = startpoint + normal * radius;
        let radius = radius.abs();

        let c_start = startpoint - center;
        let c_end = endpoint - center;

        let twist = match c_start.perp_dot(tangent).is_sign_negative() {
            true => Twist::Clockwise,
            false => Twist::CounterClockwise,
        };

        // map [-π, π] to [0, 2π]
        let mut angle = c_start.angle_between(c_end).rem_euclid(TAU);
        if let Twist::Clockwise = twist {
            angle = TAU - angle;
        };

        let center = center.extend(0.0).xzy();

        let end_direction = match twist {
            Twist::Clockwise => -c_end.perp().extend(0.0).xzy(),
            Twist::CounterClockwise => c_end.perp().extend(0.0).xzy(),
            Twist::Straight => start.forward().into(),
        };

        let end = Transform::from_translation(end).looking_to(end_direction, Vec3::Y);

        RoadEdge {
            start,
            end,
            center,
            radius,
            angle,
            twist,
            lanes,
        }
    }

    pub fn rotation(&self) -> Vec2 {
        let midpoint = (self.start.translation + self.end.translation) * 0.5;
        let c_midpoint = midpoint - self.center;
        if c_midpoint.length_squared() < f32::EPSILON {
            return self.start.forward().xz();
        }

        let c_end = self.end.translation - self.center;

        let inverse = c_end.dot(self.start.forward().into()).signum();
        // Center to midpoint on chord. Which is also the center of the circle. Normalized, this gives sine cosine of angle.
        // If c_end is pointing opposite of the tangent the center of the circle is opposite
        (c_midpoint * inverse).xz().normalize()
    }

    pub fn get_end_transform(&self, lane: Option<u8>) -> Transform {
        match lane {
            Some(l) => self
                .end
                .with_translation(self.end.translation + self.end.left() * ROAD_WIDTH * l as f32),
            None => self.end,
        }
    }

    pub fn length(&self) -> f32 {
        match self.twist {
            Twist::Straight => self.radius,
            _ => self.radius * self.angle,
        }
    }

    pub fn resize(&mut self, angle: f32) {
        let new_end = self.interpolate(self.radius * angle);
        self.angle = angle;
        self.end = new_end;
    }

    pub fn coord_to_angle(&self, coord: Vec3) -> f32 {
        let c_coord = coord - self.center;
        let mut angle = (self.start.translation - self.center)
            .angle_between(c_coord)
            .rem_euclid(TAU);

        if let Twist::Clockwise = self.twist {
            angle = TAU - angle;
        };

        angle
    }

    pub fn coord_to_length(&self, coord: Vec3) -> f32 {
        self.radius * self.coord_to_angle(coord)
    }

    pub fn interpolate(&self, length: f32) -> Transform {
        let angle = match self.twist {
            Twist::Straight => {
                return self
                    .start
                    .with_translation(self.start.translation + self.start.forward() * length)
            }
            Twist::Clockwise => -length / self.radius,
            Twist::CounterClockwise => length / self.radius,
        };

        let mut rotated = self.start.clone();
        rotated.rotate_around(self.center, Quat::from_axis_angle(Vec3::Y, angle));

        rotated
    }

    pub fn aabb(&self) -> Aabb {
        let r = self.radius + self.lanes as f32 * ROAD_WIDTH;

        Aabb {
            center: self.center.into(),
            half_extents: Vec3A::new(r, 0.1, r),
        }
    }

    pub fn check_hit(&self, hitpoint: Vec3) -> bool {
        let local = self
            .start
            .compute_matrix()
            .inverse()
            .transform_point(hitpoint);

        match self.twist {
            Twist::Straight => {
                if local.x.abs() > self.lanes as f32 * ROAD_WIDTH * 0.5 {
                    return false;
                }

                if local.z > 0.0 || local.z > -self.length() {
                    return false;
                }

                true
            }
            _ => {
                let length = (local - self.center).length();
                let road_thickness = self.lanes as f32 * ROAD_WIDTH * 0.5;

                if length < self.radius.abs() - road_thickness {
                    return false;
                }

                if length > self.radius.abs() + road_thickness {
                    return false;
                }

                true
            }
        }
    }

    pub fn start(&self) -> Transform {
        self.start
    }

    pub fn end(&self) -> Transform {
        self.end
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn lanes(&self) -> u8 {
        self.lanes
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn twist(&self) -> Twist {
        self.twist
    }
}
