use crate::lane::access::restriction_type::AccessRestrictionType;
use rule::AccessRule;
use serde::{Deserialize, Serialize};

pub mod restriction_type;
pub mod rule;

/// Defines access restrictions for certain types of road users.
/// Each element is valid in direction of the increasing s coordinate until a new element is
/// defined. If multiple elements are defined, they shall be listed in ascending order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Access {
    /// Identifier of the participant to whom the restriction applies
    #[serde(rename = "@restriction")]
    pub restriction: AccessRestrictionType,
    /// Specifies whether the participant given in the attribute @restriction is allowed or denied
    /// access to the given lane
    #[serde(rename = "@rule")]
    pub rule: Option<AccessRule>,
    /// s-coordinate of start position, relative to the position of the preceding `<laneSection>`
    /// element
    #[serde(rename = "@sOffset")]
    pub s_offset: f32,
}
