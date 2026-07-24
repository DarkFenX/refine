use crate::ad::{AAttrId, AValue};

pub struct AAttr {
    pub id: AAttrId,
    pub penalizable: bool,
    pub hig: bool,
    pub def_val: AValue,
    pub min_attr_id: Option<AAttrId> = None,
    pub max_attr_id: Option<AAttrId> = None,
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

    impl Serialize for AAttr {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(6)?;
            tuple.serialize_element(&self.id)?;
            tuple.serialize_element(&self.penalizable)?;
            tuple.serialize_element(&self.hig)?;
            tuple.serialize_element(&self.def_val)?;
            tuple.serialize_element(&self.min_attr_id)?;
            tuple.serialize_element(&self.max_attr_id)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for AAttr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AAttr;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 6 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        id: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        penalizable: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                        hig: seq.next_element()?.ok_or(Error::invalid_length(2, &self))?,
                        def_val: seq.next_element()?.ok_or(Error::invalid_length(3, &self))?,
                        min_attr_id: seq.next_element()?.ok_or(Error::invalid_length(4, &self))?,
                        max_attr_id: seq.next_element()?.ok_or(Error::invalid_length(5, &self))?,
                    })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
