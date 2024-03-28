use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaneChange {
    Increase,
    Decrease,
    #[default]
    Both,
    None,
}
