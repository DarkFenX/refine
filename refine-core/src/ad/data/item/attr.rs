use crate::{
    ad::{AAttrId, AValue},
    util::CMap,
};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AItemAttr {
    pub id: AAttrId,
    pub value: AValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct AItemAttrs {
    data: CMap<AAttrId, AItemAttr> = CMap::const_new(),
}
impl AItemAttrs {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemAttr) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemAttr> {
        self.data.values()
    }
}
impl FromIterator<AItemAttr> for AItemAttrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemAttr>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemAttrs {
    pub(in crate::ad) fn contains_id(&self, id: &AAttrId) -> bool {
        self.data.contains_key(id)
    }
    pub(in crate::ad) fn entry(&mut self, id: AAttrId) -> std::collections::hash_map::Entry<'_, AAttrId, AItemAttr> {
        self.data.entry(id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use serde::{
        de::{Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeSeq, Serializer},
    };

    use super::*;

    impl Serialize for AItemAttrs {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(self.data.len()))?;
            for attr in self.data.values() {
                seq.serialize_element(attr)?;
            }
            seq.end()
        }
    }

    impl<'de> Deserialize<'de> for AItemAttrs {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemAttrs;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with item attributes")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let mut data = CMap::const_new();
                    while let Some(element) = seq.next_element::<AItemAttr>()? {
                        data.insert(element.id, element);
                    }
                    Ok(AItemAttrs { data })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
