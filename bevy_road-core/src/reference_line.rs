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
    pub hdg: f32,
    pub length: f32,
    pub x: f32,
    pub y: f32,
    pub r#type: GeometryType,
}

// impl ReferenceLine {
//     pub(crate) fn interpolate(&self, s: f32) -> Transform {
//         match &self.r#type {
//             GeometryType::Line => {
//                 let (sin_hdg, cos_hdg) = self.hdg.sin_cos();
//                 let x = (cos_hdg * (s - self.s)) + self.x;
//                 let y = (sin_hdg * (s - self.s)) + self.y;

//                 let translation = Vec3::new(x, 0.0, y);
//                 let rotation = Quat::from_axis_angle(Vec3::Y, self.hdg);

//                 Transform::from_translation(translation).with_rotation(rotation)
//             }
//             GeometryType::Spiral(spiral) => {
//                 let curvature = (spiral.curvature_end - spiral.curvature_start) / self.length;
//                 let s_spiral = spiral.curvature_start / curvature;
//                 let (x0_spiral, y0_spiral, a0_spiral) = odr_spiral(s_spiral, curvature);

//                 let (xs_spiral, ys_spiral, as_spiral) =
//                     odr_spiral(s - self.s + s_spiral, curvature);
//                 let hdg = self.hdg - a0_spiral;
//                 let (s_hdg, c_hdg) = hdg.sin_cos();
//                 let x =
//                     (c_hdg * (xs_spiral - x0_spiral)) - (s_hdg * (ys_spiral - y0_spiral)) + self.x;
//                 let y =
//                     (s_hdg * (xs_spiral - x0_spiral)) + (c_hdg * (ys_spiral - y0_spiral)) + self.y;

//                 let rotation = Quat::from_axis_angle(Vec3::Y, as_spiral + self.hdg - a0_spiral);

//                 Transform::from_translation(Vec3::new(x, 0.0, y)).with_rotation(rotation)
//             }
//             GeometryType::Arc(a) => {
//                 let angle_at_s = (s - self.s) * a.curvature - PI * 0.5;
//                 let r = a.curvature.recip();
//                 let (sin, cos) = (angle_at_s + self.hdg).sin_cos();
//                 let x = r * (cos - self.hdg.sin()) + self.x;
//                 let y = r * (sin + self.hdg.cos()) + self.y;

//                 let rotation = Quat::from_axis_angle(Vec3::Y, angle_at_s);

//                 Transform::from_translation(Vec3::new(x, 0.0, y)).with_rotation(rotation)
//             }
//         }
//     }
// }
