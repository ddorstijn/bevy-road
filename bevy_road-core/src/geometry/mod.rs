use std::f64::consts::PI;

use self::odr_spiral::odr_spiral;

mod odr_spiral;

#[derive(Debug, Clone)]
pub enum GeometryType {
    Line,
    Arc {
        k: f64,
    },
    Spiral {
        k_start: f64,
        k_end: f64,

        dk: f64,
        s_offset: f64,
        x_offset: f64,
        y_offset: f64,
        a_offset: f64,
    },
}

impl GeometryType {
    pub fn new_spiral(k_start: f64, k_end: f64, length: f64) -> Self {
        let dk = (k_end - k_start) / length;
        let s_offset = k_start / dk;
        let (x_offset, y_offset, a_offset) = odr_spiral(s_offset, dk);

        Self::Spiral {
            k_start,
            k_end,
            dk,
            s_offset,
            x_offset,
            y_offset,
            a_offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub s: f64,
    pub hdg: f64,
    pub length: f64,
    pub x: f64,
    pub y: f64,

    pub r#type: GeometryType,
}

impl Geometry {
    pub fn interpolate(&self, rel_s: f64) -> (f64, f64, f64) {
        match &self.r#type {
            GeometryType::Line => {
                let (sin_hdg, cos_hdg) = self.hdg.sin_cos();
                let x = (cos_hdg * rel_s) + self.x;
                let y = (sin_hdg * rel_s) + self.y;

                (x, y, self.hdg)
            }
            GeometryType::Arc { k } => {
                let hdg = self.hdg + rel_s * k;
                let o_hdg = hdg - PI * 0.5;
                let r = k.recip();
                let x = r * (o_hdg.cos() - self.hdg.sin()) + self.x;
                let y = r * (o_hdg.sin() + self.hdg.cos()) + self.y;

                (x, y, hdg)
            }
            GeometryType::Spiral {
                dk,
                s_offset,
                x_offset,
                y_offset,
                a_offset,
                ..
            } => {
                let (xs_spiral, ys_spiral, as_spiral) = odr_spiral(rel_s + s_offset, *dk);
                let hdg = self.hdg - a_offset;
                let x_spiral = xs_spiral - x_offset;
                let y_spiral = ys_spiral - y_offset;

                let (s_hdg, c_hdg) = hdg.sin_cos();
                let x = (c_hdg * x_spiral) - (s_hdg * y_spiral) + self.x;
                let y = (s_hdg * x_spiral) + (c_hdg * y_spiral) + self.y;

                let hdg = as_spiral + hdg;

                (x, y, hdg)
            }
        }
    }
}

impl From<&opendrive::road::geometry::Geometry> for Geometry {
    fn from(geometry: &opendrive::road::geometry::Geometry) -> Self {
        let s = geometry.s;
        let hdg = geometry.hdg;
        let length = geometry.length;
        let x = geometry.x;
        let y = geometry.y;

        let r#type = match &geometry.r#type {
            opendrive::road::geometry::geometry_type::GeometryType::Line(_) => GeometryType::Line,
            opendrive::road::geometry::geometry_type::GeometryType::Arc(arc) => {
                GeometryType::Arc { k: arc.curvature }
            }
            opendrive::road::geometry::geometry_type::GeometryType::Spiral(spiral) => {
                let dk = (spiral.curvature_end - spiral.curvature_start) / length;
                let s_offset = spiral.curvature_start / dk;
                let (x_offset, y_offset, a_offset) = odr_spiral(s_offset, dk);

                GeometryType::Spiral {
                    k_start: spiral.curvature_start,
                    k_end: spiral.curvature_end,

                    dk,
                    s_offset,
                    x_offset,
                    y_offset,
                    a_offset,
                }
            }
        };

        Self {
            s,
            hdg,
            length,
            x,
            y,
            r#type,
        }
    }
}
