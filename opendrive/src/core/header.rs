use serde::{Deserialize, Serialize};

use crate::core::geo_reference::GeoReference;
use crate::core::offset::Offset;

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
    #[serde(rename = "@north")]
    pub north: Option<f32>,
    #[serde(rename = "@south")]
    pub south: Option<f32>,
    #[serde(rename = "@east")]
    pub east: Option<f32>,
    #[serde(rename = "@west")]
    pub west: Option<f32>,
    #[serde(rename = "@vendor")]
    pub vendor: Option<String>,
    #[serde(rename = "@geo_reference")]
    pub geo_reference: Option<GeoReference>,
    #[serde(rename = "@offset")]
    pub offset: Option<Offset>,
}
