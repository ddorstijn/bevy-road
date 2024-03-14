use std::f32::consts::{PI, TAU};

use bevy::{math::bounding::Aabb3d, prelude::*};

use super::ROAD_WIDTH;

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq)]
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
    length: f32,
    twist: Twist,
    lanes: u8,
}

impl RoadEdge {
    pub fn new(
        start: Transform,
        end: Transform,
        center: Vec3,
        radius: f32,
        length: f32,
        twist: Twist,
        lanes: u8,
    ) -> Self {
        Self {
            start,
            end,
            center,
            length,
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
        // Straight line
        if scalar.abs() < f32::EPSILON * 1000.0 {
            let center = start.translation + (end - start.translation) / 2.0;
            let end = Transform::from_translation(end).looking_to(start.forward().into(), Vec3::Y);
            let length = (endpoint - startpoint).length();
            return Self {
                start,
                end,
                center,
                radius: 0.0,
                length,
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

        let length = match twist {
            Twist::CounterClockwise => c_start.angle_between(c_end).rem_euclid(TAU) * radius,
            Twist::Clockwise => (-c_start.angle_between(c_end)).rem_euclid(TAU) * radius,
            Twist::Straight => unreachable!(),
        };

        let end_direction = match twist {
            Twist::CounterClockwise => c_end.perp().extend(0.0).xzy(),
            Twist::Clockwise => -c_end.perp().extend(0.0).xzy(),
            Twist::Straight => unreachable!(),
        };

        let center = center.extend(0.0).xzy();
        let end = Transform::from_translation(end).looking_to(end_direction, Vec3::Y);

        RoadEdge {
            start,
            end,
            center,
            radius,
            length,
            twist,
            lanes,
        }
    }

    pub fn rotation(&self) -> Vec2 {
        match self.twist {
            Twist::Straight => self.start.forward().xz(),
            _ => {
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
        }
    }

    pub fn get_end_transform(&self, lane: Option<u8>) -> Transform {
        match lane {
            Some(l) => self
                .end
                .with_translation(self.end.translation + self.end.left() * ROAD_WIDTH * l as f32),
            None => self.end,
        }
    }

    pub fn resize(&mut self, length: f32) {
        let new_end = self.interpolate(length);
        self.length = length;
        self.end = new_end;
    }

    pub fn coord_to_angle(&self, coord: Vec3) -> f32 {
        let c_start = self.start.translation.xz() - self.center.xz();
        let c_coord = coord.xz() - self.center.xz();

        match self.twist {
            Twist::CounterClockwise => c_start.angle_between(c_coord).rem_euclid(TAU),
            Twist::Clockwise => (-c_start.angle_between(c_coord)).rem_euclid(TAU),
            Twist::Straight => panic!("Straight roads shouldn't calculate angles"),
        }
    }

    pub fn coord_to_length(&self, coord: Vec3) -> f32 {
        match self.twist {
            Twist::Straight => coord
                .project_onto(self.end.translation - self.start.translation)
                .length(),
            _ => self.radius * self.coord_to_angle(coord),
        }
    }

    pub fn interpolate(&self, length: f32) -> Transform {
        let angle = match self.twist {
            Twist::Straight => {
                return self
                    .start
                    .with_translation(self.start.translation + self.start.forward() * length)
            }
            Twist::Clockwise => length / self.radius,
            Twist::CounterClockwise => -length / self.radius,
        };

        let mut rotated = self.start.clone();
        rotated.rotate_around(self.center, Quat::from_axis_angle(Vec3::Y, angle));

        rotated
    }

    pub fn interpolate_lane(&self, length: f32, lane: i32) -> Transform {
        let min = (self.lanes - 1) as f32 * 0.5 * ROAD_WIDTH;
        let offset = min + lane as f32 * ROAD_WIDTH;

        match self.twist {
            Twist::Straight => {
                let offset = self.start.left() * offset;

                let mut interp = self.start.clone();
                interp.translation += self.start.forward() * length + offset;

                interp
            }
            Twist::Clockwise => {
                let offset = self.start.left() * offset;
                let rotation = Quat::from_axis_angle(Vec3::Y, length / self.radius);

                let mut interp = self.start.clone();
                interp.translation += offset;
                interp.rotate_around(self.center, rotation);

                interp
            }
            Twist::CounterClockwise => {
                let offset = self.start.right() * offset;
                let rotation = Quat::from_axis_angle(Vec3::Y, -length / self.radius);

                let mut interp = self.start.clone();
                interp.translation += offset;
                interp.rotate_around(self.center, rotation);

                interp
            }
        }
    }

    #[rustfmt::skip]
    pub fn aabb3(&self) -> Aabb3d {
        let half_width = self.lanes as f32 * ROAD_WIDTH * 0.5;

        let s = self.start.translation.xz();
        let e = self.end.translation.xz();
        let c = self.center.xz();

        if Twist::Straight == self.twist {
            let min_x = s.x.min(e.x);
            let min_z = s.y.min(e.y);

            let extend_x = c.x - min_x + half_width;
            let extend_z = c.y - min_z + half_width;

            return Aabb3d::new(self.center.into(), Vec3::new(extend_x, 0.1, extend_z));
        }

        let c_min_x = c.x - self.radius - half_width;
        let c_max_x = c.x + self.radius + half_width;
        let c_min_z = c.y - self.radius - half_width;
        let c_max_z = c.y + self.radius + half_width;

        let s_angle = (s - c).to_angle().rem_euclid(TAU);
        let e_angle = (e - c).to_angle().rem_euclid(TAU);

        let q0: bool;
        let q1: bool;
        let q2: bool;
        let q3: bool;

        match self.twist {
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
            Twist::Straight => panic!("Straight lines don't have angles"),
        }

        let max_x = if q0 { c_max_x } else { s.x.max(e.x) + half_width };
        let max_z = if q1 { c_max_z } else { s.y.max(e.y) + half_width };
        let min_x = if q2 { c_min_x } else { s.x.min(e.x) - half_width };
        let min_z = if q3 { c_min_z } else { s.y.min(e.y) - half_width };

        Aabb3d {
            min: Vec3::new(min_x, -0.1, min_z),
            max: Vec3::new(max_x, 0.1, max_z),
        }
    }

    pub fn check_hit(&self, hitpoint: Vec3) -> bool {
        let road_thickness = self.lanes as f32 * ROAD_WIDTH * 0.5;

        match self.twist {
            Twist::Straight => {
                let line = (self.end.translation - self.start.translation).normalize();

                let projection_length = hitpoint.dot(line);
                if projection_length < 0.0 || projection_length > self.length {
                    return false;
                }

                let closest_point_on_line = projection_length * line;
                let vector_to_line = closest_point_on_line - hitpoint;
                let distance = vector_to_line.length();

                distance <= road_thickness
            }
            _ => {
                if self.coord_to_length(hitpoint) > self.length {
                    return false;
                }

                let radius = (hitpoint - self.center).length();
                if radius < self.radius - road_thickness || radius > self.radius + road_thickness {
                    return false;
                }

                true
            }
        }
    }

    // Properties
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

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn twist(&self) -> Twist {
        self.twist
    }
}
