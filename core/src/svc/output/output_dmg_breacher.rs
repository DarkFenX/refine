use crate::num::{Count, PValue, UnitInterval};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    // Absolute damage cap per tick/instance of damage
    pub(crate) absolute_max: PValue,
    // Relative damage cap per tick/instance of damage
    pub(crate) relative_max: UnitInterval,
    pub(crate) tick_count: Count,
}
impl OutputDmgBreacher {
    pub(crate) fn new(absolute_max: PValue, relative_max: UnitInterval, tick_count: Count) -> Option<Self> {
        if tick_count == Count::ZERO {
            return None;
        }
        Some(Self {
            absolute_max,
            relative_max,
            tick_count,
        })
    }
}
