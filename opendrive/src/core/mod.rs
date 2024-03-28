use serde::{Deserialize, Serialize};

use crate::core::header::Header;
// use crate::junction::junction_group::JunctionGroup;
// use crate::junction::Junction;
use crate::road::Road;
// use crate::signal::controller::Controller;

pub mod header;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenDrive {
    pub header: Header,
    pub road: Vec<Road>,
    // pub controller: Vec<Controller>,
    // pub junction: Vec<Junction>,
    // pub junction_group: Vec<JunctionGroup>,
}
