use serde::{Deserialize, Serialize};

/// The known keywords for the road mark color information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    /// equivalent to [`Color::White`]
    Standard,
    Blue,
    Green,
    Red,
    White,
    Yellow,
    Orange,
    Violet,
}
