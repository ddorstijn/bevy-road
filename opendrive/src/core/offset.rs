use serde::{Deserialize, Serialize};

/// To avoid large coordinates, an offset of the whole dataset may be applied using the `<offset>`
/// element. It enables inertial relocation and re-orientation of datasets. The dataset is first
/// translated by @x, @y, and @z. Afterwards, it is rotated by @hdg around the new origin. Rotation
/// around the z-axis should be avoided. In ASAM OpenDRIVE, the offset of a database is represented
/// by the `<offset>` element within the `<header>` element.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Offset {
    /// Heading offset (rotation around resulting z-axis)
    #[serde(rename = "@hdg")]
    pub hdg: f32,
    /// Inertial x offset
    #[serde(rename = "@x")]
    pub x: f32,
    /// Inertial y offset
    #[serde(rename = "@y")]
    pub y: f32,
    /// Inertial z offset
    #[serde(rename = "@z")]
    pub z: f32,
}
