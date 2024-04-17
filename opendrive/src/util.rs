use serde::{Deserialize, Deserializer};

pub fn deserialize_scientific<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer).and_then(|string| {
        let float = string
            .parse::<f64>()
            .map_err(|err| Error::custom(err.to_string()))?;

        Ok(float)
    })
}
