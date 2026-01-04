use crate::def::{AttrVal, DefCount};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    pub(crate) absolute_max: AttrVal,
    pub(crate) relative_max: AttrVal,
    pub(crate) tick_count: DefCount,
}
impl OutputDmgBreacher {
    pub(crate) fn new(absolute_max: AttrVal, relative_max: AttrVal, tick_count: DefCount) -> Option<Self> {
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
