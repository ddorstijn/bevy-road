use serde::{Deserialize, Serialize};

use crate::road::unit::SpeedUnit;

/// Defines the default maximum speed allowed in conjunction with the specified road type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Speed {
    /// Maximum allowed speed. Given as string (only "no limit" / "undefined") or numerical value in
    /// the respective unit (see attribute unit). If the attribute unit is not specified, m/s is
    /// used as default.
    #[serde(rename = "@max")]
    pub max: MaxSpeed,
    /// Unit of the attribute max. For values, see chapter “units”.
    #[serde(rename = "@unit")]
    pub unit: Option<SpeedUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaxSpeed {
    Limit(f64),
    NoLimit,
    Undefined,
}
