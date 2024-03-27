use crate::road::road_type_e::RoadTypeE;
use crate::road::speed::Speed;
use serde::{Deserialize, Serialize};

/// A road type element is valid for the entire cross section of a road. It is valid until a new
/// road type element is provided or until the road ends.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoadType {
    pub speed: Option<Speed>,
    /// Country code of the road, see ISO 3166-1, alpha-2 codes.
    #[serde(rename = "@rule")]
    pub country: Option<String>,
    /// s-coordinate of start position
    #[serde(rename = "@s")]
    pub s: f32,
    /// Type of the road defined as enumeration
    #[serde(rename = "@type")]
    pub r#type: RoadTypeE,
}
