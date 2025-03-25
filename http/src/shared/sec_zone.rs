#[derive(serde::Serialize, serde::Deserialize)]
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
impl From<&rc::SecZone> for HSecZone {
    fn from(core_sec_zone: &rc::SecZone) -> Self {
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
}
impl From<&HSecZone> for rc::SecZone {
    fn from(h_sec_zone: &HSecZone) -> Self {
        match h_sec_zone {
            HSecZone::HiSec => Self::HiSec(rc::SecZoneCorruption::None),
            HSecZone::HiSecC5 => Self::HiSec(rc::SecZoneCorruption::C5),
            HSecZone::LowSec => Self::LowSec(rc::SecZoneCorruption::None),
            HSecZone::LowSecC5 => Self::LowSec(rc::SecZoneCorruption::C5),
            HSecZone::NullSec => Self::NullSec,
            HSecZone::WSpace => Self::WSpace,
            HSecZone::Hazard => Self::Hazard,
        }
    }
}
