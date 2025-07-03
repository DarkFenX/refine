use crate::def::{AttrVal, OF};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UadProjRange {
    // Center-to-center range
    pub(crate) c2c: AttrVal,
    // Surface-to-surface range
    pub(crate) s2s: AttrVal,
    pub(crate) src_radius: AttrVal,
    pub(crate) tgt_radius: AttrVal,
}
impl UadProjRange {
    pub(crate) fn new_tmp(range: AttrVal) -> Self {
        Self {
            c2c: range,
            s2s: range,
            src_radius: OF(0.0),
            tgt_radius: OF(0.0),
        }
    }
}
