use struson::{
    reader::{JsonReader, JsonStreamReader},
    writer::{JsonStreamWriter, JsonWriter},
};

use super::error::{JsonZfileAdcDataReadError, JsonZfileAdcWriteError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn try_serialize<W>(a_data: &rc::ad::AData, writer: W) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
{
    let mut writer = JsonStreamWriter::new(writer);
    writer.begin_object()?;
    write_array_to_object("items", a_data.items.iter(), &mut writer)?;
    write_array_to_object("attrs", a_data.attrs.iter(), &mut writer)?;
    write_array_to_object("mutas", a_data.mutas.iter(), &mut writer)?;
    write_array_to_object("effects", a_data.effects.iter(), &mut writer)?;
    write_array_to_object("buffs", a_data.buffs.iter(), &mut writer)?;
    write_array_to_object("abils", a_data.abils.iter(), &mut writer)?;
    write_array_to_object("item_lists", a_data.item_lists.iter(), &mut writer)?;
    writer.name("warnings")?;
    writer.serialize_value(&a_data.warnings)?;
    writer.end_object()?;
    writer.finish_document()?;
    Ok(())
}

fn write_array_to_object<'a, W, T>(
    name: &str,
    entities: impl Iterator<Item = &'a T>,
    writer: &mut JsonStreamWriter<W>,
) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    T: serde::ser::Serialize + 'a,
{
    writer.name(name)?;
    write_array(entities, writer)
}

fn write_array<'a, W, T>(
    entities: impl Iterator<Item = &'a T>,
    writer: &mut JsonStreamWriter<W>,
) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    T: serde::ser::Serialize + 'a,
{
    writer.begin_array()?;
    for entity in entities {
        writer.serialize_value(entity)?;
    }
    writer.end_array()?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn try_deserialize<R>(reader: R) -> Result<rc::ad::AData, JsonZfileAdcDataReadError>
where
    R: std::io::Read,
{
    let mut a_data = rc::ad::AData::new();
    let mut reader = JsonStreamReader::new(reader);
    reader.begin_object()?;
    while reader.has_next()? {
        match reader.next_name()? {
            "items" => read_array(|e| a_data.items.insert(e), &mut reader)?,
            "attrs" => read_array(|e| a_data.attrs.insert(e), &mut reader)?,
            "mutas" => read_array(|e| a_data.mutas.insert(e), &mut reader)?,
            "effects" => read_array(|e| a_data.effects.insert(e), &mut reader)?,
            "buffs" => read_array(|e| a_data.buffs.insert(e), &mut reader)?,
            "abils" => read_array(|e| a_data.abils.insert(e), &mut reader)?,
            "item_lists" => read_array(|e| a_data.item_lists.insert(e), &mut reader)?,
            "warnings" => a_data.warnings = reader.deserialize_next()?,
            _ => reader.skip_value()?,
        }
    }
    reader.end_object()?;
    Ok(a_data)
}

fn read_array<F, T, R>(mut inserter: F, reader: &mut JsonStreamReader<R>) -> Result<(), JsonZfileAdcDataReadError>
where
    F: FnMut(T),
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
{
    reader.begin_array()?;
    while reader.has_next()? {
        let entity = reader.deserialize_next::<T>()?;
        inserter(entity);
    }
    reader.end_array()?;
    Ok(())
}
