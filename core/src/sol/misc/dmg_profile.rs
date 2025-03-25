use crate::sol::AttrVal;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmgProfile {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
}
impl DmgProfile {
    pub fn new(em: AttrVal, thermal: AttrVal, kinetic: AttrVal, explosive: AttrVal) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
}
