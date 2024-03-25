use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PredecessorSuccessor {
    /// ID of the preceding / succeeding linked lane
    #[serde(rename = "@id")]
    pub id: i64,
}
