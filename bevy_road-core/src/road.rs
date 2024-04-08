use std::collections::BTreeMap;

use crate::{lane::LaneSection, reference_line::ReferenceLine, Polynomal};

use bevy::{math::Vec3, transform::components::Transform};
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Road {
    pub length: OrderedFloat<f32>,
    pub offset: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub elevation: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub reference_line: BTreeMap<OrderedFloat<f32>, ReferenceLine>,
    pub sections: BTreeMap<OrderedFloat<f32>, LaneSection>,
    pub predecessor: Option<u32>,
    pub sucessor: Option<u32>,
}

impl From<&opendrive::road::Road> for Road {
    fn from(r: &opendrive::road::Road) -> Self {
        let offset = r
            .lanes
            .lane_offset
            .iter()
            .map(|o| {
                (
                    OrderedFloat::<f32>::from(o.s),
                    Polynomal::new(o.a, o.b, o.c, o.d),
                )
            })
            .collect();

        let elevation = r
            .elevation_profile
            .as_ref()
            .and_then(|ep| {
                Some(
                    ep.elevation
                        .iter()
                        .map(|e| (OrderedFloat(e.s), Polynomal::new(e.a, e.b, e.c, e.d)))
                        .collect(),
                )
            })
            .unwrap_or_default();

        let reference_line = r
            .plan_view
            .geometry
            .iter()
            .map(|g| (OrderedFloat::<f32>::from(g.s), ReferenceLine::from(g)))
            .collect();

        let sections = r
            .lanes
            .lane_section
            .iter()
            .map(|ls| (OrderedFloat::<f32>::from(ls.s), LaneSection::from(ls)))
            .collect();

        Road {
            length: r.length.into(),
            offset,
            reference_line,
            sections,
            predecessor: r.link.as_ref().and_then(|link| {
                link.predecessor
                    .as_ref()
                    .and_then(|prd| Some(prd.element_id.parse().unwrap()))
            }),
            sucessor: r.link.as_ref().and_then(|link| {
                link.successor
                    .as_ref()
                    .and_then(|scr| Some(scr.element_id.parse().unwrap()))
            }),
            elevation,
        }
    }
}

impl Road {
    pub fn interpolate(&self, s: OrderedFloat<f32>) -> Transform {
        let ref_line = self
            .reference_line
            .range(..=s)
            .next_back()
            .unwrap()
            .1
            .interpolate(*s);

        ref_line.with_translation(
            ref_line.translation
                + Vec3::Y
                    * self
                        .elevation
                        .range(..=s)
                        .next_back()
                        .and_then(|e| Some(e.1.eval(*s)))
                        .unwrap_or_default(),
        )
    }
}
