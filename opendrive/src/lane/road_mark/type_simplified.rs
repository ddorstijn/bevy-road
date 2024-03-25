use serde::{Deserialize, Serialize};

/// The known keywords for the simplified road mark type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeSimplified {
    None,
    Solid,
    Broken,
    /// for double solid line
    #[serde(rename = "solid solid")]
    SolidSolid,
    /// from inside to outside, exception: center lane – from left to right
    #[serde(rename = "solid broken")]
    SolidBroken,
    /// from inside to outside, exception: center lane – from left to right
    #[serde(rename = "broken solid")]
    BrokenSolid,
    /// from inside to outside, exception: center lane – from left to right
    #[serde(rename = "broken broken")]
    BrokenBroken,
    #[serde(rename = "botts dots")]
    BottsDots,
    /// meaning a grass edge
    Grass,
    Curb,
    /// if detailed description is given in child tags (via [`Type`])
    Custom,
    /// describing the limit of usable space on a road
    Edge,
}
