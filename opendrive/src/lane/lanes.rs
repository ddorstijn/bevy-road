use crate::lane::lane_section::LaneSection;
use crate::lane::offset::Offset;
use serde::{Deserialize, Serialize};

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lanes {
    #[serde(default)]
    pub lane_offset: Vec<Offset>,
    pub lane_section: Vec<LaneSection>,
}