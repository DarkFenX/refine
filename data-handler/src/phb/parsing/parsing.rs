use struson::reader::{JsonReader, JsonStreamReader};

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
            e_cont.warns.push(warning);
            reader.skip_value()?;
            continue;
        };
        let raw_value = reader.deserialize_next::<serde_json::Value>()?;
        let value = match serde_json::from_value::<PHB>(raw_value) {
            Ok(value) => value,
            // In case of an unexpected value format - log error and skip element
            Err(e) => {
                let warning = format!("failed to parse value with key \"{key}\": {e}");
                e_cont.warns.push(warning);
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
            e_cont1.warns.push(warning.clone());
            e_cont2.warns.push(warning);
            reader.skip_value()?;
            continue;
        };
        let raw_value = reader.deserialize_next::<serde_json::Value>()?;
        let value = match serde_json::from_value::<PHB>(raw_value) {
            Ok(value) => value,
            // In case of an unexpected value format - log error and skip element
            Err(e) => {
                let warning = format!("failed to parse value with key \"{key}\": {e}");
                e_cont1.warns.push(warning.clone());
                e_cont2.warns.push(warning);
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
