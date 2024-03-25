use serde::{Deserialize, Serialize};
use uom::si::f64::Curvature;

use crate::curvature_from_scientific;

/// An arc describes a road reference line with constant curvature. In ASAM OpenDRIVE, an arc is
/// represented by an `<arc>` element within the `<geometry>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arc {
    /// Constant curvature throughout the element
    #[serde(rename = "@curvature")]
    #[serde(deserialize_with = "curvature_from_scientific")]
    pub curvature: Curvature,
}
