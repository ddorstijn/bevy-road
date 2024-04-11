use std::f32::consts::PI;

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
    pub fn interpolate(&self, s: f32) -> (f32, f32, f32) {
        match &self.r#type {
            GeometryType::Line => {
                let (sin_hdg, cos_hdg) = self.hdg.sin_cos();
                let x = (cos_hdg * (s - self.s)) + self.x;
                let y = (sin_hdg * (s - self.s)) + self.y;

                (x, y, self.hdg)
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

                (x, y, hdg)
            }
            GeometryType::Arc { curvature } => {
                let hdg = self.hdg + (s - self.s) * curvature;
                let o_hdg = hdg - PI * 0.5;
                let r = curvature.recip();
                let x = r * (o_hdg.cos() - self.hdg.sin()) + self.x;
                let y = r * (o_hdg.sin() + self.hdg.cos()) + self.y;

                (x, y, hdg)
            }
        }
    }
}
