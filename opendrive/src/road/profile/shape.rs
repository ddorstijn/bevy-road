use serde::{Deserialize, Serialize};

/// Defined as the road section’s surface relative to the reference plane. There may be several
/// shape definitions at one s-position that have different t-values, thereby describing the curvy
/// shape of the road.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    /// Polynom parameter a, relative height at @t (dt=0)
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
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f64,
    /// t-coordinate of start position
    #[serde(rename = "@t")]
    pub t: f64,
}
