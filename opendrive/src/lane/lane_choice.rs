use serde::{Deserialize, Serialize};

use crate::lane::border::Border;
use crate::lane::width::Width;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum LaneChoice {
    Border(Border),
    Width(Width),
}
