use std::f32::consts::PI;

use bevy::{math::Vec3, transform::components::Transform};

use crate::odr_spiral::odr_spiral;

#[derive(Debug)]
pub enum GeometryType {
    Line,
    Spiral {
        k_start: f32,
        k_end: f32,

        dk: f32,
        s_offset: f32,
        x_offset: f32,
        y_offset: f32,
        a_offset: f32,
    },
    Arc {
        curvature: f32,
    },
}

#[derive(Debug)]
pub struct ReferenceLine {
    pub s: f32,
    pub hdg: f32,
    pub length: f32,
    pub x: f32,
    pub y: f32,
    pub r#type: GeometryType,
}

impl From<&opendrive::road::geometry::Geometry> for ReferenceLine {
    fn from(g: &opendrive::road::geometry::Geometry) -> Self {
        let r#type = match &g.r#type {
            opendrive::road::geometry::geometry_type::GeometryType::Line(_) => GeometryType::Line,
            opendrive::road::geometry::geometry_type::GeometryType::Spiral(s) => {
                let dk = (s.curvature_end - s.curvature_start) / g.length;
                let s0 = s.curvature_start / dk;
                let (x0, y0, a0) = odr_spiral(s0, dk);

                GeometryType::Spiral {
                    k_start: s.curvature_start,
                    k_end: s.curvature_end,
                    dk,
                    s_offset: s0,
                    x_offset: x0,
                    y_offset: y0,
                    a_offset: a0,
                }
            }
            opendrive::road::geometry::geometry_type::GeometryType::Arc(a) => GeometryType::Arc {
                curvature: a.curvature,
            },
        };

        ReferenceLine {
            s: g.s,
            hdg: g.hdg,
            length: g.length,
            x: g.x,
            y: g.y,
            r#type,
        }
    }
}

impl ReferenceLine {
    pub fn interpolate(&self, s: f32) -> Transform {
        match &self.r#type {
            GeometryType::Line => {
                let (sin_hdg, cos_hdg) = self.hdg.sin_cos();
                let x = (cos_hdg * (s - self.s)) + self.x;
                let y = (sin_hdg * (s - self.s)) + self.y;

                Transform::from_xyz(x, 0.0, -y)
                    .looking_to(Vec3::new(cos_hdg, 0.0, -sin_hdg), Vec3::Y)
            }
            GeometryType::Spiral {
                k_start: _,
                k_end: _,
                dk,
                s_offset,
                x_offset,
                y_offset,
                a_offset,
            } => {
                let (xs_spiral, ys_spiral, as_spiral) = odr_spiral(s - self.s + s_offset, *dk);
                let hdg = self.hdg - a_offset;
                let (s_hdg, c_hdg) = hdg.sin_cos();
                let x =
                    (c_hdg * (xs_spiral - x_offset)) - (s_hdg * (ys_spiral - y_offset)) + self.x;
                let y =
                    (s_hdg * (xs_spiral - x_offset)) + (c_hdg * (ys_spiral - y_offset)) + self.y;

                let hdg = as_spiral + hdg;

                Transform::from_xyz(x, 0.0, -y)
                    .looking_to(Vec3::new(hdg.cos(), 0.0, -hdg.sin()), Vec3::Y)
            }
            GeometryType::Arc { curvature } => {
                let angle_at_s = (s - self.s) * curvature - PI * 0.5;
                let r = curvature.recip();
                let x = r * ((self.hdg + angle_at_s).cos() - self.hdg.sin()) + self.x;
                let y = r * ((self.hdg + angle_at_s).sin() + self.hdg.cos()) + self.y;

                let delta = PI * 0.5 - (s - self.s) * curvature - self.hdg;

                Transform::from_xyz(x, 0.0, -y)
                    .looking_to(Vec3::new(delta.sin(), 0.0, -delta.cos()), Vec3::Y)
            }
        }
    }
}
