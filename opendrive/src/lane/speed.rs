use crate::road::unit::SpeedUnit;
use serde::{Deserialize, Serialize};

/// Defines the maximum allowed speed on a given lane. Each element is valid in direction of the
/// increasing s-coordinate until a new element is defined.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Speed {
    /// Maximum allowed speed. If the attribute unit is not specified, m/s is used as default.
    #[serde(rename = "@max")]
    pub max: f32,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    #[serde(rename = "@sOffset")]
    pub s_offset: f32,
    /// Unit of the attribute max
    #[serde(rename = "@unit")]
    pub unit: Option<SpeedUnit>,
}
