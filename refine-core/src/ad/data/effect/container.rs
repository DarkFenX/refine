use crate::{
    ad::{AEffect, AEffectId},
    util::RMap,
};

#[derive(Default)]
pub struct AEffects {
    pub(crate) data: RMap<AEffectId, AEffect>,
}
impl AEffects {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AEffect) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AEffect> {
        self.data.values()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl FromIterator<AEffect> for AEffects {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AEffect>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
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

    impl Serialize for AEffects {
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

    impl<'de> Deserialize<'de> for AEffects {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AEffects;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with effects")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let size_hint = seq.size_hint().unwrap_or(0);
                    let mut data = RMap::with_capacity(size_hint);
                    while let Some(element) = seq.next_element::<AEffect>()? {
                        data.insert(element.id, element);
                    }
                    Ok(AEffects { data })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
