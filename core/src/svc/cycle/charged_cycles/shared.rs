use crate::{def::AttrVal, util::InfCount};

pub(in crate::svc::cycle) struct ChargedCycleCount {
    pub(in crate::svc::cycle) fully_charged: InfCount,
    pub(in crate::svc::cycle) part_charged: Option<AttrVal>,
    pub(in crate::svc::cycle) can_run_uncharged: bool,
}
