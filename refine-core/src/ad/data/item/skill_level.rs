use crate::util::round_f64_to_i32;

#[cfg_attr(feature = "serde-ad", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ASkillLevel(u8);
impl ASkillLevel {
    pub fn from_u8_clamped(level: u8) -> Self {
        Self(level.clamp(0, 5))
    }
    pub fn from_i32_clamped(level: i32) -> Self {
        Self(level.clamp(0, 5) as u8)
    }
    pub fn into_u8(self) -> u8 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ASkillLevel {
    pub(in crate::ad) fn from_f64_rounded_clamped(level: f64) -> Self {
        Self::from_i32_clamped(round_f64_to_i32(level))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de> Deserialize<'de> for ASkillLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            i32::deserialize(deserializer).map(ASkillLevel::from_i32_clamped)
        }
    }
}
