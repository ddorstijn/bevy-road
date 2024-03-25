use serde::{Deserialize, Serialize};

/// The known keywords for the road type information
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoadTypeE {
    Unknown,
    Rural,
    Motorway,
    Town,
    /// In Germany, lowSpeed is equivalent to a 30km/h zone
    LowSpeed,
    Pedestrian,
    Bicycle,
    TownExpressway,
    TownCollector,
    TownArterial,
    TownPrivate,
    TownLocal,
    TownPlayStreet,
}
