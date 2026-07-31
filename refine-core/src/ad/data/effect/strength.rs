use crate::ad::{AAttrId, AValue};

#[derive(Copy, Clone, PartialEq)]
pub enum AEffectModStrength {
    Attr(AAttrId),
    Hardcoded(AValue),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    use super::*;

    // Human-readable representation - relies on knowledge that variants are serialized differently
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "AEffectModStrength")]
    #[serde(untagged)]
    enum AEffectModStrengthHrDef {
        Attr(AAttrId),
        Hardcoded(AValue),
    }

    // Binary representation
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "AEffectModStrength")]
    enum AEffectModStrengthBinDef {
        Attr(AAttrId),
        Hardcoded(AValue),
    }

    // Serialization
    impl Serialize for AEffectModStrength {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match serializer.is_human_readable() {
                true => AEffectModStrengthHrDef::serialize(self, serializer),
                false => AEffectModStrengthBinDef::serialize(self, serializer),
            }
        }
    }

    // Deserialization
    impl<'de> Deserialize<'de> for AEffectModStrength {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match deserializer.is_human_readable() {
                true => AEffectModStrengthHrDef::deserialize(deserializer),
                false => AEffectModStrengthBinDef::deserialize(deserializer),
            }
        }
    }
}
