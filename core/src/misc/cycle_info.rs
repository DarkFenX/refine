use crate::{
    def::{AttrVal, Count},
    util::InfCount,
};

pub(crate) enum CycleInfo {
    Simple(CycleSimple),
    Complex(CycleComplex),
}

pub(crate) struct CycleInner {
    pub(crate) active_time: AttrVal,
    pub(crate) inactive_time: AttrVal,
    pub(crate) repeat_count: Count,
}

pub(crate) struct CycleSimple {
    pub(crate) active_time: AttrVal,
    pub(crate) inactive_time: AttrVal,
    pub(crate) repeat_count: InfCount,
}

pub(crate) struct CycleComplex {
    pub(crate) inner1: CycleInner,
    pub(crate) inner2: CycleInner,
    pub(crate) repeat_count: InfCount,
}
