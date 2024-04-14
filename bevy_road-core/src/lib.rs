use std::{collections::BTreeMap, path::Path};

use bevy::prelude::*;

use polynomal::Polynomal;
use road::Road;

pub mod geometry;
pub mod lane;
pub mod road;

mod polynomal;

#[derive(Component)]
pub struct Lanes;

#[derive(Debug, Resource, Clone)]
pub struct BevyRoad {
    pub name: String,
    pub version: String,
    pub roads: BTreeMap<String, Entity>,
}

impl Default for BevyRoad {
    fn default() -> Self {
        Self {
            name: "Untitled project".to_string(),
            version: "0.01".to_string(),
            roads: BTreeMap::new(),
        }
    }
}

impl BevyRoad {
    pub fn from_xodr<P: AsRef<Path>>(
        path: P,
        mut commands: Commands,
    ) -> Result<BevyRoad, opendrive::DeError> {
        let odr = opendrive::load_opendrive(path)?;

        let roads = odr
            .road
            .iter()
            .map(|r| {
                let road = Road::from(r);
                let id = commands.spawn((Name::new(r.id.clone()), road)).id();

                (r.id.clone(), id)
            })
            .collect();

        Ok(BevyRoad {
            name: odr.header.name.unwrap_or("Untitled project".to_string()),
            version: odr.header.version.unwrap_or("0.01".to_string()),
            roads,
        })
    }
}
