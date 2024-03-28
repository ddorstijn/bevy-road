use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rule {
    #[serde(rename = "no passing")]
    NoPassing,
    Caution,
    None,
}