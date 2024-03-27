use crate::road::geometry::Geometry;
use serde::{Deserialize, Serialize};

/// Contains geometry elements that define the layout of the road reference line in the x/y-plane
/// (plan view).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanView {
    pub geometry: Vec<Geometry>,
}
