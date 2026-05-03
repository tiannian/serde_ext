use crate::{Config, ser::serializer::Serializer};

pub struct WrapValue<'a, T: ?Sized> {
    pub value: &'a T,
    pub config: &'a Config,
}

impl<'a, T: ?Sized> serde::ser::Serialize for WrapValue<'a, T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S2>(&self, serializer: S2) -> Result<S2::Ok, S2::Error>
    where
        S2: serde::ser::Serializer,
    {
        self.value
            .serialize(Serializer::new(serializer, self.config))
    }
}
