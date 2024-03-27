use crate::lane::road_mark::explicit_line::ExplicitLine;
use serde::{Deserialize, Serialize};

/// Irregular road markings that cannot be described by repetitive line patterns may be described by
/// individual road marking elements. These explicit definitions also contain `<line>` elements for
/// the line definition, however, these lines will not be repeated automatically as in repetitive
/// road marking types. In ASAM OpenDRIVE, irregular road marking types and lines are represented by
/// `<explicit>` elements within elements. The line definitions are contained in `<line>` elements
/// within the `<explicit>` element.
// The `<explicit>` element should specifically be used for measurement data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Explicit {
    pub line: Vec<ExplicitLine>,
}
