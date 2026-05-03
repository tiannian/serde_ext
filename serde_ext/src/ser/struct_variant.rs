use serde::ser::SerializeStructVariant;

use crate::{Config, ser::value::WrapValue};

pub struct WrapSerializeStructVariant<'a, Struct> {
    pub inner: Struct,
    pub config: &'a Config,
}

impl<'a, Struct> SerializeStructVariant for WrapSerializeStructVariant<'a, Struct>
where
    Struct: serde::ser::SerializeStructVariant,
{
    type Ok = Struct::Ok;
    type Error = Struct::Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(
            key,
            &WrapValue {
                value,
                config: self.config,
            },
        )
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
