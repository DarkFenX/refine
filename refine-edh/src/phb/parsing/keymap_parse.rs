use std::marker::PhantomData;

use serde::de::{Deserializer, MapAccess, Visitor};
use serde_json::value::RawValue;

use super::error::ReadParseFailReason;
use crate::{
    phb::data::{Key, KeyMergeOne, KeyMergeTwo},
    shared::{cap_len, cap_warning_len},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interface methods
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Visitors
////////////////////////////////////////////////////////////////////////////////////////////////////
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
        for_each_entry::<PHB, _>(&mut map, &mut e_cont.warnings, |key, phb| {
            phb.key_merge(key, &mut e_cont.data);
        })?;
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
        let mut warnings = Vec::new();
        for_each_entry::<PHB, _>(&mut map, &mut warnings, |key, phb| {
            phb.key_merge(key, &mut e_cont1.data, &mut e_cont2.data);
        })?;
        e_cont1.warnings = warnings.clone();
        e_cont2.warnings = warnings;
        Ok((e_cont1, e_cont2))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
fn for_each_entry<'de, PHB, A>(
    map: &mut A,
    warnings: &mut Vec<String>,
    mut process: impl FnMut(Key, PHB),
) -> Result<(), A::Error>
where
    PHB: serde::de::DeserializeOwned,
    A: MapAccess<'de>,
{
    while let Some(raw_key) = map.next_key::<String>()? {
        // Value has to be taken even when its key is unusable, to advance to the next entry
        let raw_value = map.next_value::<Box<RawValue>>()?;
        // In case of malformed key - log a warning and skip element
        let Ok(key) = raw_key.parse::<Key>() else {
            let warning = format!("failed to cast key \"{}\" to integer", cap_len(raw_key, 20));
            warnings.push(warning);
            continue;
        };
        // In case of malformed value - same, log a warning and skip element
        let phb = match serde_json::from_str::<PHB>(raw_value.get()) {
            Ok(phb) => phb,
            Err(err) => {
                let warning = cap_warning_len(format!("failed to parse value with key \"{key}\": {err}"));
                warnings.push(warning);
                continue;
            }
        };
        process(key, phb);
    }
    Ok(())
}
