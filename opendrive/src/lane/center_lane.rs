use serde::{Deserialize, Serialize};

use super::{
    access::Access, height::Height, lane_choice::LaneChoice, lane_link::LaneLink,
    lane_type::LaneType, material::Material, road_mark::RoadMark, rule::Rule, speed::Speed,
};

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CenterLane {
    /// ID of the lane
    #[serde(rename = "@id")]
    pub id: i64,
    pub link: Option<LaneLink>,
    #[serde(default)]
    pub choice: Vec<LaneChoice>,
    #[serde(rename = "roadMark")]
    pub road_mark: Vec<RoadMark>,
    #[serde(default)]
    pub material: Vec<Material>,
    #[serde(default)]
    pub speed: Vec<Speed>,
    #[serde(default)]
    pub access: Vec<Access>,
    #[serde(default)]
    pub height: Vec<Height>,
    #[serde(default)]
    pub rule: Vec<Rule>,
    /// Type of the lane
    #[serde(rename = "@type")]
    pub r#type: Option<LaneType>,
}
