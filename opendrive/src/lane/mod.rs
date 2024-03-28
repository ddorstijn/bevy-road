use serde::{Deserialize, Serialize};

use self::{
    access::Access, height::Height, lane_link::LaneLink, lane_type::LaneType, material::Material,
    road_mark::RoadMark, rule::Rule, speed::Speed, width::Width,
};

pub mod access;
pub mod center;
pub mod height;
pub mod lane_link;
pub mod lane_section;
pub mod lane_type;
pub mod lanes;
pub mod left;
pub mod material;
pub mod offset;
pub mod predecessor_successor;
pub mod right;
pub mod road_mark;
pub mod rule;
pub mod speed;
pub mod type_link;
pub mod width;

/// Lane elements are included in left/center/right elements. Lane elements should represent the
/// lanes from left to right, that is, with descending ID.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lane {
    /// ID of the lane
    #[serde(rename = "@id")]
    pub id: i32,

    pub link: Option<LaneLink>,
    #[serde(default)]
    pub width: Vec<Width>,
    #[serde(default)]
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
    #[serde(default)]
    pub r#type: LaneType,
}
