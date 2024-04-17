use crate::lane::road_mark::weight::Weight;
use explicit::Explicit;
use lane_change::LaneChange;
use r#type::Type;
use serde::{Deserialize, Serialize};
use sway::Sway;
use type_simplified::TypeSimplified;

pub mod color;
pub mod explicit;
pub mod explicit_line;
pub mod lane_change;
pub mod rule;
pub mod sway;
pub mod r#type;
pub mod type_simplified;
pub mod weight;

/// Defines the style of the line at the outer border of a lane. The style of the center line that
/// separates left and right lanes is determined by the road mark element for the center lane.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoadMark {
    #[serde(default)]
    pub sway: Vec<Sway>,
    pub r#type: Option<Type>,
    pub explicit: Option<Explicit>,
    /// Color of the road mark
    #[serde(rename = "@color")]
    pub color: color::Color,
    /// Height of road mark above the road, i.e. thickness of the road mark
    #[serde(rename = "@height")]
    pub height: Option<f64>,
    /// Allows a lane change in the indicated direction, taking into account that lanes are numbered
    /// in ascending order from right to left. If the attribute is missing, “both” is used as
    /// default.
    #[serde(rename = "@laneChange")]
    #[serde(default)]
    pub lane_change: LaneChange,
    /// Material of the road mark. Identifiers to be defined by the user, use "standard" as default
    /// value.
    #[serde(rename = "@material")]
    pub material: Option<String>,
    /// s-coordinate of start position of the `<roadMark>` element, relative to the position of the
    /// preceding `<laneSection>` element
    #[serde(rename = "@sOffset")]
    pub s_offset: f64,
    /// Type of the road mark
    #[serde(rename = "@type")]
    pub type_simplified: TypeSimplified,
    /// Weight of the road mark. This attribute is optional if detailed definition is given below.
    #[serde(rename = "@weight")]
    pub weight: Option<Weight>,
    /// Width of the road mark. This attribute is optional if detailed definition is given by
    /// <line> element.
    #[serde(rename = "@width")]
    pub width: Option<f64>,
}
