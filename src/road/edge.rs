use std::f32::consts::{PI, TAU};

use bevy::{
    math::bounding::{Aabb3d, IntersectsVolume},
    prelude::*,
};

use super::ROAD_WIDTH;

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq)]
pub enum Twist {
    #[default]
    CounterClockwise,
    Clockwise,
    Straight,
}

#[derive(Component, Debug)]
pub struct RoadEdge {
    start: Transform,
    end: Transform,
    center: Vec3,
    radius: f32,
    length: f32,
    twist: Twist,
    lanes: u8,
    aabb3: Aabb3d,
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
        aabb3: Aabb3d,
    ) -> Self {
        Self {
            start,
            end,
            center,
            length,
            radius,
            twist,
            lanes,
            aabb3,
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

            let aabb3 = compute_aabb3(
                startpoint,
                endpoint,
                center.xz(),
                0.0,
                Twist::Straight,
                lanes,
            );

            return Self {
                start,
                end,
                center,
                radius: 0.0,
                length,
                twist: Twist::Straight,
                lanes,
                aabb3,
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

        let aabb3 = compute_aabb3(startpoint, endpoint, center, radius, twist, lanes);
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
            aabb3,
        }
    }

    pub fn get_end_transform(&self, lane: Option<u8>) -> Transform {
        match lane {
            Some(l) => {
                let max = (self.lanes - 1) as f32 * 0.5 * ROAD_WIDTH;
                let offset = max - l as f32 * ROAD_WIDTH;
                let translation = self.end.translation + *self.end.left() * offset;

                self.end.with_translation(translation)
            }
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
        let max = (self.lanes - 1) as f32 * 0.5 * ROAD_WIDTH;
        let offset = max - lane as f32 * ROAD_WIDTH;

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

    pub fn intersects_point(&self, hitpoint: Vec3) -> bool {
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

    pub fn intersects_edge(&self, other: &RoadEdge) -> bool {
        if !self.aabb3.intersects(&other.aabb3()) {
            return false;
        }

        // Arc - Arc
        if self.twist != Twist::Straight && other.twist() != Twist::Straight {
            if let Some((i1, i2)) = circle_intersections(
                self.center.xz(),
                self.radius,
                other.center().xz(),
                other.radius(),
            ) {
                if self.intersects_point(i1.extend(0.0).xzy())
                    && other.intersects_point(i1.extend(0.0).xzy())
                {
                    return true;
                }

                if self.intersects_point(i2.extend(0.0).xzy())
                    && other.intersects_point(i2.extend(0.0).xzy())
                {
                    return true;
                }
            }

            return false;
        }

        // Arc - Straight
        if self.twist == Twist::Straight && other.twist() != Twist::Straight
            || self.twist != Twist::Straight && other.twist() == Twist::Straight
        {
            return true;
        }

        // Straight - Straight
        if self.twist == Twist::Straight && other.twist() == Twist::Straight {
            return true;
        }

        true
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

    pub fn aabb3(&self) -> Aabb3d {
        self.aabb3
    }
}

#[rustfmt::skip]
fn compute_aabb3(start: Vec2, end: Vec2, center: Vec2, radius: f32, twist: Twist, lanes: u8) -> Aabb3d {
    let half_width = lanes as f32 * ROAD_WIDTH * 0.5;

    if Twist::Straight == twist {
        let min_x = start.x.min(end.x);
        let min_z = start.y.min(end.y);

        let extend_x = center.x - min_x + half_width;
        let extend_z = center.y - min_z + half_width;

        return Aabb3d::new(center.extend(0.0).xzy(), Vec3::new(extend_x, 0.1, extend_z));
    }

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
        Twist::Straight => panic!("Straight lines don't have angles"),
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

fn circle_intersections(
    c1_center: Vec2,
    c1_radius: f32,
    c2_center: Vec2,
    c2_radius: f32,
) -> Option<(Vec2, Vec2)> {
    let dir = c2_center - c1_center;
    let dist = dir.length();

    if dist > c1_radius + c2_radius {
        // No solutions, the circles are separate
        return None;
    }

    if dist < (c1_radius - c2_radius).abs() {
        // No solutions because one circle is contained within the other
        return None;
    }

    if dist == 0.0 && c1_radius == c2_radius {
        // Circles are coincident and there are an infinite number of solutions
        return None;
    }

    let dir_n = dir / dist;

    let center_chord = (c1_radius.powi(2) - c2_radius.powi(2) + dist.powi(2)) / (2.0 * dist);
    let half_length = (c1_radius.powi(2) - center_chord.powi(2)).sqrt();
    let mid = c1_center + center_chord * dir_n;

    let half_chord = half_length * dir_n.perp();
    let s1 = mid - half_chord;
    let s2 = mid + half_chord;

    Some((s1, s2))
}
