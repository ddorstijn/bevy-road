use serde::{Deserialize, Serialize};

use crate::road::geometry::arc::Arc;
use crate::road::geometry::line::Line;
use crate::road::geometry::spiral::Spiral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeometryType {
    Line(Line),
    Spiral(Spiral),
    Arc(Arc),
}
