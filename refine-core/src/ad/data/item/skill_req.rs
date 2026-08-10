use crate::{
    ad::{AItemId, ASkillLevel},
    util::CMap,
};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AItemSkillReq {
    pub id: AItemId,
    pub level: ASkillLevel,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct AItemSkillReqs {
    data: CMap<AItemId, AItemSkillReq> = CMap::const_new(),
}
impl AItemSkillReqs {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemSkillReq) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemSkillReq> {
        self.data.values()
    }
}
impl FromIterator<AItemSkillReq> for AItemSkillReqs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemSkillReq>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemSkillReqs {
    pub(crate) fn contains_id(&self, id: &AItemId) -> bool {
        self.data.contains_key(id)
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

    impl Serialize for AItemSkillReqs {
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

    impl<'de> Deserialize<'de> for AItemSkillReqs {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemSkillReqs;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with item skill requirements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let size_hint = seq.size_hint().unwrap_or(0);
                    let mut data = CMap::const_with_capacity(size_hint);
                    while let Some(element) = seq.next_element::<AItemSkillReq>()? {
                        data.insert(element.id, element);
                    }
                    Ok(AItemSkillReqs { data })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
