use struson::reader::{JsonReader, JsonStreamReader};

use crate::phb::parsing::ReadParseError;

pub(in crate::phb) struct ArrayIter<T, R>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
{
    reader: JsonStreamReader<R>,
    opened: bool,
    closed: bool,
    phantom: std::marker::PhantomData<T>,
}
impl<T, R> ArrayIter<T, R>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
{
    pub(in crate::phb) fn new(reader: R) -> Self {
        Self {
            reader: JsonStreamReader::new(reader),
            opened: false,
            closed: false,
            phantom: Default::default(),
        }
    }
}
impl<T, R> Iterator for ArrayIter<T, R>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
{
    type Item = Result<T, ReadParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.closed {
            return None;
        }
        if !self.opened {
            if let Err(error) = self.reader.begin_array() {
                return Some(Err(error.into()));
            }
            self.opened = true;
        }
        loop {
            match self.reader.has_next() {
                Ok(has_next) => match has_next {
                    true => match self.reader.deserialize_next::<serde_json::Value>() {
                        Ok(raw_value) => match serde_json::from_value::<T>(raw_value) {
                            Ok(value) => return Some(Ok(value)),
                            // Silently skip malformed entries
                            Err(_) => continue,
                        },
                        Err(error) => return Some(Err(error.into())),
                    },
                    false => break,
                },
                Err(e) => return Some(Err(e.into())),
            }
        }
        if !self.closed {
            if let Err(error) = self.reader.end_array() {
                return Some(Err(error.into()));
            }
            self.closed = true;
        }
        None
    }
}
