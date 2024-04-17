use crate::lane::road_mark::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ExplicitLine {
    /// Length of the visible line
    #[serde(rename = "@length")]
    pub length: f64,
    /// Rule that must be observed when passing the line from inside, that is, from the lane with
    /// the lower absolute ID to the lane with the higher absolute ID
    #[serde(rename = "@rule")]
    pub rule: Option<Rule>,
    /// Offset of start position of the `<line>` element, relative to the @sOffset  given in the
    /// `<roadMark>` element
    #[serde(rename = "@sOffset")]
    pub s_offset: f64,
    /// Lateral offset from the lane border. If `<sway>` element is present, the lateral offset
    /// follows the sway.
    #[serde(rename = "@tOffset")]
    pub t_offset: f64,
    /// Line width. This attribute supersedes the definition in the `<roadMark>` element.
    #[serde(rename = "@width")]
    pub width: Option<f64>,
}
