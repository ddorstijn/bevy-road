use std::collections::BTreeMap;

use opendrive::lane::lane_type::LaneType;
use ordered_float::OrderedFloat;

use crate::Polynomal;

#[derive(Debug)]
pub struct LaneSection {
    pub left_lanes: BTreeMap<i32, Lane>,
    pub right_lanes: BTreeMap<i32, Lane>,
}

impl From<&opendrive::lane::lane_section::LaneSection> for LaneSection {
    fn from(ls: &opendrive::lane::lane_section::LaneSection) -> Self {
        let left_lanes = ls.left.as_ref().map_or_else(BTreeMap::new, |left_lanes| {
            left_lanes
                .lane
                .iter()
                .map(|l| (l.id, Lane::from(l)))
                .collect()
        });

        let right_lanes = ls.right.as_ref().map_or_else(BTreeMap::new, |right_lanes| {
            right_lanes
                .lane
                .iter()
                .map(|l| (l.id, Lane::from(l)))
                .collect()
        });

        LaneSection {
            left_lanes,
            right_lanes,
        }
    }
}

#[derive(Debug, Default)]
pub struct Lane {
    pub width: BTreeMap<OrderedFloat<f64>, Polynomal>,
    pub height: BTreeMap<OrderedFloat<f64>, (f64, f64)>,
    pub r#type: LaneType,
    pub predecessor: Option<i32>,
    pub successor: Option<i32>,
}

impl From<&opendrive::lane::Lane> for Lane {
    fn from(lane: &opendrive::lane::Lane) -> Self {
        let widths = BTreeMap::from_iter(lane.width.iter().map(|w| {
            (
                OrderedFloat::<f64>::from(w.s_offset),
                Polynomal::new(w.a, w.b, w.c, w.d),
            )
        }));

        let heights = BTreeMap::from_iter(
            lane.height
                .iter()
                .map(|h| (OrderedFloat::<f64>::from(h.s_offset), (h.inner, h.outer))),
        );

        Lane {
            r#type: lane.r#type.clone(),
            width: widths,
            height: heights,
            predecessor: lane
                .link
                .as_ref()
                .and_then(|link| link.predecessor.first().and_then(|prd| Some(prd.id))),
            successor: lane
                .link
                .as_ref()
                .and_then(|link| link.successor.first().and_then(|scr| Some(scr.id))),
        }
    }
}
