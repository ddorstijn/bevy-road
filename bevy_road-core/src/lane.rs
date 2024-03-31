use std::collections::BTreeMap;

use opendrive::lane::lane_type::LaneType;
use ordered_float::OrderedFloat;

use crate::Polynomal;

#[derive(Debug)]
pub struct LaneSection {
    pub lanes: BTreeMap<i32, Lane>,
}

#[derive(Debug, Default)]
pub struct Lane {
    pub width: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub height: BTreeMap<OrderedFloat<f32>, (f32, f32)>,
    pub r#type: LaneType,
    pub predecessor: Option<i32>,
    pub successor: Option<i32>,
}
