use std::{collections::BTreeMap, f64::consts::PI};

use crate::{
    geometry::{Geometry, GeometryType},
    lane::LaneSection,
    Polynomal,
};

use bevy::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Component, Debug, Default)]
pub struct Road {
    pub length: OrderedFloat<f64>,
    pub offset: BTreeMap<OrderedFloat<f64>, Polynomal>,
    pub elevation: BTreeMap<OrderedFloat<f64>, Polynomal>,
    pub sections: BTreeMap<OrderedFloat<f64>, LaneSection>,
    pub reference_line: BTreeMap<OrderedFloat<f64>, Geometry>,
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
                    OrderedFloat::<f64>::from(o.s),
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

        let sections = r
            .lanes
            .lane_section
            .iter()
            .map(|ls| (OrderedFloat::<f64>::from(ls.s), LaneSection::from(ls)))
            .collect();

        let reference_line = r
            .plan_view
            .geometry
            .iter()
            .map(|g| (OrderedFloat::<f64>::from(g.s), Geometry::from(g)))
            .collect();

        Road {
            length: r.length.into(),
            offset,
            sections,
            reference_line,
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
    pub fn interpolate(&self, s: OrderedFloat<f64>) -> (f64, f64, f64, f64) {
        let geom = self.reference_line.range(..=s).next_back().unwrap().1;
        let (x, y, hdg) = geom.interpolate(*s - geom.s);

        let z = self
            .elevation
            .range(..=s)
            .next_back()
            .and_then(|e| Some(e.1.eval(*s)))
            .unwrap_or_default();

        (x, y, z, hdg)
    }

    fn maxima(&self, s: OrderedFloat<f64>) -> [Vec3; 2] {
        let (x, neg_z, y, hdg) = self.interpolate(s);
        let transform = Transform::from_xyz(x as f32, y as f32, -neg_z as f32)
            .with_rotation(Quat::from_axis_angle(Vec3::Y, (hdg - PI * 0.5) as f32));

        let (s_section, section) = self.sections.range(..=s).next_back().unwrap();

        let left_point = section
            .left_lanes
            .values()
            .map(|lane| {
                let (s_width, width) = lane.width.range(..=s - s_section).next_back().unwrap();

                width.eval((s - s_section - s_width).0)
            })
            .sum::<f64>();

        let right_point = section
            .right_lanes
            .values()
            .map(|lane| {
                let (s_width, width) = lane.width.range(..=s - s_section).next_back().unwrap();

                width.eval((s - s_section - s_width).0)
            })
            .sum::<f64>();

        let left_point = transform.translation + transform.left() * left_point as f32;
        let right_point = transform.translation + transform.right() * right_point as f32;

        [left_point, right_point]
    }
}

impl Meshable for Road {
    type Output = Mesh;

    fn mesh(&self) -> Self::Output {
        let mut positions: Vec<Vec3> = self
            .reference_line
            .iter()
            .flat_map(|(s, g)| match g.r#type {
                GeometryType::Line => self.maxima(*s).to_vec(),
                GeometryType::Arc { .. } => {
                    let steps = g.length.round();
                    let step_size = g.length / steps;
                    (0..=steps as u32)
                        .flat_map(|step| self.maxima(*s + step_size * step as f64))
                        .collect::<Vec<Vec3>>()
                }
                GeometryType::Spiral { .. } => {
                    let steps = g.length.round();
                    let step_size = g.length / steps;
                    (0..=steps as u32)
                        .flat_map(|step| self.maxima(*s + step_size * step as f64))
                        .collect::<Vec<Vec3>>()
                }
            })
            .collect();

        positions.extend_from_slice(&self.maxima(self.length));

        let normals = (0..positions.len() as u32)
            .map(|_| Vec3::Y)
            .collect::<Vec<_>>();
        let indices = bevy::render::mesh::Indices::U32(
            (0..positions.len() as u32 - 2)
                .step_by(2)
                .flat_map(|i| [i, i + 1, i + 2, i + 1, i + 3, i + 2])
                .collect::<Vec<u32>>(),
        );

        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            bevy::render::render_asset::RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(indices)
    }
}
