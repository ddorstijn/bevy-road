use serde::{Deserialize, Serialize};

/// The lane type is defined per lane. A lane type defines the main purpose of a lane and its
/// corresponding traffic rules.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LaneType {
    /// Describes a soft shoulder  at the edge of the roa
    Shoulder,
    /// Describes a hard border at the edge of the road. has the same height as the drivable lane
    Border,
    /// “normal” drivable road, which is not one of the other type
    Driving,
    /// Hard shoulder on motorways for emergency stop
    Stop,
    /// "Invisible" lane. This lane is on the most ouside of the road. Its only purpose is for simulation, that there is still opendrive present in case the (human) driver leaves the road.
    #[default]
    None,
    /// Lane on which cars should not drive, but have the same height as the drivable lanes. Typically they are separated with lines and often there are additional striped lines on them.
    Restricted,
    /// Lane with parking space
    Parking,
    /// Lane between driving lanes in oposite directions. Typically used in towns on large roads, to separate the traffic
    Median,
    /// Lane reserved for Cyclists
    Biking,
    /// Lane on which pedestrians can walk savel
    Sidewalk,
    /// Lane "curb" is used for curbstones. These have a different height compared to the drivable lanes
    Curb,
    /// Lane Type „exit“ is used for the sections which is parallel to the main road (meaning deceleration lanes)
    Exit,
    /// Lane Type „entry“ is used for the sections which is parallel to the main road (meaning acceleration lane
    Entry,
    /// A ramp leading to a motorway from rural/urban roads is an „onRamp“.
    OnRamp,
    /// A ramp leading away from a motorway and onto rural/urban roads is an „offRamp”.
    OffRamp,
    /// A ramp connecting two motorways is a „connectingRamp“ (e.g. motorway junction
    ConnectingRamp,
    /// this lane type has two use cases: a) only driving lane on a narrow road which may be used in both directions; b) continuous two-way left turn lane on multi-lane roads – US road network
    Bidirectional,
    Special1,
    Special2,
    Special3,
    RoadWorks,
    Tram,
    Rail,
    Bus,
    Taxi,
    HOV,
}
