use crate::lane::road_mark::color::Color;
use crate::lane::road_mark::rule::Rule;
use serde::{Deserialize, Serialize};

/// A road mark may consist of one or more elements. Multiple elements are usually positioned
/// side-by-side. A line definition is valid for a given length of the lane and will be repeated
/// automatically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeLine {
    /// Line color. If given, this attribute supersedes the definition in the `<roadMark>` element.
    #[serde(rename = "@color")]
    pub color: Option<Color>,
    /// Length of the visible part
    #[serde(rename = "@length")]
    pub length: f32,
    /// Rule that must be observed when passing the line from inside, for example, from the lane
    /// with the lower absolute ID to the lane with the higher absolute ID
    #[serde(rename = "@rule")]
    pub rule: Option<Rule>,
    /// Initial longitudinal offset of the line definition from the start of the road mark
    /// definition
    #[serde(rename = "@sOffset")]
    pub s_offset: f32,
    /// Length of the gap between the visible parts
    #[serde(rename = "@space")]
    pub space: f32,
    /// Lateral offset from the lane border.
    /// If `<sway>` element is present, the lateral offset follows the sway.
    #[serde(rename = "@tOffset")]
    pub t_offset: f32,
    /// Line width
    #[serde(rename = "@width")]
    pub width: Option<f32>,
}
