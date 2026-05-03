use serde::ser::SerializeSeq;

use crate::{Config, ser::value::WrapValue};

pub struct WrapSerializeSeq<'a, Seq> {
    pub inner: Seq,
    pub config: &'a Config,
}

impl<'a, Seq> SerializeSeq for WrapSerializeSeq<'a, Seq>
where
    Seq: serde::ser::SerializeSeq,
{
    type Ok = Seq::Ok;
    type Error = Seq::Error;

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
