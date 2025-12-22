use crate::{def::AttrVal, util::InfCount};

pub(crate) struct CycleChargedInfo {
    pub(crate) fully_charged: InfCount,
    pub(crate) part_charged: Option<AttrVal>,
}
