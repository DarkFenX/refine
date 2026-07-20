use crate::num::UnitInterval;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatResists {
    pub shield: StatResistsLayer,
    pub armor: StatResistsLayer,
    pub hull: StatResistsLayer,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatResistsLayer {
    pub em: UnitInterval,
    pub thermal: UnitInterval,
    pub kinetic: UnitInterval,
    pub explosive: UnitInterval,
}
