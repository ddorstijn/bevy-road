use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum ElementDir {
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
}
