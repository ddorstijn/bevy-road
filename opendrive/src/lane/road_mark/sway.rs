use serde::{Deserialize, Serialize};

/// Relocates the lateral reference position for the following (explicit) type definition and thus
/// defines an offset. The sway offset is relative to the nominal reference position of the lane
/// marking, meaning the lane border.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sway {
    /// Polynom parameter a, sway value at @s (ds=0)
    #[serde(rename = "@a")]
    a: f32,
    /// Polynom parameter b
    #[serde(rename = "@b")]
    b: f32,
    /// Polynom parameter c
    #[serde(rename = "@c")]
    c: f32,
    /// Polynom parameter d
    #[serde(rename = "@d")]
    d: f32,
    /// s-coordinate of start position of the `<sway>` element, relative to the @sOffset given in
    /// the `<roadMark>` element
    #[serde(rename = "@d_s")]
    d_s: f32,
}
