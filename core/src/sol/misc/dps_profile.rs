use crate::sol::AttrVal;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
    pub breacher: Option<BreacherInfo>,
}
impl DpsProfile {
    pub fn new(
        em: AttrVal,
        thermal: AttrVal,
        kinetic: AttrVal,
        explosive: AttrVal,
        breacher: Option<BreacherInfo>,
    ) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherInfo {
    pub absolute_max: AttrVal,
    pub percent_max: AttrVal,
}
impl BreacherInfo {
    pub fn new(absolute_max: AttrVal, percent_max: AttrVal) -> Self {
        Self {
            absolute_max,
            percent_max,
        }
    }
}
