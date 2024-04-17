use serde::{Deserialize, Serialize};

use crate::util::deserialize_scientific;

/// An arc describes a road reference line with constant curvature. In ASAM OpenDRIVE, an arc is
/// represented by an `<arc>` element within the `<geometry>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arc {
    /// Constant curvature throughout the element
    #[serde(rename = "@curvature")]
    #[serde(deserialize_with = "deserialize_scientific")]
    pub curvature: f64,
}
