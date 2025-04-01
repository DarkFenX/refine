#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HDpsProfile {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<&rc::DpsProfile> for HDpsProfile {
    fn from(core_dps_profile: &rc::DpsProfile) -> Self {
        Self {
            em: core_dps_profile.em,
            thermal: core_dps_profile.thermal,
            kinetic: core_dps_profile.kinetic,
            explosive: core_dps_profile.explosive,
        }
    }
}
impl From<&HDpsProfile> for rc::DpsProfile {
    fn from(h_dps_profile: &HDpsProfile) -> Self {
        Self {
            em: h_dps_profile.em,
            thermal: h_dps_profile.thermal,
            kinetic: h_dps_profile.kinetic,
            explosive: h_dps_profile.explosive,
            breacher: None,
        }
    }
}
