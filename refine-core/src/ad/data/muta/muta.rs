use crate::ad::{AItemId, AMutaAttrs, AMutaItemConvs};

pub struct AMuta {
    pub id: AItemId,
    pub item_map: AMutaItemConvs = AMutaItemConvs::new(),
    pub attr_mods: AMutaAttrs = AMutaAttrs::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::*;

    const FIELDS: usize = 3;

    impl Serialize for AMuta {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(FIELDS)?;
            tuple.serialize_element(&self.id)?;
            tuple.serialize_element(&self.item_map)?;
            tuple.serialize_element(&self.attr_mods)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for AMuta {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AMuta;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 3 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        id: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        item_map: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                        attr_mods: seq.next_element()?.ok_or(Error::invalid_length(2, &self))?,
                    })
                }
            }

            deserializer.deserialize_tuple(FIELDS, VisitorImpl)
        }
    }
}
