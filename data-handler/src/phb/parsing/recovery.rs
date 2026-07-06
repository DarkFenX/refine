use struson::{
    reader::{JsonReader, JsonStreamReader, ReaderError},
    serde::DeserializerError,
};

use super::error::ReadParseFailReason;

pub(super) fn try_recover(
    reader: &mut JsonStreamReader<impl std::io::Read>,
    error: DeserializerError,
) -> Result<DeserializerError, ReadParseFailReason> {
    // When calling deserialize_next(), if it fails it usually returns Custom variant. However, in
    // some niche cases it can return other error kinds as well (e.g. array instead of an object in
    // an array of objects)
    match error {
        DeserializerError::Custom(_)
        | DeserializerError::ReaderError(ReaderError::UnexpectedValueType { .. })
        | DeserializerError::ReaderError(ReaderError::UnexpectedStructure { .. }) => (),
        _ => return Err(error.into()),
    }
    // Recover by skipping element which failed deserialization
    let current = reader.current_position(true).path.unwrap();
    match current.len() > 1 {
        // Just get back to depth of 1 when parser is deeper
        true => reader.seek_back(&current[1..])?,
        // If parser is on the same level, likely it didn't even try to touch value, so just skip it
        false => reader.skip_value()?,
    }
    Ok(error)
}
