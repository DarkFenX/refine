#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum HSecZone {
    #[serde(rename = "hisec")]
    HiSec,
    #[serde(rename = "corrupted_hisec")]
    CorruptedHiSec,
    #[serde(rename = "lowsec")]
    LowSec,
    #[serde(rename = "corrupted_lowsec")]
    CorruptedLowSec,
    #[serde(rename = "nullsec")]
    NullSec,
    #[serde(rename = "wspace")]
    WSpace,
    #[serde(rename = "hazard")]
    Hazard,
}
impl From<&rc::SolSecZone> for HSecZone {
    fn from(core_sec_zone: &rc::SolSecZone) -> Self {
        match core_sec_zone {
            rc::SolSecZone::HiSec(rc::SolSecZoneCorruption::Any) => Self::HiSec,
            rc::SolSecZone::HiSec(rc::SolSecZoneCorruption::Full) => Self::CorruptedHiSec,
            rc::SolSecZone::LowSec(rc::SolSecZoneCorruption::Any) => Self::LowSec,
            rc::SolSecZone::LowSec(rc::SolSecZoneCorruption::Full) => Self::CorruptedLowSec,
            rc::SolSecZone::NullSec => Self::NullSec,
            rc::SolSecZone::WSpace => Self::WSpace,
            rc::SolSecZone::Hazard => Self::Hazard,
        }
    }
}
impl From<&HSecZone> for rc::SolSecZone {
    fn from(h_sec_zone: &HSecZone) -> Self {
        match h_sec_zone {
            HSecZone::HiSec => Self::HiSec(rc::SolSecZoneCorruption::Any),
            HSecZone::CorruptedHiSec => Self::HiSec(rc::SolSecZoneCorruption::Full),
            HSecZone::LowSec => Self::LowSec(rc::SolSecZoneCorruption::Any),
            HSecZone::CorruptedLowSec => Self::LowSec(rc::SolSecZoneCorruption::Full),
            HSecZone::NullSec => Self::NullSec,
            HSecZone::WSpace => Self::WSpace,
            HSecZone::Hazard => Self::Hazard,
        }
    }
}
