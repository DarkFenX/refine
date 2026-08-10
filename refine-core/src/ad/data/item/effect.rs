use crate::{
    ad::{AEffectId, AItemEffectData},
    util::CMap,
};

pub struct AItemEffect {
    pub id: AEffectId,
    pub data: AItemEffectData = AItemEffectData::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct AItemEffects {
    data: CMap<AEffectId, AItemEffect> = CMap::const_new(),
}
impl AItemEffects {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemEffect) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemEffect> {
        self.data.values()
    }
}
impl FromIterator<AItemEffect> for AItemEffects {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemEffect>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemEffects {
    pub(crate) fn contains_id(&self, id: &AEffectId) -> bool {
        self.data.contains_key(id)
    }
    pub(in crate::ad) fn keys(&self) -> impl ExactSizeIterator<Item = &AEffectId> {
        self.data.keys()
    }
    pub(in crate::ad) fn get_mut(&mut self, id: &AEffectId) -> Option<&mut AItemEffect> {
        self.data.get_mut(id)
    }
    pub(in crate::ad) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut AItemEffect> {
        self.data.values_mut()
    }
    pub(in crate::ad) fn remove(&mut self, id: &AEffectId) -> Option<AItemEffect> {
        self.data.remove(id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad_entry {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::*;

    const FIELDS: usize = 2;

    impl Serialize for AItemEffect {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(FIELDS)?;
            tuple.serialize_element(&self.id)?;
            tuple.serialize_element(&self.data)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for AItemEffect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemEffect;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("tuple with 2 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        id: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        data: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                    })
                }
            }

            deserializer.deserialize_tuple(FIELDS, VisitorImpl)
        }
    }
}

#[cfg(feature = "serde-ad")]
mod custom_serde_ad_container {
    use serde::{
        de::{Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeSeq, Serializer},
    };

    use super::*;

    impl Serialize for AItemEffects {
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

    impl<'de> Deserialize<'de> for AItemEffects {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemEffects;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with item effects")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let size_hint = seq.size_hint().unwrap_or(0);
                    let mut data = CMap::const_with_capacity(size_hint);
                    while let Some(element) = seq.next_element::<AItemEffect>()? {
                        data.insert(element.id, element);
                    }
                    Ok(AItemEffects { data })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
