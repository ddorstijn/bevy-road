use std::{borrow::Borrow, collections::BTreeMap, path::Path};

use lane::{Lane, LaneSection};
use odr_spiral::odr_spiral;
use opendrive::core::OpenDrive;
use ordered_float::OrderedFloat;
use polynomal::Polynomal;
use reference_line::{GeometryType, ReferenceLine};
use road::Road;

pub mod lane;
pub mod reference_line;
pub mod road;

mod odr_spiral;
mod polynomal;

#[derive(Debug)]
pub struct BevyRoad {
    pub name: String,
    pub version: String,

    pub roads: BTreeMap<u32, Road>,
}

impl From<OpenDrive> for BevyRoad {
    fn from(odr: OpenDrive) -> Self {
        let name = odr.header.name.unwrap_or("Untitled project".to_string());
        let version = odr.header.version.unwrap_or("0.01".to_string());

        let roads = BTreeMap::from_iter(odr.road.iter().map(|r| {
            let id: u32 = r.id.parse().unwrap_or(0);

            let offsets = BTreeMap::from_iter(r.lanes.lane_offset.iter().map(|o| {
                let id = OrderedFloat::<f32>::from(o.s);
                let offset = Polynomal::new(o.a, o.b, o.c, o.d);

                (id, offset)
            }));

            let reference_line = BTreeMap::from_iter(r.plan_view.geometry.iter().map(|g| {
                let id = OrderedFloat::<f32>::from(g.s);

                let r#type = match &g.r#type {
                    opendrive::road::geometry::geometry_type::GeometryType::Line(_) => {
                        GeometryType::Line
                    }
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
                    opendrive::road::geometry::geometry_type::GeometryType::Arc(a) => {
                        GeometryType::Arc {
                            curvature: a.curvature,
                        }
                    }
                };

                let geom = ReferenceLine {
                    hdg: g.hdg,
                    length: g.length,
                    x: g.x,
                    y: g.y,
                    r#type,
                };

                (id, geom)
            }));

            let sections = BTreeMap::from_iter(r.lanes.lane_section.iter().map(|ls| {
                let id = OrderedFloat::<f32>::from(ls.s);

                let center_lane_arr = [ls.center.lane.borrow()];

                let left_lanes = ls.left.as_ref().unwrap().lane.iter();
                let center_lane = center_lane_arr.into_iter();
                let right_lanes = ls.right.as_ref().unwrap().lane.iter();

                let lanes = BTreeMap::from_iter(
                    left_lanes.chain(center_lane).chain(right_lanes).map(|l| {
                        let id = l.id;

                        let widths = BTreeMap::from_iter(l.width.iter().map(|w| {
                            (
                                OrderedFloat::<f32>::from(w.s_offset),
                                Polynomal::new(w.a, w.b, w.c, w.d),
                            )
                        }));

                        let heights =
                            BTreeMap::from_iter(l.height.iter().map(|h| {
                                (OrderedFloat::<f32>::from(h.s_offset), (h.inner, h.outer))
                            }));

                        let lane = Lane {
                            r#type: l.r#type.clone(),
                            width: widths,
                            height: heights,
                            predecessor: l.link.as_ref().and_then(|link| {
                                link.predecessor.first().and_then(|prd| Some(prd.id))
                            }),
                            successor: l.link.as_ref().and_then(|link| {
                                link.successor.first().and_then(|scr| Some(scr.id))
                            }),
                        };

                        (id, lane)
                    }),
                );

                let section = LaneSection { lanes };

                (id, section)
            }));

            let road = Road {
                length: r.length.into(),
                offsets,
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
            };

            (id, road)
        }));

        Self {
            name,
            version,

            roads,
        }
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> BevyRoad {
    BevyRoad::from(opendrive::load_opendrive(path).unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {
        let odr = opendrive::load_opendrive("C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Line-Spiral-Arc.xodr").unwrap();
        let br = crate::BevyRoad::from(odr);

        println!("{:?}", br);
    }
}
