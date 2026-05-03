use crate::{Config, de::Deserializer};
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess};

pub(crate) struct WrapSeed<'a, S> {
    pub seed: S,
    pub config: &'a Config,
}

impl<'de, S> DeserializeSeed<'de> for WrapSeed<'de, S>
where
    S: DeserializeSeed<'de>,
{
    type Value = S::Value;

    fn deserialize<D2>(self, de2: D2) -> Result<Self::Value, D2::Error>
    where
        D2: serde::Deserializer<'de>,
    {
        let de = Deserializer::with_config(de2, self.config);
        self.seed.deserialize(de)
    }
}

pub(crate) struct WrapSeqAccess<'a, A> {
    pub inner: A,
    pub config: &'a Config,
}

impl<'de, A> SeqAccess<'de> for WrapSeqAccess<'de, A>
where
    A: SeqAccess<'de>,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.inner.next_element_seed(WrapSeed {
            seed,
            config: self.config,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

pub(crate) struct WrapMapAccess<'a, A> {
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

pub(crate) struct WrapEnumAccess<'a, A> {
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
