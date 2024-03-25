use serde::{Deserialize, Serialize};

use crate::road::predecessor_successor::PredecessorSuccessor;

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
}
