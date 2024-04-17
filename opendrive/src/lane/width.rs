use serde::{Deserialize, Serialize};

/// The width of a lane is defined along the t-coordinate. The width of a lane may change within a lane section.
/// In ASAM OpenDRIVE, lane width is described by the `<width>` element within the `<lane>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Width {
    /// Polynom parameter a, width at @s (ds=0)
    #[serde(rename = "@a")]
    pub a: f64,
    /// Polynom parameter b
    #[serde(rename = "@b")]
    pub b: f64,
    /// Polynom parameter c
    #[serde(rename = "@c")]
    pub c: f64,
    /// Polynom parameter d
    #[serde(rename = "@d")]
    pub d: f64,
    /// s-coordinate of start position of the `<width>` element, relative to the position of the
    /// preceding `<laneSection>` element
    #[serde(rename = "@sOffset")]
    pub s_offset: f64,
}
