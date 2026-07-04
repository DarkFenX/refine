use struson::{
    reader::{JsonReader, JsonStreamReader},
    writer::{JsonStreamWriter, JsonWriter},
};

use crate::cacher_json::{
    data::{AdaptedConv, CAbil, CAttr, CBuff, CDataWarnings, CEffect, CItem, CItemList, CMuta},
    error::{JsonZfileAdcReadError, JsonZfileAdcWriteError},
};

#[derive(Default)]
pub(in crate::cacher_json) struct CData {
    items: Vec<CItem>,
    attrs: Vec<CAttr>,
    mutas: Vec<CMuta>,
    effects: Vec<CEffect>,
    buffs: Vec<CBuff>,
    abils: Vec<CAbil>,
    item_lists: Vec<CItemList>,
    warnings: CDataWarnings,
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
            warnings: CDataWarnings::from_adapted(&a_data.warnings),
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
            warnings: self.warnings.into_adapted(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CData {
    pub(in crate::cacher_json) fn try_serialize<W>(&self, writer: W) -> Result<(), JsonZfileAdcWriteError>
    where
        W: std::io::Write,
    {
        let mut writer = JsonStreamWriter::new(writer);
        writer.begin_object()?;
        write_array_to_object("items", &self.items, &mut writer)?;
        write_array_to_object("attrs", &self.attrs, &mut writer)?;
        write_array_to_object("mutas", &self.mutas, &mut writer)?;
        write_array_to_object("effects", &self.effects, &mut writer)?;
        write_array_to_object("buffs", &self.buffs, &mut writer)?;
        write_array_to_object("abils", &self.abils, &mut writer)?;
        write_array_to_object("item_lists", &self.item_lists, &mut writer)?;
        writer.name("warnings")?;
        writer.serialize_value(&self.warnings)?;
        writer.end_object()?;
        writer.finish_document()?;
        Ok(())
    }
}

fn write_array_to_object<W, C>(
    name: &str,
    c_entities: &[C],
    writer: &mut JsonStreamWriter<W>,
) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    C: serde::ser::Serialize,
{
    writer.name(name)?;
    write_array(c_entities, writer)
}

fn write_array<W, C>(c_entities: &[C], writer: &mut JsonStreamWriter<W>) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    C: serde::ser::Serialize,
{
    writer.begin_array()?;
    for c_entity in c_entities {
        writer.serialize_value(c_entity)?;
    }
    writer.end_array()?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CData {
    pub(in crate::cacher_json) fn try_deserialize<R>(reader: R) -> Result<Self, JsonZfileAdcReadError>
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
                "warnings" => c_data.warnings = reader.deserialize_next()?,
                _ => reader.skip_value()?,
            }
        }
        reader.end_object()?;
        Ok(c_data)
    }
}

fn read_array<R, C>(c_entities: &mut Vec<C>, reader: &mut JsonStreamReader<R>) -> Result<(), JsonZfileAdcReadError>
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
