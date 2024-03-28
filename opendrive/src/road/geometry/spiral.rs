use serde::{Deserialize, Serialize};

use crate::util::deserialize_scientific;

/// In ASAM OpenDRIVE, a spiral is represented by a `<spiral>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spiral {
    /// Curvature at the start of the element
    #[serde(rename = "@curvStart")]
    #[serde(deserialize_with = "deserialize_scientific")]
    pub curvature_start: f32,
    /// Curvature at the end of the element
    #[serde(rename = "@curvEnd")]
    #[serde(deserialize_with = "deserialize_scientific")]
    pub curvature_end: f32,
}
