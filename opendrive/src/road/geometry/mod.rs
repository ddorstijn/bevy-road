use geometry_type::GeometryType;

use serde::{Deserialize, Serialize};
use uom::si::f64::{Angle, Length};

pub mod arc;
pub mod geometry_type;
pub mod line;
pub mod plan_view;
pub mod spiral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// Start orientation (inertial heading)
    #[serde(rename = "@hdg")]
    pub hdg: Angle,
    /// Length of the element's reference line
    #[serde(rename = "@length")]
    pub length: Length,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: Length,
    /// Start position (x inertial)
    #[serde(rename = "@x")]
    pub x: Length,
    /// Start position (y inertial)
    #[serde(rename = "@y")]
    pub y: Length,

    #[serde(flatten)]
    pub r#type: GeometryType,
}
