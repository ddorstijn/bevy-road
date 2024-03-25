use crate::lane::lane_section::LaneSection;
use crate::lane::offset::Offset;
use serde::{Deserialize, Serialize};
use vec1::Vec1;

/// Contains a series of lane section elements that define the characteristics of the road cross
/// sections with respect to the lanes along the reference line.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lanes {
    #[serde(default)]
    pub lane_offset: Vec<Offset>,
    pub lane_section: Vec1<LaneSection>,
}
