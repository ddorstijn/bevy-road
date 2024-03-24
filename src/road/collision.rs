use bevy::{math::bounding::IntersectsVolume, prelude::*};

use super::{arc::ArcEdge, line::LineEdge, RoadEdge, ROAD_WIDTH};

trait EdgeCollision {
    fn collides_arc(&self, other: &ArcEdge) -> bool;
    fn collides_line(&self, other: &LineEdge) -> bool;

    fn intersects_arc(&self, other: &ArcEdge) -> (Option<Vec2>, Option<Vec2>);
    fn intersects_line(&self, other: &LineEdge) -> (Option<Vec2>, Option<Vec2>);
}

impl EdgeCollision for ArcEdge {
    fn collides_arc(&self, other: &ArcEdge) -> bool {
        if !self.aabb3().intersects(&other.aabb3()) {
            return false;
        }

        let dist = self.center().distance(other.center());
        let half_width_self = self.lanes() as f32 * ROAD_WIDTH * 0.5;
        let half_width_other = other.lanes() as f32 * ROAD_WIDTH * 0.5;
        let (self_radius, other_radius) = match dist < (self.radius() - other.radius()).abs() {
            true => match self.radius() > other.radius() {
                true => (
                    self.radius() - half_width_self,
                    other.radius() + half_width_other,
                ),
                false => (
                    self.radius() + half_width_self,
                    other.radius() - half_width_other,
                ),
            },
            false => (
                self.radius() + half_width_self,
                other.radius() + half_width_other,
            ),
        };

        if let Some((i1, i2)) =
            circle_circle_intersections(self.center(), self_radius, other.center(), other_radius)
        {
            if self.intersects_point(i1) && other.intersects_point(i1) {
                return true;
            }

            if self.intersects_point(i2) && other.intersects_point(i2) {
                return true;
            }
        }

        false
    }

    fn collides_line(&self, other: &LineEdge) -> bool {
        todo!()
    }

    fn intersects_arc(&self, other: &ArcEdge) -> (Option<Vec2>, Option<Vec2>) {
        if let Some((i1, i2)) = circle_circle_intersections(
            self.center(),
            self.radius(),
            other.center(),
            other.radius(),
        ) {
            let s1 = match self.intersects_point(i1) && other.intersects_point(i1) {
                true => Some(i1),
                false => None,
            };

            let s2 = match self.intersects_point(i2) && other.intersects_point(i2) {
                true => Some(i2),
                false => None,
            };

            return (s1, s2);
        }

        (None, None)
    }

    fn intersects_line(&self, other: &LineEdge) -> (Option<Vec2>, Option<Vec2>) {
        todo!()
    }
}

impl EdgeCollision for LineEdge {
    fn collides_arc(&self, other: &ArcEdge) -> bool {
        todo!()
    }

    fn collides_line(&self, other: &LineEdge) -> bool {
        let width1 = self.lanes() as f32 * ROAD_WIDTH * 0.5;
        let width2 = other.lanes() as f32 * ROAD_WIDTH * 0.5;
        (ray_ray_distance(self, other) - (width1 - width2)).is_sign_positive()
    }

    fn intersects_arc(&self, other: &ArcEdge) -> (Option<Vec2>, Option<Vec2>) {
        todo!()
    }

    fn intersects_line(&self, other: &LineEdge) -> (Option<Vec2>, Option<Vec2>) {
        todo!()
    }
}

fn ray_ray_distance(self_edge: &LineEdge, other: &LineEdge) -> f32 {
    // Step 1: Calculate intersection point of lines
    let intersection = line_intersection(self_edge, other);

    if let Some(point) = intersection {
        if is_point_on_ray(self_edge, point) && is_point_on_ray(other, point) {
            return 0.0;
        }
    }

    let dist1 = match is_point_on_ray(self_edge, other.start()) {
        true => Some(point_to_line_distance(self_edge, other.start())),
        false => None,
    };

    let dist2 = match is_point_on_ray(self_edge, other.end()) {
        true => Some(point_to_line_distance(self_edge, other.end())),
        false => None,
    };

    let dist3 = match is_point_on_ray(other, self_edge.start()) {
        true => Some(point_to_line_distance(other, self_edge.start())),
        false => None,
    };

    let dist4 = match is_point_on_ray(other, self_edge.end()) {
        true => Some(point_to_line_distance(other, self_edge.end())),
        false => None,
    };

    let distances = [dist1, dist2, dist3, dist4]
        .iter()
        .filter_map(|x| *x)
        .collect::<Vec<f32>>();

    match distances.is_empty() {
        false => *distances.iter().min_by(|a, b| a.total_cmp(b)).unwrap(),
        true => {
            let dist1 = (self_edge.start() - other.start()).length_squared();
            let dist2 = (self_edge.start() - other.end()).length_squared();
            let dist3 = (self_edge.end() - other.start()).length_squared();
            let dist4 = (self_edge.end() - other.end()).length_squared();

            [dist1, dist2, dist3, dist4]
                .iter()
                .min_by(|a, b| a.total_cmp(b))
                .unwrap()
                .sqrt()
        }
    }
}

fn line_intersection(r1: &LineEdge, r2: &LineEdge) -> Option<Vec2> {
    let denominator = r1.tangent().perp_dot(r2.tangent());

    if denominator.abs() < 1e-6 {
        return None; // Lines are parallel, no intersection
    }

    let t = (r2.start() - r1.start()).perp_dot(r2.tangent()) / denominator;

    Some(r1.start() + t * r1.tangent())
}

#[inline]
fn point_to_line_distance(line: &LineEdge, point: Vec2) -> f32 {
    line.tangent().perp_dot(point - line.start()).abs()
}

fn is_point_on_ray(ray: &LineEdge, point: Vec2) -> bool {
    let t = ray.tangent().dot(point - ray.start());

    t >= 0.0 && t <= ray.length()
}

fn circle_circle_intersections(
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
