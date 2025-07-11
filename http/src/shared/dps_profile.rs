use crate::util::HExecError;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HDpsProfile {
    Full(HDpsProfileFull),
    Short(HDpsProfileShort),
}
impl HDpsProfile {
    fn get_em(&self) -> rc::AttrVal {
        match self {
            Self::Full(dps_profile) => dps_profile.em,
            Self::Short(dps_profile) => dps_profile.em,
        }
    }
    fn get_thermal(&self) -> rc::AttrVal {
        match self {
            Self::Full(dps_profile) => dps_profile.thermal,
            Self::Short(dps_profile) => dps_profile.thermal,
        }
    }
    fn get_kinetic(&self) -> rc::AttrVal {
        match self {
            Self::Full(dps_profile) => dps_profile.kinetic,
            Self::Short(dps_profile) => dps_profile.kinetic,
        }
    }
    fn get_explosive(&self) -> rc::AttrVal {
        match self {
            Self::Full(dps_profile) => dps_profile.explosive,
            Self::Short(dps_profile) => dps_profile.explosive,
        }
    }
    fn get_breacher(&self) -> Option<(rc::AttrVal, rc::AttrVal)> {
        match self {
            Self::Full(dps_profile) => dps_profile.breacher,
            Self::Short(_) => None,
        }
    }
}
impl From<rc::DpsProfile> for HDpsProfile {
    fn from(core_dps_profile: rc::DpsProfile) -> Self {
        Self::Full(HDpsProfileFull {
            em: core_dps_profile.get_em(),
            thermal: core_dps_profile.get_thermal(),
            kinetic: core_dps_profile.get_kinetic(),
            explosive: core_dps_profile.get_explosive(),
            breacher: core_dps_profile
                .get_breacher()
                .map(|v| (v.get_absolute_max(), v.get_relative_max())),
        })
    }
}
impl TryFrom<HDpsProfile> for rc::DpsProfile {
    type Error = HExecError;

    fn try_from(h_dps_profile: HDpsProfile) -> Result<Self, Self::Error> {
        let breacher_info = match h_dps_profile.get_breacher() {
            Some((br_abs, br_rel)) => match rc::BreacherInfo::try_new(br_abs, br_rel) {
                Ok(breacher_info) => Some(breacher_info),
                Err(core_err) => {
                    return Err(match core_err {
                        rc::err::BreacherInfoError::InvalidValue(e) => Self::Error::InvalidBreacher(e),
                    });
                }
            },
            None => None,
        };
        match rc::DpsProfile::try_new(
            h_dps_profile.get_em(),
            h_dps_profile.get_thermal(),
            h_dps_profile.get_kinetic(),
            h_dps_profile.get_explosive(),
            breacher_info,
        ) {
            Ok(dps_profile) => Ok(dps_profile),
            Err(core_err) => Err(match core_err {
                rc::err::DpsProfileError::InvalidDmg(e) => Self::Error::InvalidDpsProfile(e),
            }),
        }
    }
}

#[derive(Copy, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HDpsProfileFull {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
    breacher: Option<(rc::AttrVal, rc::AttrVal)>,
}

#[derive(Copy, Clone, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HDpsProfileShort {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
