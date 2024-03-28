use std::{collections::BTreeMap, sync::Arc};

use opendrive::lane::lane_type::LaneType;
use ordered_float::OrderedFloat;

use crate::Polynomal;

#[derive(Debug)]
pub struct LaneSection {
    pub lanes: BTreeMap<OrderedFloat<f32>, Lane>,
}

#[derive(Debug)]
pub struct Lane {
    pub width: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub height: BTreeMap<OrderedFloat<f32>, (f32, f32)>,
    pub r#type: LaneType,
    pub predecessor: Option<Arc<Lane>>,
    pub successor: Option<Arc<Lane>>,
    pub left_neighbour: Option<Arc<Lane>>,
    pub right_neighbour: Option<Arc<Lane>>,
}
