use struson::reader::simple::{SimpleJsonReader, ValueReader};

use crate::{phb::data::PMetadata, util::Error};

pub(in crate::phb) type Key = i32;

pub(in crate::phb) trait KeyMerge<T> {
    fn key_merge(self, key: Key) -> Vec<T>;
}

pub(in crate::phb) fn handle_keyed_map_one<F, E>(
    reader: impl std::io::Read,
    suffix: &str,
) -> rc::ed::EResult<rc::ed::EDataCont<E>>
where
    F: serde::de::DeserializeOwned + KeyMerge<E>,
{
    let mut e_cont = rc::ed::EDataCont::new();
    let json_reader = SimpleJsonReader::new(reader);
    match json_reader.read_object_borrowed_names(|mut member_reader| {
        let raw_key = member_reader.read_name()?;
        // In case of malformed ID - log error and skip element
        let Ok(key) = raw_key.parse::<Key>() else {
            e_cont
                .warns
                .push(format!("failed to cast key \"{}\" to integer", raw_key));
            return Ok(());
        };
        let value = match member_reader.read_deserialize::<F>() {
            Ok(value) => value,
            // In case of an unexpected value format - log error and skip element
            Err(e) => {
                e_cont
                    .warns
                    .push(format!("failed to parse value with key \"{key}\": {e}"));
                return Ok(());
            }
        };
        e_cont.data.extend(value.key_merge(key));
        Ok(())
    }) {
        Ok(_) => Ok(e_cont),
        Err(e) => Err(Error::PhbUnrecoverableError(suffix.to_string(), e).into()),
    }
}

pub(in crate::phb) fn handle_metadata_client_build(
    reader: impl std::io::Read,
    suffix: &str,
) -> rc::ed::EResult<Option<String>> {
    let mut result = None;
    let json_reader = SimpleJsonReader::new(reader);
    match json_reader.read_array_items(|value_reader| {
        // Uses `client_build` value of the metadata file as version.
        if let Ok(metadata) = value_reader.read_deserialize::<PMetadata>()
            && metadata.field_name == "client_build"
        {
            result = Some(metadata.field_value.to_string())
        }
        Ok(())
    }) {
        Ok(_) => Ok(result),
        Err(e) => Err(Error::PhbUnrecoverableError(suffix.to_string(), e).into()),
    }
}
