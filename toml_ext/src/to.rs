use crate::Config;
use serde_ext::ser::serializer::Serializer;

/// Serialize a value into TOML text.
pub fn to_string<T>(value: &T, config: &Config) -> Result<String, toml::ser::Error>
where
    T: ?Sized + serde::Serialize,
{
    let mut output = toml::ser::Buffer::new();
    let serializer = toml::ser::Serializer::new(&mut output);
    let serializer = Serializer::new(serializer, config);
    value.serialize(serializer)?;
    Ok(output.to_string())
}

/// Serialize a value into pretty TOML text.
pub fn to_string_pretty<T>(value: &T, config: &Config) -> Result<String, toml::ser::Error>
where
    T: ?Sized + serde::Serialize,
{
    let mut output = toml::ser::Buffer::new();
    let serializer = toml::ser::Serializer::pretty(&mut output);
    let serializer = Serializer::new(serializer, config);
    value.serialize(serializer)?;
    Ok(output.to_string())
}
