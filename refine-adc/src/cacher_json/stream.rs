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
    c_entities: impl Iterator<Item = &'a T>,
    writer: &mut JsonStreamWriter<W>,
) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    T: serde::ser::Serialize + 'a,
{
    writer.name(name)?;
    write_array(c_entities, writer)
}

fn write_array<'a, W, T>(
    c_entities: impl Iterator<Item = &'a T>,
    writer: &mut JsonStreamWriter<W>,
) -> Result<(), JsonZfileAdcWriteError>
where
    W: std::io::Write,
    T: serde::ser::Serialize + 'a,
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
pub(super) fn try_deserialize<R>(reader: R) -> Result<rc::ad::AData, JsonZfileAdcDataReadError>
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

fn read_array<R, C>(c_entities: &mut Vec<C>, reader: &mut JsonStreamReader<R>) -> Result<(), JsonZfileAdcDataReadError>
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
