use struson::reader::simple::{SimpleJsonReader, ValueReader};

use crate::util::Error;

pub(in crate::phb) type Key = i32;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::phb) trait KeyMergeOne<EVE> {
    fn key_merge(self, key: Key) -> Vec<EVE>;
}

pub(in crate::phb) trait KeyMergeTwo<EVE1, EVE2> {
    fn key_merge(self, key: Key) -> (Vec<EVE1>, Vec<EVE2>);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Keymap handling
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::phb) fn handle_keymap_one<PHB, EVE>(
    reader: impl std::io::Read,
    suffix: &str,
) -> rc::ed::EResult<rc::ed::EDataCont<EVE>>
where
    PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
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
        let value = match member_reader.read_deserialize::<PHB>() {
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

pub(in crate::phb) fn handle_keymap_two<PHB, EVE1, EVE2>(
    reader: impl std::io::Read,
    suffix: &str,
) -> rc::ed::EResult<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>)>
where
    PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
{
    let mut e_cont1 = rc::ed::EDataCont::new();
    let mut e_cont2 = rc::ed::EDataCont::new();
    let json_reader = SimpleJsonReader::new(reader);
    match json_reader.read_object_borrowed_names(|mut member_reader| {
        let raw_key = member_reader.read_name()?;
        // In case of malformed ID - log error and skip element
        let Ok(key) = raw_key.parse::<Key>() else {
            let warning = format!("failed to cast key \"{}\" to integer", raw_key);
            e_cont1.warns.push(warning.clone());
            e_cont2.warns.push(warning);
            return Ok(());
        };
        let value = match member_reader.read_deserialize::<PHB>() {
            Ok(value) => value,
            // In case of an unexpected value format - log error and skip element
            Err(e) => {
                let warning = format!("failed to parse value with key \"{key}\": {e}");
                e_cont1.warns.push(warning.clone());
                e_cont2.warns.push(warning);
                return Ok(());
            }
        };
        let (e_data1, e_data2) = value.key_merge(key);
        e_cont1.data.extend(e_data1);
        e_cont2.data.extend(e_data2);
        Ok(())
    }) {
        Ok(_) => Ok((e_cont1, e_cont2)),
        Err(e) => Err(Error::PhbUnrecoverableError(suffix.to_string(), e).into()),
    }
}
