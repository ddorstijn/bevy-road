use core::OpenDrive;
use std::path::Path;

pub use quick_xml::DeError;

pub mod core;
pub mod junction;
pub mod lane;
pub mod road;

pub(crate) mod util;

pub fn load_opendrive<P: AsRef<Path>>(path: P) -> Result<OpenDrive, DeError> {
    let xml = std::fs::read_to_string(path).map_err(|error| DeError::Custom(error.to_string()))?;
    quick_xml::de::from_str::<OpenDrive>(&xml)
}
