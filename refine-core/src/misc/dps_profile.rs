use crate::num::{PValue, UnitInterval};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    pub em: PValue = PValue::ZERO,
    pub thermal: PValue = PValue::ZERO,
    pub kinetic: PValue = PValue::ZERO,
    pub explosive: PValue = PValue::ZERO,
    pub breacher: Option<BreacherProfile> = None,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherProfile {
    pub absolute_max: PValue = PValue::ZERO,
    pub relative_max: UnitInterval = UnitInterval::ZERO,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DpsProfile {
    pub(crate) fn deals_breacher_dps(&self) -> bool {
        match self.breacher {
            Some(breacher) => breacher.relative_max > UnitInterval::ZERO && breacher.absolute_max > PValue::ZERO,
            None => false,
        }
    }
    pub(crate) fn get_sum_regular(&self) -> PValue {
        self.em + self.thermal + self.kinetic + self.explosive
    }
}
