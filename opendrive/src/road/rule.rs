use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Rule {
    #[serde(rename = "RHT")]
    RightHandTraffic,
    #[serde(rename = "LHT")]
    LeftHandTraffic,
}
