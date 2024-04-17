use crate::junction::contact_point::ContactPoint;
use crate::junction::element_dir::ElementDir;
use crate::road::element_type::ElementType;

use serde::{Deserialize, Serialize};

/// Successors and predecessors can be junctions or roads. For each, different attribute sets shall
/// be used.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PredecessorSuccessor {
    /// Contact point of link on the linked element
    #[serde(rename = "@contactPoint")]
    pub contact_point: Option<ContactPoint>,
    /// To be provided when elementS is used for the connection definition. Indicates the direction
    /// on the predecessor from which the road is entered.
    #[serde(rename = "@elementDir")]
    pub element_dir: Option<ElementDir>,
    /// ID of the linked element
    #[serde(rename = "@elementId")]
    pub element_id: String,
    /// Alternative to contactPoint for virtual junctions. Indicates a connection within the
    /// predecessor, meaning not at the start or end of the predecessor. Shall only be used for
    /// elementType "road"
    #[serde(rename = "@elementS")]
    pub element_s: Option<f64>,
    /// Type of the linked element
    #[serde(rename = "@elementType")]
    pub element_type: Option<ElementType>,
}
