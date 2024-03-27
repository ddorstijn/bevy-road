use serde::{Deserialize, Serialize};

/// Defines an elevation element at a given position on the reference line. Elements shall be
/// defined in ascending order along the reference line. The s length does not change with the
/// elevation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elevation {
    /// Polynom parameter a, elevation at @s (ds=0)
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
