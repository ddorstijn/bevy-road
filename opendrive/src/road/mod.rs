use crate::lane::lanes::Lanes;
// use crate::object::objects::Objects;
use crate::road::profile::ElevationProfile;
use crate::road::road_type::RoadType;
// use crate::signal::signals::Signals;
use geometry::plan_view::PlanView;
use link::Link;
use profile::lateral_profile::LateralProfile;
use rule::Rule;
use serde::{Deserialize, Serialize};

pub mod element_type;
pub mod geometry;
pub mod link;
pub mod predecessor_successor;
pub mod profile;
pub mod road_type;
pub mod road_type_e;
pub mod rule;
pub mod speed;
pub mod unit;

/// In ASAM OpenDRIVE, the road network is represented by `<road>` elements. Each road runs along
/// one road reference line. A road shall have at least one lane with a width larger than 0.
/// Vehicles may drive in both directions of the reference line. The standard driving direction is
/// defined by the value which is assigned to the @rule attribute (RHT=right-hand traffic,
/// LHT=left-hand traffic).
/// ASAM OpenDRIVE roads may be roads in the real road network or artificial road network created
/// for application use. Each road is described by one or more `<road>` elements. One `<road>`
/// element may cover a long stretch of a road, shorter stretches between junctions, or even several
/// roads. A new `<road>` element should only start if the properties of the road cannot be
/// described within the previous `<road>` element or if a junction is required.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Road {
    /// Unique ID within the database. If it represents an integer number, it should comply to
    /// `uint32_t` and stay within the given range.
    #[serde(rename = "@id")]
    pub id: String,
    /// ID of the junction to which the road belongs as a connecting road (= -1 for none)
    #[serde(rename = "@junction")]
    pub junction: String,
    /// Total length of the reference line in the xy-plane. Change in length due to elevation is not
    /// considered.
    /// Only positive values are valid.
    #[serde(rename = "@length")]
    pub length: f32,
    /// Name of the road. May be chosen freely.
    #[serde(rename = "@name")]
    pub name: Option<String>,
    /// Basic rule for using the road; RHT=right-hand traffic, LHT=left-hand traffic. When this
    /// attribute is missing, RHT is assumed.
    #[serde(rename = "@rule")]
    #[serde(default)]
    pub rule: Option<Rule>,
    pub link: Option<Link>,
    #[serde(default)]
    pub r#type: Vec<RoadType>,
    pub plan_view: PlanView,
    pub elevation_profile: Option<ElevationProfile>,
    pub lateral_profile: Option<LateralProfile>,
    pub lanes: Lanes,
    // pub objects: Option<Objects>,
    // pub signals: Option<Signals>,
}
