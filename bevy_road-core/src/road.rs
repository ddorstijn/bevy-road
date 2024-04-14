use std::{collections::BTreeMap, f32::consts::PI};

use crate::{
    geometry::{Geometry, GeometryType},
    lane::LaneSection,
    Polynomal,
};

use bevy::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Component, Debug, Default)]
pub struct Road {
    pub length: OrderedFloat<f32>,
    pub offset: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub elevation: BTreeMap<OrderedFloat<f32>, Polynomal>,
    pub sections: BTreeMap<OrderedFloat<f32>, LaneSection>,
    pub reference_line: BTreeMap<OrderedFloat<f32>, Geometry>,
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

        let sections = r
            .lanes
            .lane_section
            .iter()
            .map(|ls| (OrderedFloat::<f32>::from(ls.s), LaneSection::from(ls)))
            .collect();

        let reference_line = r
            .plan_view
            .geometry
            .iter()
            .map(|g| (OrderedFloat::<f32>::from(g.s), Geometry::from(g)))
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
    pub fn interpolate(&self, s: OrderedFloat<f32>) -> (f32, f32, f32, f32) {
        let (x, y, hdg) = self
            .reference_line
            .range(..=s)
            .next_back()
            .unwrap()
            .1
            .interpolate(*s);

        let z = self
            .elevation
            .range(..=s)
            .next_back()
            .and_then(|e| Some(e.1.eval(*s)))
            .unwrap_or_default();

        (x, y, z, hdg)
    }
}

impl Meshable for Road {
    type Output = Mesh;

    fn mesh(&self) -> Self::Output {
        let mut v: Vec<_> = self
            .reference_line
            .iter()
            .flat_map(|(s, g)| match g.r#type {
                GeometryType::Line => vec![*s],
                GeometryType::Arc { .. } => {
                    let steps = g.length.round();
                    let step_size = g.length / steps;
                    (0..=steps as u32)
                        .map(|step| *s + step_size * step as f32)
                        .collect()
                }
                GeometryType::Spiral { .. } => {
                    let steps = g.length.round();
                    let step_size = g.length / steps;
                    (0..=steps as u32)
                        .map(|step| *s + step_size * step as f32)
                        .collect()
                }
            })
            .collect();

        v.push(self.length);

        let positions = v
            .into_iter()
            .flat_map(|road_s| {
                let (x, neg_z, y, hdg) = self.interpolate(road_s);
                let transform = Transform::from_xyz(x, y, -neg_z)
                    .with_rotation(Quat::from_axis_angle(Vec3::Y, hdg - PI * 0.5));

                let (s_section, section) = self.sections.range(..=road_s).next_back().unwrap();

                let left_point = section
                    .left_lanes
                    .values()
                    .map(|lane| {
                        let (s_width, width) =
                            lane.width.range(..=road_s - s_section).next_back().unwrap();

                        width.eval((road_s - s_section - s_width).0)
                    })
                    .sum::<f32>();

                let right_point = section
                    .right_lanes
                    .values()
                    .map(|lane| {
                        let (s_width, width) =
                            lane.width.range(..=road_s - s_section).next_back().unwrap();

                        width.eval((road_s - s_section - s_width).0)
                    })
                    .sum::<f32>();

                let left_point = transform.translation + transform.left() * left_point;
                let right_point = transform.translation + transform.right() * right_point;

                [left_point, right_point]
            })
            .collect::<Vec<_>>();

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
