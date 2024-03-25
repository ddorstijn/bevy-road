use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessRestrictionType {
    Simulator,
    AutonomousTraffic,
    Pedestrian,
    PassengerCar,
    Bus,
    Delivery,
    Emergency,
    Taxi,
    ThroughTraffic,
    Truck,
    Bicycle,
    Motorcycle,
    None,
    Trucks,
}
