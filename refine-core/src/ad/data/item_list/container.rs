use crate::{
    ad::{AItemList, AItemListId},
    util::RMap,
};

#[derive(Default)]
pub struct AItemLists {
    pub(crate) data: RMap<AItemListId, AItemList>,
}
impl AItemLists {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AItemList) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemList> {
        self.data.values()
    }
}
impl FromIterator<AItemList> for AItemLists {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemList>,
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

    impl Serialize for AItemLists {
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

    impl<'de> Deserialize<'de> for AItemLists {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemLists;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with item lists")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let mut data = RMap::new();
                    while let Some(element) = seq.next_element::<AItemList>()? {
                        data.insert(element.id, element);
                    }
                    Ok(AItemLists { data })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}
