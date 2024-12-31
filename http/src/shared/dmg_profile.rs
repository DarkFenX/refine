#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HDmgProfile {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<&rc::SolDmgProfile> for HDmgProfile {
    fn from(core_dmg_profile: &rc::SolDmgProfile) -> Self {
        Self {
            em: core_dmg_profile.em,
            thermal: core_dmg_profile.thermal,
            kinetic: core_dmg_profile.kinetic,
            explosive: core_dmg_profile.explosive,
        }
    }
}
impl Into<rc::SolDmgProfile> for &HDmgProfile {
    fn into(self) -> rc::SolDmgProfile {
        rc::SolDmgProfile {
            em: self.em,
            thermal: self.thermal,
            kinetic: self.kinetic,
            explosive: self.explosive,
        }
    }
}
