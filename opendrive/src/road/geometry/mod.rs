use geometry_type::GeometryType;

use serde::{Deserialize, Serialize};

pub mod arc;
pub mod geometry_type;
pub mod line;
pub mod plan_view;
pub mod spiral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// Start orientation (inertial heading)
    #[serde(rename = "@hdg")]
    pub hdg: f32,
    /// Length of the element's reference line
    #[serde(rename = "@length")]
    pub length: f32,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f32,
    /// Start position (x inertial)
    #[serde(rename = "@x")]
    pub x: f32,
    /// Start position (y inertial)
    #[serde(rename = "@y")]
    pub y: f32,

    #[serde(flatten)]
    pub r#type: GeometryType,
}
