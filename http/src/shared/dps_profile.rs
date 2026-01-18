use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum HDpsProfile {
    Full(HDpsProfileFull),
    Short(HDpsProfileShort),
}

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple)]
pub(crate) struct HDpsProfileFull {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
    breacher: Option<(f64, f64)>,
}

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple)]
pub(crate) struct HDpsProfileShort {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDpsProfile {
    pub(crate) fn from_core(core_dps_profile: rc::DpsProfile) -> Self {
        Self::Full(HDpsProfileFull {
            em: core_dps_profile.get_em().into_f64(),
            thermal: core_dps_profile.get_thermal().into_f64(),
            kinetic: core_dps_profile.get_kinetic().into_f64(),
            explosive: core_dps_profile.get_explosive().into_f64(),
            breacher: core_dps_profile
                .get_breacher()
                .map(|v| (v.get_absolute_max().into_f64(), v.get_relative_max().into_f64())),
        })
    }
    pub(crate) fn into_core(self) -> rc::DpsProfile {
        let core_breacher = self.get_breacher().map(|(br_abs, br_rel)| {
            rc::Breacher::new(
                rc::PValue::from_f64_clamped(br_abs),
                rc::UnitInterval::from_f64_clamped(br_rel),
            )
        });
        rc::DpsProfile::new(
            rc::PValue::from_f64_clamped(self.get_em()),
            rc::PValue::from_f64_clamped(self.get_thermal()),
            rc::PValue::from_f64_clamped(self.get_kinetic()),
            rc::PValue::from_f64_clamped(self.get_explosive()),
            core_breacher,
        )
    }
    fn get_em(&self) -> f64 {
        match self {
            Self::Full(dps_profile) => dps_profile.em,
            Self::Short(dps_profile) => dps_profile.em,
        }
    }
    fn get_thermal(&self) -> f64 {
        match self {
            Self::Full(dps_profile) => dps_profile.thermal,
            Self::Short(dps_profile) => dps_profile.thermal,
        }
    }
    fn get_kinetic(&self) -> f64 {
        match self {
            Self::Full(dps_profile) => dps_profile.kinetic,
            Self::Short(dps_profile) => dps_profile.kinetic,
        }
    }
    fn get_explosive(&self) -> f64 {
        match self {
            Self::Full(dps_profile) => dps_profile.explosive,
            Self::Short(dps_profile) => dps_profile.explosive,
        }
    }
    fn get_breacher(&self) -> Option<(f64, f64)> {
        match self {
            Self::Full(dps_profile) => dps_profile.breacher,
            Self::Short(_) => None,
        }
    }
}
