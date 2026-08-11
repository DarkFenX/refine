use std::marker::PhantomData;

use serde::de::{Deserializer, MapAccess, Visitor};
use serde_json::value::RawValue;

use super::{error::ReadParseFailReason, warning::cap_len};
use crate::phb::data::{Key, KeyMergeOne, KeyMergeTwo};

const KEY_LEN_LIMIT: usize = 20;
const WARNING_LEN_LIMIT: usize = 200;

pub(in crate::phb) fn extract_from_keymap_one<PHB, EVE>(
    reader: impl std::io::Read,
) -> Result<rc::ed::EDataCont<EVE>, ReadParseFailReason>
where
    PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
{
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let e_cont = deserializer.deserialize_map(KeymapOne::<PHB, EVE>(PhantomData))?;
    deserializer.end()?;
    Ok(e_cont)
}

pub(in crate::phb) fn extract_from_keymap_two<PHB, EVE1, EVE2>(
    reader: impl std::io::Read,
) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), ReadParseFailReason>
where
    PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
{
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let (e_cont1, e_cont2) = deserializer.deserialize_map(KeymapTwo::<PHB, EVE1, EVE2>(PhantomData))?;
    deserializer.end()?;
    Ok((e_cont1, e_cont2))
}

struct KeymapOne<PHB, EVE>(PhantomData<(PHB, EVE)>);
impl<'de, PHB, EVE> Visitor<'de> for KeymapOne<PHB, EVE>
where
    PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
{
    type Value = rc::ed::EDataCont<EVE>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("object with entries keyed by ID")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let size_hint = map.size_hint().unwrap_or(0);
        let mut e_cont = rc::ed::EDataCont::with_capacity(size_hint);
        while let Some(raw_key) = map.next_key::<String>()? {
            let raw_value = map.next_value::<Box<RawValue>>()?;
            // In case of malformed ID - log error and skip element
            let Ok(key) = raw_key.parse::<Key>() else {
                let warning = format!("failed to cast key \"{}\" to integer", cap_len(raw_key, KEY_LEN_LIMIT));
                e_cont.warnings.push(warning);
                continue;
            };
            // In case of malformed value - same, log error and skip element
            let phb = match serde_json::from_str::<PHB>(raw_value.get()) {
                Ok(phb) => phb,
                Err(err) => {
                    let warning = cap_len(
                        format!("failed to parse value with key \"{key}\": {err}"),
                        WARNING_LEN_LIMIT,
                    );
                    e_cont.warnings.push(warning);
                    continue;
                }
            };
            e_cont.data.extend(phb.key_merge(key));
        }
        Ok(e_cont)
    }
}

struct KeymapTwo<PHB, EVE1, EVE2>(PhantomData<(PHB, EVE1, EVE2)>);
impl<'de, PHB, EVE1, EVE2> Visitor<'de> for KeymapTwo<PHB, EVE1, EVE2>
where
    PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
{
    type Value = (rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("object with entries keyed by ID")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let size_hint = map.size_hint().unwrap_or(0);
        let mut e_cont1 = rc::ed::EDataCont::with_capacity(size_hint);
        let mut e_cont2 = rc::ed::EDataCont::with_capacity(size_hint);
        while let Some(raw_key) = map.next_key::<String>()? {
            let raw_value = map.next_value::<Box<RawValue>>()?;
            // In case of malformed ID - log error and skip element
            let Ok(key) = raw_key.parse::<Key>() else {
                let warning = format!("failed to cast key \"{}\" to integer", cap_len(raw_key, KEY_LEN_LIMIT));
                e_cont1.warnings.push(warning.clone());
                e_cont2.warnings.push(warning);
                continue;
            };
            // In case of malformed value - same, log error and skip element
            let phb = match serde_json::from_str::<PHB>(raw_value.get()) {
                Ok(phb) => phb,
                Err(err) => {
                    let warning = cap_len(
                        format!("failed to parse value with key \"{key}\": {err}"),
                        WARNING_LEN_LIMIT,
                    );
                    e_cont1.warnings.push(warning.clone());
                    e_cont2.warnings.push(warning);
                    continue;
                }
            };
            let (e_data1, e_data2) = phb.key_merge(key);
            e_cont1.data.extend(e_data1);
            e_cont2.data.extend(e_data2);
        }
        Ok((e_cont1, e_cont2))
    }
}
