use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec3},
    transform::components::Transform,
};
use geometry_type::GeometryType;

use serde::{Deserialize, Serialize};

use self::spiral::odr_spiral;

pub mod arc;
pub mod geometry_type;
pub mod line;
pub mod plan_view;
pub mod spiral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// Start orientation (inertial heading)
    #[serde(rename = "@hdg")]
    pub hdg: f32,
    /// Length of the element's reference line
    #[serde(rename = "@length")]
    pub length: f32,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f32,
    /// Start position (x inertial)
    #[serde(rename = "@x")]
    pub x: f32,
    /// Start position (y inertial)
    #[serde(rename = "@y")]
    pub y: f32,

    #[serde(flatten)]
    pub r#type: GeometryType,
}

impl Geometry {
    pub(crate) fn interpolate(&self, s: f32) -> Transform {
        match &self.r#type {
            GeometryType::Line(_) => {
                let (sin_hdg, cos_hdg) = self.hdg.sin_cos();
                let x = (cos_hdg * (s - self.s)) + self.x;
                let y = (sin_hdg * (s - self.s)) + self.y;

                let translation = Vec3::new(x, 0.0, y);
                let rotation = Quat::from_axis_angle(Vec3::Y, self.hdg);

                Transform::from_translation(translation).with_rotation(rotation)
            }
            GeometryType::Spiral(spiral) => {
                let curvature = (spiral.curvature_end - spiral.curvature_start) / self.length;
                let s_spiral = spiral.curvature_start / curvature;
                let (x0_spiral, y0_spiral, a0_spiral) = odr_spiral(s_spiral, curvature);

                let (xs_spiral, ys_spiral, as_spiral) =
                    odr_spiral(s - self.s + s_spiral, curvature);
                let hdg = self.hdg - a0_spiral;
                let (s_hdg, c_hdg) = hdg.sin_cos();
                let x =
                    (c_hdg * (xs_spiral - x0_spiral)) - (s_hdg * (ys_spiral - y0_spiral)) + self.x;
                let y =
                    (s_hdg * (xs_spiral - x0_spiral)) + (c_hdg * (ys_spiral - y0_spiral)) + self.y;

                let rotation = Quat::from_axis_angle(Vec3::Y, as_spiral + self.hdg - a0_spiral);

                Transform::from_translation(Vec3::new(x, 0.0, y)).with_rotation(rotation)
            }
            GeometryType::Arc(a) => {
                let angle_at_s = (s - self.s) * a.curvature - PI * 0.5;
                let r = a.curvature.recip();
                let (sin, cos) = (angle_at_s + self.hdg).sin_cos();
                let x = r * (cos - self.hdg.sin()) + self.x;
                let y = r * (sin + self.hdg.cos()) + self.y;

                let rotation = Quat::from_axis_angle(Vec3::Y, angle_at_s);

                Transform::from_translation(Vec3::new(x, 0.0, y)).with_rotation(rotation)
            }
        }
    }
}
