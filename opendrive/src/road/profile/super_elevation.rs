use serde::{Deserialize, Serialize};

/// Defined as the road section’s roll angle around the s-axis. Elements must be defined in
/// ascending order along the reference line. The parameters of an element are valid until the next
/// element starts or the road reference line ends. Per default, the superelevation of a road is
/// zero.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuperElevation {
    /// Polynom parameter a, superelevation at @s (ds=0)
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
}
