use struson::reader::{JsonReader, JsonStreamReader};

use crate::cacher_json::{
    data::{AdaptedConv, CAbil, CAttr, CBuff, CEffect, CItem, CItemList, CMuta},
    error::JsonZfileAdcError,
};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub(in crate::cacher_json) struct CData {
    items: Vec<CItem>,
    attrs: Vec<CAttr>,
    mutas: Vec<CMuta>,
    effects: Vec<CEffect>,
    buffs: Vec<CBuff>,
    abils: Vec<CAbil>,
    item_lists: Vec<CItemList>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////

impl CData {
    pub(in crate::cacher_json) fn from_adapted(a_data: &rc::ad::AData) -> Self {
        Self {
            items: a_data.items.iter().map(CItem::from_adapted).collect(),
            attrs: a_data.attrs.iter().map(CAttr::from_adapted).collect(),
            mutas: a_data.mutas.iter().map(CMuta::from_adapted).collect(),
            effects: a_data.effects.iter().map(CEffect::from_adapted).collect(),
            buffs: a_data.buffs.iter().map(CBuff::from_adapted).collect(),
            abils: a_data.abils.iter().map(CAbil::from_adapted).collect(),
            item_lists: a_data.item_lists.iter().map(CItemList::from_adapted).collect(),
        }
    }
    pub(in crate::cacher_json) fn into_adapted(self) -> rc::ad::AData {
        rc::ad::AData {
            items: self.items.into_iter().map(|v| v.into_adapted()).collect(),
            attrs: self.attrs.into_iter().map(|v| v.into_adapted()).collect(),
            mutas: self.mutas.into_iter().map(|v| v.into_adapted()).collect(),
            effects: self.effects.into_iter().map(|v| v.into_adapted()).collect(),
            buffs: self.buffs.into_iter().map(|v| v.into_adapted()).collect(),
            abils: self.abils.into_iter().map(|v| v.into_adapted()).collect(),
            item_lists: self.item_lists.into_iter().map(|v| v.into_adapted()).collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CData {
    pub(in crate::cacher_json) fn try_deserialize<R>(reader: R) -> Result<Self, JsonZfileAdcError>
    where
        R: std::io::Read,
    {
        let mut c_data = CData::default();
        let mut reader = JsonStreamReader::new(reader);
        reader.begin_object()?;
        while reader.has_next()? {
            match reader.next_name()? {
                "items" => read_array(&mut c_data.items, &mut reader)?,
                "attrs" => read_array(&mut c_data.attrs, &mut reader)?,
                "mutas" => read_array(&mut c_data.mutas, &mut reader)?,
                "effects" => read_array(&mut c_data.effects, &mut reader)?,
                "buffs" => read_array(&mut c_data.buffs, &mut reader)?,
                "abils" => read_array(&mut c_data.abils, &mut reader)?,
                "item_lists" => read_array(&mut c_data.item_lists, &mut reader)?,
                _ => reader.skip_value()?,
            }
        }
        reader.end_object()?;
        Ok(c_data)
    }
}

fn read_array<R, C>(c_entities: &mut Vec<C>, reader: &mut JsonStreamReader<R>) -> Result<(), JsonZfileAdcError>
where
    R: std::io::Read,
    C: serde::de::DeserializeOwned,
{
    reader.begin_array()?;
    while reader.has_next()? {
        let entry = reader.deserialize_next::<C>()?;
        c_entities.push(entry);
    }
    reader.end_array()?;
    Ok(())
}
