use crate::defs::AttrVal;

pub struct SolDmgProfile {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
}
impl SolDmgProfile {
    pub fn new(em: AttrVal, thermal: AttrVal, kinetic: AttrVal, explosive: AttrVal) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
}
