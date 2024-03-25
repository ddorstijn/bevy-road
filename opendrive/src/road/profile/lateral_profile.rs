use serde::{Deserialize, Serialize};

use crate::road::profile::shape::Shape;
use crate::road::profile::super_elevation::SuperElevation;

/// Contains a series of superelevation elements that define the characteristics of the road
/// surface's banking along the reference line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LateralProfile {
    #[serde(default)]
    pub super_elevation: Vec<SuperElevation>,
    #[serde(default)]
    pub shape: Vec<Shape>,
}
