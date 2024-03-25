use crate::lane::right_lane::RightLane;
use serde::{Deserialize, Serialize};
use vec1::Vec1;

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Right {
    pub lane: Vec1<RightLane>,
}
