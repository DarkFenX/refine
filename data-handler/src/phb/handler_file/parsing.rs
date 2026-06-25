use struson::reader::{JsonReader, JsonStreamReader};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Array handling
////////////////////////////////////////////////////////////////////////////////////////////////////
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
    type Item = rc::ed::EResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.closed {
            return None;
        }
        if !self.opened {
            if let Err(e) = self.reader.begin_array() {
                return Some(Err(e.into()));
            }
            self.opened = true;
        }
        match self.reader.has_next() {
            Ok(has_next) if has_next => match self.reader.deserialize_next::<T>() {
                Ok(value) => return Some(Ok(value)),
                Err(e) => return Some(Err(e.into())),
            },
            Err(e) => return Some(Err(e.into())),
            _ => (),
        }
        if !self.closed {
            if let Err(e) = self.reader.end_array() {
                return Some(Err(e.into()));
            }
            self.closed = true;
        }
        None
    }
}
