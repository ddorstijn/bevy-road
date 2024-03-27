use serde::{Deserialize, Serialize};

/// Lane borders are another method to describe the width of lanes. Instead of defining the width
/// directly, lane borders describe the outer limits of a lane, independent of the parameters of
/// their inner borders. In this case, inner lanes are defined as lanes which have the same sign for
/// their ID as the lane currently defined, but with a smaller absolute value for their ID.
/// Especially when road data is derived from automatic measurements, this type of definition is
/// easier than specifying the lane width because it avoids creating many lane sections.
/// Lane width and lane border elements are mutually exclusive within the same lane group. If both
/// width and lane border elements are present for a lane section in the ASAM OpenDRIVE file, the
/// application shall use the information from the `<width>` elements.
/// In ASAM OpenDRIVE, lane borders are represented by the `<border>` element within the `<lane>`
/// element.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Border {
    /// Polynom parameter a, border position at @s (ds=0)
    #[serde(rename = "@a")]
    pub a: f32,
    /// Polynom parameter b
    #[serde(rename = "@b")]
    pub b: f32,
    /// Polynom parameter c
    #[serde(rename = "@c")]
    pub c: f32,
    /// Polynom parameter d
    #[serde(rename = "@d")]
    pub d: f32,
    /// s-coordinate of start position of the `<border>` element , relative to the position of the
    /// preceding `<laneSection>` element
    #[serde(rename = "@sOffset")]
    pub s_offset: f32,
}
