use elevation::Elevation;
use serde::{Deserialize, Serialize};

pub mod elevation;
pub mod lateral_profile;
pub mod shape;
pub mod super_elevation;

/// Defines the characteristics of the road elevation along the reference line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevationProfile {
    pub elevation: Vec<Elevation>,
}
