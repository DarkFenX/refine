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
impl From<&rc::SolSecZone> for HSecZone {
    fn from(core_sec_zone: &rc::SolSecZone) -> Self {
        match core_sec_zone {
            rc::SolSecZone::HiSec(rc::SolSecZoneCorruption::None) => Self::HiSec,
            rc::SolSecZone::HiSec(rc::SolSecZoneCorruption::C5) => Self::HiSecC5,
            rc::SolSecZone::LowSec(rc::SolSecZoneCorruption::None) => Self::LowSec,
            rc::SolSecZone::LowSec(rc::SolSecZoneCorruption::C5) => Self::LowSecC5,
            rc::SolSecZone::NullSec => Self::NullSec,
            rc::SolSecZone::WSpace => Self::WSpace,
            rc::SolSecZone::Hazard => Self::Hazard,
        }
    }
}
impl From<&HSecZone> for rc::SolSecZone {
    fn from(h_sec_zone: &HSecZone) -> Self {
        match h_sec_zone {
            HSecZone::HiSec => Self::HiSec(rc::SolSecZoneCorruption::None),
            HSecZone::HiSecC5 => Self::HiSec(rc::SolSecZoneCorruption::C5),
            HSecZone::LowSec => Self::LowSec(rc::SolSecZoneCorruption::None),
            HSecZone::LowSecC5 => Self::LowSec(rc::SolSecZoneCorruption::C5),
            HSecZone::NullSec => Self::NullSec,
            HSecZone::WSpace => Self::WSpace,
            HSecZone::Hazard => Self::Hazard,
        }
    }
}
