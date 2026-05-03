use serde::ser::SerializeMap;

use crate::{Config, ser::value::WrapValue};

pub struct WrapSerializeMap<'a, Map> {
    pub inner: Map,
    pub config: &'a Config,
}

impl<'a, Map> SerializeMap for WrapSerializeMap<'a, Map>
where
    Map: serde::ser::SerializeMap,
{
    type Ok = Map::Ok;
    type Error = Map::Error;

    fn serialize_key<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        key: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_key(&WrapValue {
            value: key,
            config: self.config,
        })
    }

    fn serialize_value<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_value(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
