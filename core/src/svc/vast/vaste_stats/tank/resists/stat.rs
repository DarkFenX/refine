use crate::num::UnitInterval;

#[derive(Copy, Clone)]
pub struct StatResists {
    pub shield: StatResistsLayer,
    pub armor: StatResistsLayer,
    pub hull: StatResistsLayer,
}

#[derive(Copy, Clone)]
pub struct StatResistsLayer {
    pub em: UnitInterval,
    pub thermal: UnitInterval,
    pub kinetic: UnitInterval,
    pub explosive: UnitInterval,
}
