use serde::{Deserialize, Serialize};
use uom::si::f64::Length;

/// Lane height shall be defined along the h-coordinate. Lane height may be used to elevate a lane
/// independent from the road elevation. Lane height is used to implement small-scale elevation such
/// as raising pedestrian walkways. Lane height is specified as offset from the road (including
/// elevation, superelevation, shape) in z direction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Height {
    /// Inner offset from road level
    #[serde(rename = "@inner")]
    pub inner: Length,
    /// Outer offset from road level
    #[serde(rename = "@outer")]
    pub outer: Length,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    #[serde(rename = "@sOffset")]
    pub s_offset: Length,
}
