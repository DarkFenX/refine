use crate::ad::{AAbils, AAttrs, ABuffs, ADataWarnings, AEffects, AItemLists, AItems, AMutas};

#[derive(Default)]
pub struct AData {
    pub items: AItems,
    pub attrs: AAttrs,
    pub mutas: AMutas,
    pub effects: AEffects,
    pub buffs: ABuffs,
    pub abils: AAbils,
    pub item_lists: AItemLists,
    pub warnings: ADataWarnings,
}
impl AData {
    pub fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    //! The reason to have custom serialization here is to allow human-readable format to skip
    //! fields while not breaking binary format support

    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    use super::*;

    // Human-readable representation
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "AData")]
    struct ADataHrDef {
        #[serde(default, skip_serializing_if = "AItems::is_empty")]
        items: AItems,
        #[serde(default, skip_serializing_if = "AAttrs::is_empty")]
        attrs: AAttrs,
        #[serde(default, skip_serializing_if = "AMutas::is_empty")]
        mutas: AMutas,
        #[serde(default, skip_serializing_if = "AEffects::is_empty")]
        effects: AEffects,
        #[serde(default, skip_serializing_if = "ABuffs::is_empty")]
        buffs: ABuffs,
        #[serde(default, skip_serializing_if = "AAbils::is_empty")]
        abils: AAbils,
        #[serde(default, skip_serializing_if = "AItemLists::is_empty")]
        item_lists: AItemLists,
        #[serde(default, skip_serializing_if = "ADataWarnings::is_empty")]
        warnings: ADataWarnings,
    }

    // Binary representation
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "AData")]
    struct ADataBinDef {
        items: AItems,
        attrs: AAttrs,
        mutas: AMutas,
        effects: AEffects,
        buffs: ABuffs,
        abils: AAbils,
        item_lists: AItemLists,
        warnings: ADataWarnings,
    }

    // Serialization
    impl Serialize for AData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match serializer.is_human_readable() {
                true => ADataHrDef::serialize(self, serializer),
                false => ADataBinDef::serialize(self, serializer),
            }
        }
    }

    // Deserialization
    impl<'de> Deserialize<'de> for AData {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match deserializer.is_human_readable() {
                true => ADataHrDef::deserialize(deserializer),
                false => ADataBinDef::deserialize(deserializer),
            }
        }
    }
}
