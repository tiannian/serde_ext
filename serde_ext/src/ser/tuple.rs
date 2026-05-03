use serde::ser::SerializeTuple;

use crate::{Config, ser::value::WrapValue};

pub struct WrapSerializeTuple<'a, Tup> {
    pub inner: Tup,
    pub config: &'a Config,
}

impl<'a, Tup> SerializeTuple for WrapSerializeTuple<'a, Tup>
where
    Tup: serde::ser::SerializeTuple,
{
    type Ok = Tup::Ok;
    type Error = Tup::Error;

    fn serialize_element<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_element(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
