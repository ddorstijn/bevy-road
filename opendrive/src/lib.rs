use serde::{Deserialize, Deserializer};
use uom::si::{curvature::radian_per_meter, f64::Curvature};

pub mod core;
pub mod junction;
pub mod lane;
pub mod road;

pub fn curvature_from_scientific<'de, D>(deserializer: D) -> Result<Curvature, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer).and_then(|string| {
        let float = string
            .parse::<f64>()
            .map_err(|err| Error::custom(err.to_string()))?;

        Ok(Curvature::new::<radian_per_meter>(float))
    })
}
