use crate::{svc::cycle::charged_cycles::ChargedCycleCount, util::InfCount};

pub(in crate::svc::cycle) fn get_undepletable_cycle_count() -> ChargedCycleCount {
    ChargedCycleCount {
        fully_charged: InfCount::Infinite,
        part_charged: None,
        can_run_uncharged: false,
    }
}
