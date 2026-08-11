use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde_json::value::RawValue;

use super::error::ReadParseFailReason;

// Returns first array entry accepted by the predicate
pub(in crate::phb) fn find_in_array<T>(
    reader: impl std::io::Read,
    predicate: impl FnMut(&T) -> bool,
) -> Result<Option<T>, ReadParseFailReason>
where
    T: serde::de::DeserializeOwned,
{
    let mut found = None;
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let visitor = Array {
        predicate,
        found: &mut found,
        phantom: std::marker::PhantomData,
    };
    match deserializer.deserialize_seq(visitor) {
        Ok(()) => {
            deserializer.end()?;
            Ok(None)
        }
        // Error happens in two cases: when entry is found, then we report success to caller, and
        // some legitimate error occurred during search, in which case it is propagated to caller
        Err(..) if found.is_some() => Ok(found),
        Err(error) => Err(error.into()),
    }
}

// Visitor which goes through all entries and returns one matching predicate via error (there seems
// to be no other way to return early)
struct Array<'a, T, F> {
    predicate: F,
    found: &'a mut Option<T>,
    phantom: std::marker::PhantomData<T>,
}
impl<'de, T, F> Visitor<'de> for Array<'_, T, F>
where
    T: serde::de::DeserializeOwned,
    F: FnMut(&T) -> bool,
{
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("array of {}", std::any::type_name::<T>()))
    }

    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(raw_value) = seq.next_element::<Box<RawValue>>()? {
            // Skip elements which cannot be converted to the target type
            let Ok(value) = serde_json::from_str::<T>(raw_value.get()) else {
                continue;
            };
            if (self.predicate)(&value) {
                *self.found = Some(value);
                return Err(A::Error::custom("entry found"));
            }
        }
        Ok(())
    }
}
