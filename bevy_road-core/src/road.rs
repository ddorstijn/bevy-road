use std::collections::BTreeMap;

use crate::{lane::LaneSection, reference_line::ReferenceLine, Polynomal};

use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Road {
    pub length: OrderedFloat<f32>,
    pub offsets: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub reference_line: BTreeMap<OrderedFloat<f32>, ReferenceLine>,
    pub sections: BTreeMap<OrderedFloat<f32>, LaneSection>,
    pub predecessor: Option<u32>,
    pub sucessor: Option<u32>,
}

// impl Interpolatable for Road {
//     fn interpolate(&self, s: f32) -> Transform {
//         let geom = match self.plan_view.geometry.len() == 1 {
//             true => self.plan_view.geometry.first().unwrap(),
//             false => match self
//                 .plan_view
//                 .geometry
//                 .windows(2)
//                 .find(|w| w[0].s <= s && w[1].s > s)
//             {
//                 Some(x) => x.first().unwrap(),
//                 None => self.plan_view.geometry.last().unwrap(),
//             },
//         };

//         geom.interpolate(s)
//     }
// }
