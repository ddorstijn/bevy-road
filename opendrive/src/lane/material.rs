use serde::{Deserialize, Serialize};

/// Stores information about the material of lanes. Each element is valid until a new element is
/// defined. If multiple elements are defined, they must be listed in ascending order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    /// Friction coefficient
    #[serde(rename = "@friction")]
    pub friction: f32,
    /// Roughness, for example, for sound and motion systems
    #[serde(rename = "@roughness")]
    pub roughness: Option<f32>,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    #[serde(rename = "@sOffset")]
    pub s_offset: f32,
    /// Surface material code, depending on application
    #[serde(rename = "@surface")]
    pub surface: Option<String>,
}
