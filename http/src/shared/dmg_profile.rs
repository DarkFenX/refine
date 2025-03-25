#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HDmgProfile {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<&rc::DmgProfile> for HDmgProfile {
    fn from(core_dmg_profile: &rc::DmgProfile) -> Self {
        Self {
            em: core_dmg_profile.em,
            thermal: core_dmg_profile.thermal,
            kinetic: core_dmg_profile.kinetic,
            explosive: core_dmg_profile.explosive,
        }
    }
}
impl From<&HDmgProfile> for rc::DmgProfile {
    fn from(h_dmg_profile: &HDmgProfile) -> Self {
        Self {
            em: h_dmg_profile.em,
            thermal: h_dmg_profile.thermal,
            kinetic: h_dmg_profile.kinetic,
            explosive: h_dmg_profile.explosive,
        }
    }
}
