use bevy::{math::bounding::Aabb3d, prelude::*};

use super::{RoadEdge, ROAD_WIDTH};

#[derive(Component)]
pub struct LineEdge {
    start: Vec2,
    tangent: Vec2,
    end: Vec2,
    length: f32,
    lanes: u8,

    aabb3: Aabb3d,
}

impl LineEdge {
    pub fn start(&self) -> Vec2 {
        self.start
    }
    pub fn tangent(&self) -> Vec2 {
        self.tangent
    }
    pub fn end(&self) -> Vec2 {
        self.end
    }
    pub fn length(&self) -> f32 {
        self.length
    }
    pub fn lanes(&self) -> u8 {
        self.lanes
    }

    pub fn aabb3(&self) -> Aabb3d {
        self.aabb3
    }
}

impl RoadEdge for LineEdge {
    fn interpolate(&self, length: f32, lane_offset: f32) -> Transform {
        let pos = self.start + self.tangent * length;

        Transform::from_xyz(pos.x, 0.0, pos.y).looking_to(self.tangent.extend(0.0).xzy(), Vec3::Y)
    }

    fn intersects_point(&self, point: Vec2) -> bool {
        let road_thickness = self.lanes as f32 * ROAD_WIDTH * 0.5;

        let projection_length = point.dot(self.tangent);
        if projection_length < -road_thickness || projection_length > self.length + road_thickness {
            return false;
        }

        let closest_point_on_line = projection_length * self.tangent;
        let vector_to_line = closest_point_on_line - point;
        let distance = vector_to_line.length();

        distance <= road_thickness
    }

    fn coord_to_length(&self, coord: Vec2) -> f32 {
        let coord = coord - self.start;
        coord.project_onto_normalized(self.tangent).length()
    }

    fn resize(&mut self, length: f32) {
        self.length = length;
    }

    fn aabb3(&self) -> Aabb3d {
        self.aabb3
    }

    fn length(&self) -> f32 {
        self.length
    }

    fn lanes(&self) -> u8 {
        self.lanes
    }
}
