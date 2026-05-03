use serde::de;

use crate::{Config, de::Deserializer};

pub struct WrapSeed<'a, S> {
    pub seed: S,
    pub config: &'a Config,
}

impl<'de, S> de::DeserializeSeed<'de> for WrapSeed<'de, S>
where
    S: de::DeserializeSeed<'de>,
{
    type Value = S::Value;

    fn deserialize<D2>(self, de2: D2) -> Result<Self::Value, D2::Error>
    where
        D2: de::Deserializer<'de>,
    {
        let de = Deserializer::with_config(de2, self.config);

        self.seed.deserialize(de)
    }
}
