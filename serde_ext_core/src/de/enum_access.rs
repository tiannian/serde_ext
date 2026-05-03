use serde::de::{DeserializeSeed, EnumAccess};

use crate::{Config, de::seed::WrapSeed};

pub struct WrapEnumAccess<'a, A> {
    pub inner: A,
    pub config: &'a Config,
}

impl<'de, A> EnumAccess<'de> for WrapEnumAccess<'de, A>
where
    A: EnumAccess<'de>,
{
    type Error = A::Error;
    type Variant = A::Variant;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner.variant_seed(WrapSeed {
            seed,
            config: self.config,
        })
    }
}
