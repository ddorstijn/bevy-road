use serde::{Deserialize, Serialize};

use super::Lane;

/// For easier navigation through an ASAM OpenDRIVE road description, the lanes within a lane
/// section are grouped into left, center, and right lanes. Each lane section shall contain one
/// `<center>` element and at least one `<right>` or `<left>` element.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Left {
    pub lane: Vec<Lane>,
}
