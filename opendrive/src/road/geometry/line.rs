use serde::{Deserialize, Serialize};

/// A straight line is the simplest geometry element. It contains no further attributes.
/// In ASAM OpenDRIVE, a straight line is represented by a `<line>` element within the `<geometry>`
/// element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {}
