use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Unit {
    Distance(DistanceUnit),
    Speed(SpeedUnit),
    Mass(MassUnit),
    Slope(SlopeUnit),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistanceUnit {
    #[serde(rename = "m")]
    Meter,
    #[serde(rename = "km")]
    KiloMeter,
    #[serde(rename = "ft")]
    Feet,
    #[serde(rename = "mile")]
    Mile,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeedUnit {
    #[serde(rename = "km/h")]
    KilometersPerHour,
    #[serde(rename = "m/s")]
    MetersPerSecond,
    #[serde(rename = "mph")]
    MilesPerHour,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MassUnit {
    #[serde(rename = "kg")]
    KiloGram,
    #[serde(rename = "t")]
    Ton,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlopeUnit {
    #[serde(rename = "%")]
    Percentage,
}
