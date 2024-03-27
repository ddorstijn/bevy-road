use serde::{Deserialize, Serialize};

/// A lane offset may be used to shift the center lane away from the road reference line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    /// Polynom parameter a, offset at @s (ds=0)
    #[serde(rename = "@a")]
    pub a: f32,
    /// Polynom parameter b
    #[serde(rename = "@b")]
    pub b: f32,
    /// Polynom parameter c
    #[serde(rename = "@c")]
    pub c: f32,
    /// Polynom parameter d
    #[serde(rename = "@d")]
    pub d: f32,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f32,
}
