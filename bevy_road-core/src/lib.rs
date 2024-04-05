use std::{collections::BTreeMap, path::Path};

use opendrive::core::OpenDrive;

use polynomal::Polynomal;
use road::Road;

pub mod lane;
pub mod reference_line;
pub mod road;

mod odr_spiral;
mod polynomal;

#[derive(Debug)]
pub struct BevyRoad {
    pub name: String,
    pub version: String,

    pub roads: BTreeMap<u32, Road>,
}

impl From<OpenDrive> for BevyRoad {
    fn from(odr: OpenDrive) -> Self {
        let name = odr.header.name.unwrap_or("Untitled project".to_string());
        let version = odr.header.version.unwrap_or("0.01".to_string());

        let roads = odr
            .road
            .iter()
            .map(|r| (r.id.parse().unwrap_or(0), Road::from(r)))
            .collect();

        Self {
            name,
            version,
            roads,
        }
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> BevyRoad {
    BevyRoad::from(opendrive::load_opendrive(path).expect("File can not be found"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {
        let odr = opendrive::load_opendrive("C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Line-Spiral-Arc.xodr").unwrap();
        let br = crate::BevyRoad::from(odr);

        println!("{:?}", br);
    }
}
