use crate::Config;
use serde::de::DeserializeOwned;
use serde_ext_core::de::Deserializer;
use std::io::Read;

/// Deserialize a value from TOML text.
pub fn from_str<T>(s: &str, config: &Config) -> Result<T, toml::de::Error>
where
    T: DeserializeOwned,
{
    let value = toml::Deserializer::parse(s)?;
    let de = Deserializer::with_config(value, config);
    T::deserialize(de)
}

/// Deserialize a value from any reader by first reading it into memory.
pub fn from_reader<R, T>(mut rdr: R, config: &Config) -> Result<T, toml::de::Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut input = String::new();
    rdr.read_to_string(&mut input)
        .map_err(|e| <toml::de::Error as serde::de::Error>::custom(e.to_string()))?;
    from_str(&input, config)
}

/// Deserialize a value from a parsed TOML value.
pub fn from_value<T>(value: toml::Value, config: &Config) -> Result<T, toml::de::Error>
where
    T: DeserializeOwned,
{
    let de = Deserializer::with_config(value, config);
    T::deserialize(de)
}
