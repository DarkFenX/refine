use crate::def::{AttrVal, Count};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    pub(crate) absolute_max: AttrVal,
    pub(crate) relative_max: AttrVal,
    pub(crate) tick_count: Count,
}
impl OutputDmgBreacher {
    pub(crate) fn new(absolute_max: AttrVal, relative_max: AttrVal, tick_count: Count) -> Option<Self> {
        if tick_count == 0 {
            return None;
        }
        Some(Self {
            absolute_max,
            relative_max,
            tick_count,
        })
    }
}
