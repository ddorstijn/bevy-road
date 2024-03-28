use serde::{Deserialize, Serialize};

/// The `<header>` element is the very first element within the `<OpenDRIVE>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "@revMajor")]
    pub rev_major: u16,
    #[serde(rename = "@revMinor")]
    pub rev_minor: u16,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@date")]
    pub date: Option<String>,
    #[serde(rename = "@vendor")]
    pub vendor: Option<String>,
}
