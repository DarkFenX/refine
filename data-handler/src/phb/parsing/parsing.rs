use struson::{
    reader::{JsonReader, JsonStreamReader},
    serde::DeserializerError,
};

use super::{
    aliases::Key,
    error::ReadParseFailReason,
    traits::{KeyMergeOne, KeyMergeTwo},
};

pub(in crate::phb) fn handle_keymap_one<PHB, EVE>(
    reader: impl std::io::Read,
) -> Result<rc::ed::EDataCont<EVE>, ReadParseFailReason>
where
    PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
{
    let mut e_cont = rc::ed::EDataCont::new();
    let mut reader = JsonStreamReader::new(reader);
    reader.begin_object()?;
    while reader.has_next()? {
        let raw_key = reader.next_name()?;
        // In case of malformed ID - log error and skip element
        let Ok(key) = raw_key.parse::<Key>() else {
            let warning = format!("failed to cast key \"{}\" to integer", raw_key);
            e_cont.warnings.push(warning);
            reader.skip_value()?;
            continue;
        };
        let value = match reader.deserialize_next::<PHB>() {
            Ok(value) => value,
            Err(error) => {
                let error = try_recover(&mut reader, error)?;
                let warning = format!("failed to parse value with key \"{key}\": {error}");
                e_cont.warnings.push(warning);
                continue;
            }
        };
        e_cont.data.extend(value.key_merge(key));
    }
    reader.end_object()?;
    reader.consume_trailing_whitespace()?;
    Ok(e_cont)
}

pub(in crate::phb) fn handle_keymap_two<PHB, EVE1, EVE2>(
    reader: impl std::io::Read,
) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), ReadParseFailReason>
where
    PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
{
    let mut e_cont1 = rc::ed::EDataCont::new();
    let mut e_cont2 = rc::ed::EDataCont::new();
    let mut reader = JsonStreamReader::new(reader);
    reader.begin_object()?;
    while reader.has_next()? {
        let raw_key = reader.next_name()?;
        // In case of malformed ID - log error and skip element
        let Ok(key) = raw_key.parse::<Key>() else {
            let warning = format!("failed to cast key \"{}\" to integer", raw_key);
            e_cont1.warnings.push(warning.clone());
            e_cont2.warnings.push(warning);
            reader.skip_value()?;
            continue;
        };
        let value = match reader.deserialize_next::<PHB>() {
            Ok(value) => value,
            Err(error) => {
                let error = try_recover(&mut reader, error)?;
                let warning = format!("failed to parse value with key \"{key}\": {error}");
                e_cont1.warnings.push(warning.clone());
                e_cont2.warnings.push(warning);
                continue;
            }
        };
        let (e_data1, e_data2) = value.key_merge(key);
        e_cont1.data.extend(e_data1);
        e_cont2.data.extend(e_data2);
    }
    reader.end_object()?;
    reader.consume_trailing_whitespace()?;
    Ok((e_cont1, e_cont2))
}

fn try_recover(
    reader: &mut JsonStreamReader<impl std::io::Read>,
    error: DeserializerError,
) -> Result<DeserializerError, ReadParseFailReason> {
    // Consider only custom error as recoverable - they seem to be produced when serde
    // deserialization fails
    match error {
        DeserializerError::Custom(_) => (),
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
