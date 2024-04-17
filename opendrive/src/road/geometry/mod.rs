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
    pub hdg: f64,
    /// Length of the element's reference line
    #[serde(rename = "@length")]
    pub length: f64,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f64,
    /// Start position (x inertial)
    #[serde(rename = "@x")]
    pub x: f64,
    /// Start position (y inertial)
    #[serde(rename = "@y")]
    pub y: f64,

    #[serde(flatten)]
    pub r#type: GeometryType,
}
