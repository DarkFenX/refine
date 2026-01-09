use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) enum HSecZone {
    #[serde(rename = "hisec")]
    HiSec,
    #[serde(rename = "hisec_c5")]
    HiSecC5,
    #[serde(rename = "lowsec")]
    LowSec,
    #[serde(rename = "lowsec_c5")]
    LowSecC5,
    #[serde(rename = "nullsec")]
    NullSec,
    #[serde(rename = "wspace")]
    WSpace,
    #[serde(rename = "hazard")]
    Hazard,
}
impl HSecZone {
    pub(crate) fn from_core(core_sec_zone: rc::SecZone) -> Self {
        match core_sec_zone {
            rc::SecZone::HiSec(rc::SecZoneCorruption::None) => Self::HiSec,
            rc::SecZone::HiSec(rc::SecZoneCorruption::C5) => Self::HiSecC5,
            rc::SecZone::LowSec(rc::SecZoneCorruption::None) => Self::LowSec,
            rc::SecZone::LowSec(rc::SecZoneCorruption::C5) => Self::LowSecC5,
            rc::SecZone::NullSec => Self::NullSec,
            rc::SecZone::WSpace => Self::WSpace,
            rc::SecZone::Hazard => Self::Hazard,
        }
    }
    pub(crate) fn into_core(self) -> rc::SecZone {
        match self {
            Self::HiSec => rc::SecZone::HiSec(rc::SecZoneCorruption::None),
            Self::HiSecC5 => rc::SecZone::HiSec(rc::SecZoneCorruption::C5),
            Self::LowSec => rc::SecZone::LowSec(rc::SecZoneCorruption::None),
            Self::LowSecC5 => rc::SecZone::LowSec(rc::SecZoneCorruption::C5),
            Self::NullSec => rc::SecZone::NullSec,
            Self::WSpace => rc::SecZone::WSpace,
            Self::Hazard => rc::SecZone::Hazard,
        }
    }
}
