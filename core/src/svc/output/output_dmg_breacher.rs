use crate::misc::{Count, PValue};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    pub(crate) absolute_max: PValue,
    pub(crate) relative_max: PValue,
    pub(crate) tick_count: Count,
}
impl OutputDmgBreacher {
    pub(crate) fn new(absolute_max: PValue, relative_max: PValue, tick_count: Count) -> Option<Self> {
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
