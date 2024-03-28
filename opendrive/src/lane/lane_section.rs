use serde::{Deserialize, Serialize};

use crate::lane::center::Center;
use crate::lane::left::Left;
use crate::lane::right::Right;

/// Lanes may be split into multiple lane sections. Each lane section contains a fixed number of
/// lanes. Every time the number of lanes changes, a new lane section is required. The distance
/// between two succeeding lane sections shall not be zero.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LaneSection {
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f32,
    /// Lane section element is valid for one side only (left, center, or right), depending on the
    /// child elements.
    #[serde(rename = "@singleSide")]
    pub single_side: Option<bool>,
    pub left: Option<Left>,
    pub center: Center,
    pub right: Option<Right>,
}
