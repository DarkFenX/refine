use crate::{svc::cycle::charged_cycles::ChargedCycleCount, util::InfCount};

pub(in crate::svc::cycle) fn get_uncharged_cycle_count() -> ChargedCycleCount {
    ChargedCycleCount {
        fully_charged: InfCount::Count(0),
        part_charged: None,
        can_run_uncharged: true,
    }
}
