use crate::lane::type_link::TypeLine;
use serde::{Deserialize, Serialize};
use uom::si::f64::Length;
use vec1::Vec1;

/// Each type definition shall contain one or more line definitions with additional information
/// about the lines that the road mark is composed of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub line: Vec1<TypeLine>,
    /// Name of the road mark type. May be chosen freely.
    #[serde(rename = "@name")]
    pub name: String,
    /// Accumulated width of the road mark. In case of several `<line>` elements this @width is the
    /// sum of all @width of `<line>` elements and spaces in between, necessary to form the road
    /// mark. This attribute supersedes the definition in the `<roadMark>` element.
    #[serde(rename = "@width")]
    pub width: Length,
}
