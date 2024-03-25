use serde::{Deserialize, Serialize};

use crate::lane::predecessor_successor::PredecessorSuccessor;

/// For links between lanes with an identical reference line, the lane predecessor and successor
/// information provide the IDs of lanes on the preceding or following lane section.
/// For links between lanes with different reference line,  the lane predecessor and successor
/// information provide the IDs of lanes on the first or last lane section of the other reference
/// line depending on the contact point of the road linkage.
/// This element may only be omitted, if lanes end at a junction or have no physical link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneLink {
    #[serde(default)]
    pub predecessor: Vec<PredecessorSuccessor>,
    #[serde(default)]
    pub successor: Vec<PredecessorSuccessor>,
}
