use serde::de::{DeserializeSeed, MapAccess};

use crate::{Config, de::seed::WrapSeed};

pub struct WrapMapAccess<'a, A> {
    pub inner: A,
    pub config: &'a Config,
}

impl<'de, A> MapAccess<'de> for WrapMapAccess<'de, A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        self.inner.next_key_seed(WrapSeed {
            seed,
            config: self.config,
        })
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(WrapSeed {
            seed,
            config: self.config,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}
