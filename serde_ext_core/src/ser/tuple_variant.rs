use serde::ser::SerializeTupleVariant;

use crate::{Config, ser::value::WrapValue};

pub struct WrapSerializeTupleVariant<'a, Tup> {
    pub inner: Tup,
    pub config: &'a Config,
}

impl<'a, Tup> SerializeTupleVariant for WrapSerializeTupleVariant<'a, Tup>
where
    Tup: serde::ser::SerializeTupleVariant,
{
    type Ok = Tup::Ok;
    type Error = Tup::Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
