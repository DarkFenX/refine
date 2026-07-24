use crate::ad::{AAttrId, ABuffId, AEffectModStrength, AItemListId};

#[derive(Clone)]
pub struct AEffectBuff {
    pub attr_merge: Option<AEffectBuffAttrMerge> = None,
    pub full: Vec<AEffectBuffFull> = Vec::new(),
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
#[derive(Copy, Clone)]
pub struct AEffectBuffAttrMerge {
    pub duration: AEffectBuffDuration,
    pub scope: AEffectBuffScope,
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
#[derive(Copy, Clone)]
pub struct AEffectBuffFull {
    pub buff_id: ABuffId,
    pub strength: AEffectModStrength,
    pub duration: AEffectBuffDuration,
    pub scope: AEffectBuffScope,
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone)]
pub enum AEffectBuffDuration {
    Effect,
    AttrS(AAttrId),
    AttrMs(AAttrId),
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone)]
pub enum AEffectBuffScope {
    Carrier,
    Projected(AItemListId),
    Fleet(AItemListId),
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

    impl Serialize for AEffectBuff {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.attr_merge)?;
            tuple.serialize_element(&self.full)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for AEffectBuff {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AEffectBuff;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 2 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        attr_merge: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        full: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                    })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
