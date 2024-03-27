use crate::lane::type_link::TypeLine;
use serde::{Deserialize, Serialize};

/// Each type definition shall contain one or more line definitions with additional information
/// about the lines that the road mark is composed of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub line: Vec<TypeLine>,
    /// Name of the road mark type. May be chosen freely.
    #[serde(rename = "@name")]
    pub name: String,
    /// Accumulated width of the road mark. In case of several `<line>` elements this @width is the
    /// sum of all @width of `<line>` elements and spaces in between, necessary to form the road
    /// mark. This attribute supersedes the definition in the `<roadMark>` element.
    #[serde(rename = "@width")]
    pub width: f32,
}
