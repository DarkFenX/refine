use crate::def::{AttrVal, Count};

#[derive(Copy, Clone)]
pub(crate) struct OutputDmgBreacher {
    pub(crate) absolute_max: AttrVal,
    pub(crate) relative_max: AttrVal,
    pub(crate) instance_count: Count,
}
