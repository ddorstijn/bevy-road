use serde::{Deserialize, Serialize};

/// Used to add rules that are not covered by any of the other lane attributes that are described in
/// this specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    #[serde(rename = "@sOffset")]
    pub s_offset: f64,
    /// Free text; currently recommended values are
    /// - "no stopping at any time"
    /// - "disabled parking"
    /// - "car pool"
    #[serde(rename = "@value")]
    pub value: String,
}
