use serde::de::{DeserializeSeed, SeqAccess};

use crate::{Config, de::seed::WrapSeed};

pub struct WrapSeqAccess<'a, A> {
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
